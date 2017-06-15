// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module contains functionality related to remote desktop using VNC.

use std;
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::sync::{Arc, Mutex, RwLock};

use vnc;

use dharma;
use qualia::{perceptron, Perceptron};
use qualia::{Area, CatchResult, ClientChange, Illusion, InputCode, InputForwarding, InputHandling};
use qualia::{InputValue, OutputType, Position, Size, VirtualFramebuffer, VirtualOutputBundle};
use inputs::{codes, ModState};

const AREA1: Area = Area { pos: Position { x: 0, y: 0 }, size: Size { width: 800, height: 600 } };
const AREA2: Area = Area { pos: Position { x: 800, y: 0 }, size: Size { width: 800, height: 500 } };

// -------------------------------------------------------------------------------------------------

/// This structure contains logic responsible for interpreting keyboards and pointer events from VNC
/// client and forwarding them to the rest of the application.
struct VncInputGateway {
    handler: Box<InputHandling>,
    forwarder: Box<InputForwarding>,
    modifiers: ModState,
    button_mask: u8,
}

// -------------------------------------------------------------------------------------------------

impl VncInputGateway {
    /// Constructs new `VncInputGateway`.
    fn new(handler: Box<InputHandling>, forwarder: Box<InputForwarding>) -> Self {
        VncInputGateway {
            handler: handler,
            forwarder: forwarder,
            modifiers: ModState::new(),
            button_mask: 0x0,
        }
    }

    /// Handles keyboard events from VNC client.
    fn handle_key(&mut self, code: InputCode, value: InputValue) {
        // Update modifiers
        if self.modifiers.update(code, value) != CatchResult::Passed {
            return;
        }

        // Try to execute key binding
        if self.handler.catch_key(code, value, self.modifiers.get()) == CatchResult::Passed {
            self.forwarder.emit_key(code, value);
        }
    }

    /// Handles pointer events from VNC client.
    fn handle_pointer(&mut self, button_mask: u8, x: u16, y: u16) {
        // Inform the application about new position.
        self.forwarder.emit_position_reset(Some(Position::new(x as isize, y as isize)));

        // Check if button mask changed and if so inform the application.
        for (mask, code) in vec![(0x1, codes::BTN_LEFT), (0x4, codes::BTN_RIGHT)] {
            if ((self.button_mask & mask) == 0x0) && ((button_mask & mask) == mask) {
                self.forwarder.emit_button(code, 1);
            } else if ((self.button_mask & mask) == mask) && ((button_mask & mask) == 0x0) {
                self.forwarder.emit_button(code, 0);
            }
        }
        self.button_mask = button_mask;
    }
}

// -------------------------------------------------------------------------------------------------

/// Structure containing main logic responsible for mediating between VNC client and the rest of
/// the application.
pub struct Vnc {
    /// Manages input events from VNC client.
    gateway: Arc<Mutex<VncInputGateway>>,

    /// Virtual framebuffer shared with outputs.
    ///
    /// This field contains buffer the outputs use for rendering. Its mutex-ed so no need for
    /// double buffering.
    ///
    /// All outputs will be visible in clients single window. For efficiency all outputs use the
    /// same framebuffer so there is not need to copy or join the data. Instead it is simply
    /// written to VNC socket as a whole.
    vfb: Arc<RwLock<VirtualFramebuffer>>,

    /// Area covering all outputs.
    area: Area,

    /// `dharma` dispatcher for managing event sources.
    dispatcher: dharma::DispatcherController,

    /// `dharma` signaler for communication with the application.
    signaler: dharma::Signaler<Perceptron>,
}

// -------------------------------------------------------------------------------------------------

impl Vnc {
    /// Constructs new `Vnc`.
    ///
    /// TODO: In off-screen mode always two outputs with hard-coded sizes and position are created.
    /// Their number and parameters should be configurable.
    pub fn new(input_handler: Box<InputHandling>,
               input_forwarder: Box<InputForwarding>,
               mut dispatcher: dharma::DispatcherController,
               signaler: dharma::Signaler<Perceptron>)
               -> Result<Self, std::io::Error> {
        // Calculate the area containing all outputs.
        let areas = vec![AREA1, AREA2];
        let mut area = Area::default();
        for a in areas.iter() {
            area.inflate(a);
        }

        // Create framebuffer and the outputs.
        let stride = 4 * area.size.width;
        let fb = Arc::new(RwLock::new(VirtualFramebuffer::new(vec![0; stride * area.size.height])));
        let b1 = VirtualOutputBundle::new(fb.clone(), 0, stride, AREA1);
        let b2 = VirtualOutputBundle::new(fb.clone(), 4 * AREA1.size.width, stride, AREA2);
        signaler.emit(perceptron::OUTPUT_FOUND, Perceptron::OutputFound(OutputType::Virtual(b1)));
        signaler.emit(perceptron::OUTPUT_FOUND, Perceptron::OutputFound(OutputType::Virtual(b2)));

        // Set VNC server up.
        let listener = TcpListener::bind("127.0.0.1:5900")?;
        let connection_handler = VncConnectionHandler::new(listener, signaler.clone())?;
        dispatcher.add_source(Box::new(connection_handler), dharma::event_kind::READ);

        // Construct the gateway.
        let gateway = Arc::new(Mutex::new(VncInputGateway::new(input_handler, input_forwarder)));

        // Construct `Vnc` structure.
        Ok(Vnc {
            gateway: gateway,
            vfb: fb,
            area: area,
            dispatcher: dispatcher,
            signaler: signaler,
        })
    }

    /// Handles request for connection from clients.
    pub fn handle_client_connection(&mut self, fd: RawFd) {
        match VncRequestHandler::new(fd,
                                     self.gateway.clone(),
                                     self.vfb.clone(),
                                     self.area,
                                     self.signaler.clone()) {
            Ok(connection) => {
                self.dispatcher.add_source(Box::new(connection),
                                           dharma::event_kind::READ);
            }
            Err(err) => log_warn1!("Couldn't create VNC request handler: {:?}", err),
        }
    }

    /// Handles client disconnections.
    pub fn handle_client_disconnection(&mut self, _id: u64) {
        // Nothing to do - the event source was remove automatically.
    }
}

// -------------------------------------------------------------------------------------------------

/// Handler of client connection events.
struct VncConnectionHandler {
    listener: TcpListener,
    signaler: dharma::Signaler<Perceptron>,
}

// -------------------------------------------------------------------------------------------------

impl VncConnectionHandler {
    /// Constructs new `VncConnectionHandler`.
    pub fn new(listener: TcpListener,
               signaler: dharma::Signaler<Perceptron>)
               -> Result<Self, std::io::Error>  {
        Ok(VncConnectionHandler {
            listener: listener,
            signaler: signaler,
        })
    }
}

// -------------------------------------------------------------------------------------------------

impl dharma::EventHandler for VncConnectionHandler {
    fn get_fd(&self) -> RawFd {
        self.listener.as_raw_fd()
    }

    fn process_event(&mut self, event_kind: dharma::EventKind) -> Result<(), ()> {
        if event_kind.intersects(dharma::event_kind::READ) {
            match self.listener.accept() {
                Ok((stream, addr)) => {
                    log_info1!("Accepted VNC connection from {:?}", addr);
                    let fd = stream.into_raw_fd();
                    self.signaler.emit(perceptron::REMOTE_CLIENT_CHANGE,
                                       Perceptron::RemoteClientChange(ClientChange::Connected{fd}));
                }
                Err(err) => log_warn1!("Couldn't get VNC client: {:?}", err),
            }
        } else if event_kind.intersects(dharma::event_kind::HANGUP) {
            log_error!("Lost connection to VNC socket!");
        }
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

/// Handler of VNC client requests.
struct VncRequestHandler {
    /// The TCP connection
    fd: RawFd,

    /// Manages input events from VNC client
    gateway: Arc<Mutex<VncInputGateway>>,

    /// Virtual framebuffer shared with outputs
    vfb: Arc<RwLock<VirtualFramebuffer>>,

    /// ID of the handler
    id: dharma::EventHandlerId,

    /// Area covering all outputs.
    area: Area,

    /// Instance of the VNC server
    server: Arc<RwLock<vnc::Server>>,

    /// `dharma` signaler for communication with the application
    signaler: dharma::Signaler<Perceptron>,
}

// -------------------------------------------------------------------------------------------------

impl VncRequestHandler {
    /// Constructs new `VncRequestHandler`.
    ///
    /// File descriptor `fd` must be created from `TcpStream`.
    ///
    /// `width` and `height` are dimensions of the area covering all outputs.
    pub fn new(fd: RawFd,
               gateway: Arc<Mutex<VncInputGateway>>,
               vfb: Arc<RwLock<VirtualFramebuffer>>,
               area: Area,
               signaler: dharma::Signaler<Perceptron>)
               -> Result<Self, Illusion>  {
        let stream = unsafe { TcpStream::from_raw_fd(fd) };
        let format = vnc::PixelFormat {
            bits_per_pixel: 32,
            depth:          24,
            big_endian:     true,
            true_colour:    true,
            red_max:        255,
            green_max:      255,
            blue_max:       255,
            red_shift:      16,
            green_shift:    8,
            blue_shift:     0,
        };

        match vnc::Server::from_tcp_stream(stream,
                                           area.size.width as u16,
                                           area.size.height as u16,
                                           format,
                                           "perceptia".to_owned()) {
            Ok((server, _shared)) => {
                Ok(VncRequestHandler {
                    fd: fd,
                    gateway: gateway,
                    vfb: vfb,
                    id: 0,
                    area: area,
                    server: Arc::new(RwLock::new(server)),
                    signaler: signaler,
                })
            }
            Err(err) => {
                Err(Illusion::General(format!("Failed to setup connection with client: {:?}", err)))
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl dharma::EventHandler for VncRequestHandler {
    fn get_fd(&self) -> RawFd {
        self.fd
    }

    fn process_event(&mut self, event_kind: dharma::EventKind) -> Result<(), ()> {
        if event_kind.intersects(dharma::event_kind::READ) {
            let mut server = self.server.write().unwrap();
            match server.read_event() {
                Ok(ref packet) => {
                    match *packet {
                        vnc::server::Event::SetPixelFormat(pixel_format) => {
                            // TODO: Implement pixel format requests from VNC clients.
                            log_nyimp!("VNC client requested set pixel format: {:?}", pixel_format);
                        }
                        vnc::server::Event::SetEncodings(ref encodings) => {
                            let mut supports_extended_key_event = false;

                            // Encodings are used in VNC protocol by clients to signalize supported
                            // extensions. Here we check if client supports QEMU `ExtendedKeyEvent`
                            // extension and respond to client to confirm we support it.
                            //
                            // This extension is used to obtain raw key codes for keyboard events
                            // to avoid problems with mismatching key maps on our and clients side.
                            //
                            // TODO: If VNC client does not support `ExtendedKeyEvent` the regular
                            // `KeyEvent` message should be used and X11 keysym should be
                            // translated to evdev one used in whole application.
                            for encoding in encodings {
                                match *encoding {
                                    vnc::Encoding::ExtendedKeyEvent => {
                                        server.send_framebuffer_update_header(1)
                                              .expect("sending extension confirmation");
                                        server.send_rectangle_header(0, 0, 0, 0, *encoding)
                                              .expect("sending extension confirmation");
                                        supports_extended_key_event = true;
                                    }
                                    _ => {}
                                }
                            }

                            if !supports_extended_key_event {
                                log_warn1!("VNC client does not support ExtendedKeyEvent");
                            }
                        }
                        vnc::server::Event::FramebufferUpdateRequest {
                            incremental: _,
                            x_position,
                            y_position,
                            width,
                            height,
                        } => {
                            let mut vfb = self.vfb.write().unwrap();

                            if x_position == 0 &&
                               y_position == 0 &&
                               width as usize == self.area.size.width &&
                               height as usize == self.area.size.height {
                                // Send one rectangle containing raw data of whole framebuffer.
                                server.send_framebuffer_update_header(1)
                                      .expect("sending framebuffer update header");
                                server.send_rectangle_header(x_position,
                                                             y_position,
                                                             width,
                                                             height,
                                                             vnc::Encoding::Raw)
                                    .expect("sending rectangle header");
                                server.send_raw_data(vfb.as_slice()).expect("sending pixel data");

                                // Notify subscribers that the screen was redrawn.
                                for display_id in vfb.take_subscribers() {
                                    self.signaler.emit(perceptron::PAGE_FLIP,
                                                       Perceptron::PageFlip(display_id));
                                }
                            } else {
                                // TODO: Add support for VNC `FramebufferUpdateRequest` requests
                                // the do not cover whole framebuffer.
                            }
                        }
                        vnc::server::Event::KeyEvent { down, key } => {
                            log_nyimp!("Client sent KeyEvent: {:?} {:?}", down, key);
                        }
                        vnc::server::Event::PointerEvent { button_mask, x_position, y_position } => {
                            self.gateway.lock().unwrap().handle_pointer(button_mask,
                                                                        x_position,
                                                                        y_position);
                        }
                        vnc::server::Event::CutText(ref clipboard) => {
                            log_nyimp!("VNC client requested cut text: '{:?}'", clipboard);
                        }
                        vnc::server::Event::ExtendedKeyEvent { down, keysym: _, keycode } => {
                            self.gateway.lock().unwrap().handle_key(keycode as InputCode,
                                                                    down as InputValue);
                        }
                    }
                    Ok(())
                }
                Err(ref err) => {
                    /// Inform the rest of the application we disconnect from remote client and
                    /// return error to make sure we stop handling events on the socket.
                    log_error!("VNC error: {:?}", err);
                    let change = ClientChange::Disconnected{id: self.id};
                    self.signaler.emit(perceptron::REMOTE_CLIENT_CHANGE,
                                       Perceptron::RemoteClientChange(change));
                    Err(())
                }
            }
        } else if event_kind.intersects(dharma::event_kind::HANGUP) {
            log_info1!("Lost connection to VNC client");
            Err(())
        } else {
            Ok(())
        }
    }

    fn set_id(&mut self, id: dharma::EventHandlerId) {
        self.id = id;
    }
}

// -------------------------------------------------------------------------------------------------
