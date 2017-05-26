// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This module defines frame layouts used in tests.

// TODO: rust-fmt fails on diagrams.
#![cfg_attr(rustfmt, rustfmt_skip)]

// -------------------------------------------------------------------------------------------------

use frames::Frame;
use frames::Geometry::{Horizontal, Stacked, Vertical};
use frames::Mobility::Floating;
use qualia::{Area, Position, Size, SurfaceId};

// -------------------------------------------------------------------------------------------------

/// Prepares simple frame layout containing `Vertical`, `Horizontal` and `Stacked` containers as
/// drawn below. Structure is build by appending all frames.
///
///   ┌──────────────────────────────────────────────┐
///   │ ┌──────┐                                     │
///   │ │  v1  │                                     │
///   │ ├──────┤ ┌──────┬──────┬──────┐ ┌──────────┐ │
///   │ │  v2  │ │  h1  │  h2  │  h3  │ │ s1,s2,s3 │ │
///   │ ├──────┤,└──────┴──────┴──────┘,└──────────┘ │
///   │ │  v3  │                                     │
///   │ └──────┘                                     │
///   └──────────────────────────────────────────────┘
///
pub fn make_simple_frames_appending()
    -> (Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame) {
    let mut r = Frame::new_workspace(String::new(), Vertical);
    let mut v = Frame::new_container(Vertical);
    let mut h = Frame::new_container(Horizontal);
    let mut s = Frame::new_container(Stacked);
    let mut v1 = Frame::new_leaf(SurfaceId::new(11), Stacked);
    let mut v2 = Frame::new_leaf(SurfaceId::new(12), Stacked);
    let mut v3 = Frame::new_leaf(SurfaceId::new(13), Stacked);
    let mut h1 = Frame::new_leaf(SurfaceId::new(21), Stacked);
    let mut h2 = Frame::new_leaf(SurfaceId::new(22), Stacked);
    let mut h3 = Frame::new_leaf(SurfaceId::new(23), Stacked);
    let mut s1 = Frame::new_leaf(SurfaceId::new(31), Stacked);
    let mut s2 = Frame::new_leaf(SurfaceId::new(32), Stacked);
    let mut s3 = Frame::new_leaf(SurfaceId::new(33), Stacked);
    r.append(&mut v);
    r.append(&mut h);
    r.append(&mut s);
    v.append(&mut v1);
    v.append(&mut v2);
    v.append(&mut v3);
    h.append(&mut h1);
    h.append(&mut h2);
    h.append(&mut h3);
    s.append(&mut s1);
    s.append(&mut s2);
    s.append(&mut s3);
    r. set_plumbing_position_and_size(Position::new( 0,  0), Size::new(30, 30));
    v. set_plumbing_position_and_size(Position::new( 0,  0), Size::new(30, 30));
    h. set_plumbing_position_and_size(Position::new( 0,  0), Size::new(30, 30));
    s. set_plumbing_position_and_size(Position::new( 0,  0), Size::new(30, 30));
    v1.set_plumbing_position_and_size(Position::new( 0,  0), Size::new(30, 10));
    v2.set_plumbing_position_and_size(Position::new( 0, 10), Size::new(30, 10));
    v3.set_plumbing_position_and_size(Position::new( 0, 20), Size::new(30, 10));
    h1.set_plumbing_position_and_size(Position::new( 0,  0), Size::new(10, 30));
    h2.set_plumbing_position_and_size(Position::new(10,  0), Size::new(10, 30));
    h3.set_plumbing_position_and_size(Position::new(20,  0), Size::new(10, 30));
    s1.set_plumbing_position_and_size(Position::new( 0,  0), Size::new(30, 30));
    s2.set_plumbing_position_and_size(Position::new( 0,  0), Size::new(30, 30));
    s3.set_plumbing_position_and_size(Position::new( 0,  0), Size::new(30, 30));
    (r, v, h, s, v1, v2, v3, h1, h2, h3, s1, s2, s3)
}

// -------------------------------------------------------------------------------------------------

/// Prepares simple frame layout containing `Vertical`, `Horizontal` and `Stacked` containers as
/// drawn below. Structure is build by prepending all frames so timed order is reverse of spaced
/// order.
///
///   ┌──────────────────────────────────────────────┐
///   │ ┌──────┐                                     │
///   │ │  v1  │                                     │
///   │ ├──────┤ ┌──────┬──────┬──────┐ ┌──────────┐ │
///   │ │  v2  │ │  h1  │  h2  │  h3  │ │ s1,s2,s3 │ │
///   │ ├──────┤,└──────┴──────┴──────┘,└──────────┘ │
///   │ │  v3  │                                     │
///   │ └──────┘                                     │
///   └──────────────────────────────────────────────┘
///
pub fn make_simple_frames_prepending()
    -> (Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame) {
    let mut r = Frame::new_workspace(String::new(), Vertical);
    let mut v = Frame::new_container(Vertical);
    let mut h = Frame::new_container(Horizontal);
    let mut s = Frame::new_container(Stacked);
    let mut v1 = Frame::new_leaf(SurfaceId::new(11), Stacked);
    let mut v2 = Frame::new_leaf(SurfaceId::new(12), Stacked);
    let mut v3 = Frame::new_leaf(SurfaceId::new(13), Stacked);
    let mut h1 = Frame::new_leaf(SurfaceId::new(21), Stacked);
    let mut h2 = Frame::new_leaf(SurfaceId::new(22), Stacked);
    let mut h3 = Frame::new_leaf(SurfaceId::new(23), Stacked);
    let mut s1 = Frame::new_leaf(SurfaceId::new(31), Stacked);
    let mut s2 = Frame::new_leaf(SurfaceId::new(32), Stacked);
    let mut s3 = Frame::new_leaf(SurfaceId::new(33), Stacked);
    r.prepend(&mut s);
    r.prepend(&mut h);
    r.prepend(&mut v);
    v.prepend(&mut v3);
    v.prepend(&mut v2);
    v.prepend(&mut v1);
    h.prepend(&mut h3);
    h.prepend(&mut h2);
    h.prepend(&mut h1);
    s.prepend(&mut s3);
    s.prepend(&mut s2);
    s.prepend(&mut s1);
    (r, v, h, s, v1, v2, v3, h1, h2, h3, s1, s2, s3)
}

// -------------------------------------------------------------------------------------------------

/// Prepares simple frame layout containing `Vertical`, `Horizontal` and `Stacked` containers as
/// drawn below. Structure is build by joining frames, so timed order is different than space
/// order.
///
///   ┌──────────────────────────────────────────────┐
///   │ ┌──────┐                                     │
///   │ │  v1  │                                     │
///   │ ├──────┤ ┌──────┬──────┬──────┐ ┌──────────┐ │
///   │ │  v2  │ │  h1  │  h2  │  h3  │ │ s1,s2,s3 │ │
///   │ ├──────┤,└──────┴──────┴──────┘,└──────────┘ │
///   │ │  v3  │                                     │
///   │ └──────┘                                     │
///   └──────────────────────────────────────────────┘
///
pub fn make_simple_frames_joining()
    -> (Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame) {
    let mut r = Frame::new_workspace(String::new(), Vertical);
    let mut v = Frame::new_container(Vertical);
    let mut h = Frame::new_container(Horizontal);
    let mut s = Frame::new_container(Stacked);
    let mut v1 = Frame::new_leaf(SurfaceId::new(11), Stacked);
    let mut v2 = Frame::new_leaf(SurfaceId::new(12), Stacked);
    let mut v3 = Frame::new_leaf(SurfaceId::new(13), Stacked);
    let mut h1 = Frame::new_leaf(SurfaceId::new(21), Stacked);
    let mut h2 = Frame::new_leaf(SurfaceId::new(22), Stacked);
    let mut h3 = Frame::new_leaf(SurfaceId::new(23), Stacked);
    let mut s1 = Frame::new_leaf(SurfaceId::new(31), Stacked);
    let mut s2 = Frame::new_leaf(SurfaceId::new(32), Stacked);
    let mut s3 = Frame::new_leaf(SurfaceId::new(33), Stacked);
    r.append(&mut v);
    r.append(&mut h);
    r.append(&mut s);

    // For testing joining in middle.
    v.append(&mut v1);
    v.append(&mut v3);
    v3.prejoin(&mut v2);

    // For testing joining at the begin.
    h.append(&mut h1);
    h.append(&mut h3);
    h1.adjoin(&mut h2);

    // For testing joining at the end.
    s.append(&mut s2);
    s2.prejoin(&mut s1);
    s2.adjoin(&mut s3);
    (r, v, h, s, v1, v2, v3, h1, h2, h3, s1, s2, s3)
}

// -------------------------------------------------------------------------------------------------

/// Prepares frame layout used for testing deramification.
///
///                    R
///
///     ╭──────────────┼──────────────╮
///
///     A1             A2             A3
///
///     │              │
///
///     F              B
///
///                    │
///
///                    C
///
///             ╭──────┼──────╮
///
///             D1     D2     D3
///
pub fn make_simple_for_deramifying()
-> (Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame) {
    let mut r  = Frame::new_workspace(String::new(), Vertical);
    let mut a1 = Frame::new_container(Stacked);
    let mut a2 = Frame::new_container(Stacked);
    let mut a3 = Frame::new_leaf(SurfaceId::new(1), Stacked);
    let mut f  = Frame::new_container(Stacked);
    let mut b  = Frame::new_container(Stacked);
    let mut c  = Frame::new_container(Stacked);
    let mut d1 = Frame::new_leaf(SurfaceId::new(11), Stacked);
    let mut d2 = Frame::new_leaf(SurfaceId::new(12), Stacked);
    let mut d3 = Frame::new_leaf(SurfaceId::new(13), Stacked);
    r. append(&mut a1);
    r. append(&mut a2);
    r. append(&mut a3);
    a1.append(&mut f);
    a2.append(&mut b);
    b. append(&mut c);
    c. append(&mut d1);
    c. append(&mut d2);
    c. append(&mut d3);
    (r, a1, a2, a3, f, b, c, d1, d2, d3)
}

// -------------------------------------------------------------------------------------------------

/// Prepares layout for testing homogenizing. Frame have appropriate size to check if they are not
/// homogenized when not resized along.
///
///
///     ┌───────────────┬─────┬─────────────┐
///     │┌─────────────┐│     │┌─────┬─────┐│
///     ││      A      ││     ││     │     ││
///     │├─────────────┤│     ││     │     ││
///     ││     BCD     ││     ││     │     ││
///     │├─────────────┤│  G  ││  H  │  I  ││
///     ││┌─────┬─────┐││     ││     │     ││
///     │││  E  │  F  │││     ││     │     ││
///     ││└─────┴─────┘││     ││     │     ││
///     │└─────────────┘│     │└─────┴─────┘│
///     └───────────────┴─────┴─────────────┘
///
pub fn make_sized_for_homogenizing()
    -> (Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,
        Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame) {
    let mut r = Frame::new_workspace(String::new(), Stacked);
    let mut a = Frame::new_leaf(SurfaceId::new(1), Stacked);
    let mut b = Frame::new_leaf(SurfaceId::new(2), Stacked);
    let mut c = Frame::new_leaf(SurfaceId::new(3), Stacked);
    let mut d = Frame::new_leaf(SurfaceId::new(4), Stacked);
    let mut e = Frame::new_leaf(SurfaceId::new(5), Stacked);
    let mut f = Frame::new_leaf(SurfaceId::new(6), Stacked);
    let mut g = Frame::new_leaf(SurfaceId::new(7), Stacked);
    let mut h = Frame::new_leaf(SurfaceId::new(8), Stacked);
    let mut i = Frame::new_leaf(SurfaceId::new(8), Stacked);
    let mut z = Frame::new_leaf(SurfaceId::new(9), Stacked);
    let mut bcd = Frame::new_container(Stacked);
    let mut ef = Frame::new_container(Horizontal);
    let mut abcdef = Frame::new_container(Vertical);
    let mut hi = Frame::new_container(Horizontal);
    let mut abcdefghi = Frame::new_container(Horizontal);
    bcd.append(&mut b);
    bcd.append(&mut c);
    bcd.append(&mut d);
    ef.append(&mut e);
    ef.append(&mut f);
    abcdef.append(&mut a);
    abcdef.append(&mut bcd);
    abcdef.append(&mut ef);
    hi.append(&mut h);
    hi.append(&mut i);
    abcdefghi.append(&mut abcdef);
    abcdefghi.append(&mut g);
    abcdefghi.append(&mut hi);
    r.append(&mut abcdefghi);
    r.append(&mut z);
    r.        set_plumbing_position_and_size(Position::new(  0,   0), Size::new(360, 360));
    // Make main child slightly bigger to enforce homogenization
    abcdefghi.set_plumbing_position_and_size(Position::new(  0,   0), Size::new(361, 361));
    hi.       set_plumbing_position_and_size(Position::new(240,   0), Size::new(120, 360));
    abcdef.   set_plumbing_position_and_size(Position::new(  0,   0), Size::new(180, 360));
    ef.       set_plumbing_position_and_size(Position::new(  0, 300), Size::new(180,  60));
    bcd.      set_plumbing_position_and_size(Position::new(  0, 120), Size::new(180, 180));
    i.        set_plumbing_position_and_size(Position::new( 80,   0), Size::new( 40, 360));
    h.        set_plumbing_position_and_size(Position::new(  0,   0), Size::new( 80, 360));
    g.        set_plumbing_position_and_size(Position::new(180,   0), Size::new( 60, 360));
    f.        set_plumbing_position_and_size(Position::new( 60,   0), Size::new(120,  60));
    e.        set_plumbing_position_and_size(Position::new(  0,   0), Size::new( 60,  60));
    d.        set_plumbing_position_and_size(Position::new(  0,   0), Size::new(180, 180));
    c.        set_plumbing_position_and_size(Position::new(  0,   0), Size::new(180, 180));
    b.        set_plumbing_position_and_size(Position::new(  0,   0), Size::new(180, 180));
    a.        set_plumbing_position_and_size(Position::new(  0,   0), Size::new(180, 120));
    z.        set_plumbing_position_and_size(Position::new( 13,  23), Size::new( 33,  43));
    z.        set_plumbing_mobility(Floating);
    (r, abcdefghi, hi, abcdef, ef, bcd, a, b, c, d, e, f, g, h, i, z)
}

// -------------------------------------------------------------------------------------------------

/// Prepares layout for testing search.
///
///
///     ┌───────────────────────┐
///     │┌───────┬─────────────┐│
///     ││┌─────┐│┌─────┬─────┐││
///     │││ ABC │││  D  │  E  │││
///     ││└─────┘│└─────┴─────┘││
///     │└───────┴─────────────┘│
///     ├───────────────────────┤
///     │┌────────────────┐     │
///     ││        F       │     │
///     │└────────────────┘     │
///     └───────────────────────┘
///
pub fn make_positioned_for_searching()
    -> (Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame) {
    let mut r = Frame::new_workspace(String::new(), Vertical);
    let mut a = Frame::new_leaf(SurfaceId::new(1), Stacked);
    let mut b = Frame::new_leaf(SurfaceId::new(2), Stacked);
    let mut c = Frame::new_leaf(SurfaceId::new(3), Stacked);
    let mut d = Frame::new_leaf(SurfaceId::new(4), Stacked);
    let mut e = Frame::new_leaf(SurfaceId::new(5), Stacked);
    let mut f = Frame::new_leaf(SurfaceId::new(6), Stacked);
    let mut abc = Frame::new_container(Stacked);
    let mut de = Frame::new_container(Horizontal);
    let mut abcde = Frame::new_container(Vertical);
    abc.append(&mut a);
    abc.append(&mut b);
    abc.append(&mut c);
    de.append(&mut d);
    de.append(&mut e);
    abcde.append(&mut abc);
    abcde.append(&mut de);
    r.append(&mut abcde);
    r.append(&mut f);
    a.    set_plumbing_position_and_size(Position::new( 0,  0), Size::new( 40,  60));
    b.    set_plumbing_position_and_size(Position::new( 0,  0), Size::new( 40,  60));
    c.    set_plumbing_position_and_size(Position::new( 0,  0), Size::new( 40,  60));
    d.    set_plumbing_position_and_size(Position::new( 0,  0), Size::new( 30,  60));
    e.    set_plumbing_position_and_size(Position::new(30,  0), Size::new( 30,  60));
    f.    set_plumbing_position_and_size(Position::new( 0,  0), Size::new( 70,  60));
    r.    set_plumbing_position_and_size(Position::new( 0,  0), Size::new(100, 120));
    abc.  set_plumbing_position_and_size(Position::new( 0,  0), Size::new( 40,  60));
    de.   set_plumbing_position_and_size(Position::new(40,  0), Size::new( 60,  60));
    abcde.set_plumbing_position_and_size(Position::new( 0,  0), Size::new(100,  60));
    (r, abcde, de, abc, a, b, c, d, e, f)
}

// -------------------------------------------------------------------------------------------------

/// Prepares layout for testing jumping.
///
///
///     ┌───────────────────────┐
///     │┌───────┬─────────────┐│
///     ││       │┌─────┬─────┐││
///     ││   A   ││ BCD │  E  │││
///     ││       │└─────┴─────┘││
///     │└───────┴─────────────┘│
///     ├───────────────────────┤
///     │┌─────────────────────┐│
///     ││          F          ││
///     │├─────────────────────┤│
///     ││         GHI         ││
///     │└─────────────────────┘│
///     └───────────────────────┘
///
pub fn make_positioned_for_jumping()
-> (Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame,Frame) {
    let mut r     = Frame::new_workspace(String::new(), Vertical);
    let mut w     = Frame::new_container(Vertical);
    let mut a     = Frame::new_leaf(SurfaceId::new(1), Stacked);
    let mut b     = Frame::new_leaf(SurfaceId::new(2), Stacked);
    let mut c     = Frame::new_leaf(SurfaceId::new(3), Stacked);
    let mut d     = Frame::new_leaf(SurfaceId::new(4), Stacked);
    let mut e     = Frame::new_leaf(SurfaceId::new(5), Stacked);
    let mut f     = Frame::new_leaf(SurfaceId::new(6), Stacked);
    let mut g     = Frame::new_leaf(SurfaceId::new(7), Stacked);
    let mut h     = Frame::new_leaf(SurfaceId::new(8), Stacked);
    let mut i     = Frame::new_leaf(SurfaceId::new(9), Stacked);
    let mut bcd   = Frame::new_container(Stacked);
    let mut bcde  = Frame::new_container(Horizontal);
    let mut abcde = Frame::new_container(Horizontal);
    let mut ghi   = Frame::new_container(Stacked);
    let mut fghi  = Frame::new_container(Vertical);
    bcd.  append(&mut b);
    bcd.  append(&mut c);
    bcd.  append(&mut d);
    bcde. append(&mut bcd);
    bcde. append(&mut e);
    abcde.append(&mut a);
    abcde.append(&mut bcde);
    ghi.  append(&mut g);
    ghi.  append(&mut h);
    ghi.  append(&mut i);
    fghi. append(&mut f);
    fghi. append(&mut ghi);
    w.    append(&mut abcde);
    w.    append(&mut fghi);
    r.    append(&mut w);
    a.    set_plumbing_position_and_size(Position::new( 0,  0), Size::new( 40,  40));
    b.    set_plumbing_position_and_size(Position::new( 0,  0), Size::new( 40,  40));
    c.    set_plumbing_position_and_size(Position::new( 0,  0), Size::new( 40,  40));
    d.    set_plumbing_position_and_size(Position::new( 0,  0), Size::new( 40,  40));
    e.    set_plumbing_position_and_size(Position::new(40,  0), Size::new( 40,  40));
    f.    set_plumbing_position_and_size(Position::new( 0,  0), Size::new(120,  40));
    g.    set_plumbing_position_and_size(Position::new( 0,  0), Size::new(120,  40));
    h.    set_plumbing_position_and_size(Position::new( 0,  0), Size::new(120,  40));
    i.    set_plumbing_position_and_size(Position::new( 0,  0), Size::new(120,  40));
    bcd.  set_plumbing_position_and_size(Position::new( 0,  0), Size::new( 40,  40));
    bcde. set_plumbing_position_and_size(Position::new(40,  0), Size::new( 80,  40));
    abcde.set_plumbing_position_and_size(Position::new( 0,  0), Size::new(120,  40));
    ghi.  set_plumbing_position_and_size(Position::new( 0, 40), Size::new(120,  40));
    fghi. set_plumbing_position_and_size(Position::new( 0, 40), Size::new(120,  80));
    w.    set_plumbing_position_and_size(Position::new( 0,  0), Size::new(120, 120));
    r.    set_plumbing_position_and_size(Position::new( 0,  0), Size::new(120, 120));
    (r, w, fghi, ghi, abcde, bcde, bcd, a, b, c, d, e, f, g, h, i)
}

// -------------------------------------------------------------------------------------------------
