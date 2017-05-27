// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for settling `Frame` functionality.

#![cfg_attr(rustfmt, rustfmt_skip)]

// -------------------------------------------------------------------------------------------------

extern crate frames;
extern crate qualia;
extern crate testing;

mod common;

use frames::{Frame, Parameters};
use frames::Geometry::{Horizontal, Stacked, Vertical};
use frames::Side::{Before, On, After};
use frames::settling::Settling;

use qualia::{Position, Size, SurfaceId};
use testing::frame_representation::FrameRepresentation;

use common::{assertions, layouts, surface_access_mock};

// -------------------------------------------------------------------------------------------------

/// Test popping of directed frame.
///
/// Given frame should be popped as well as its parent.
/// Spatial order should be preserved.
#[test]
fn test_poping_directed() {
    let (mut r, _, _, _, _, _, _, _, mut h2, _, _, _, _) = layouts::make_simple_frames_appending();

    r.pop_recursively(&mut h2);

    let repr = FrameRepresentation::new(
        Parameters::new_workspace(String::new(), Vertical),
        vec![
            FrameRepresentation::new(
                Parameters::new_container(Horizontal),
                vec![
                    FrameRepresentation::new_leaf(22, Stacked),
                    FrameRepresentation::new_leaf(21, Stacked),
                    FrameRepresentation::new_leaf(23, Stacked),
                ]
            ),
            FrameRepresentation::new(
                Parameters::new_container(Vertical),
                vec![
                    FrameRepresentation::new_leaf(11, Stacked),
                    FrameRepresentation::new_leaf(12, Stacked),
                    FrameRepresentation::new_leaf(13, Stacked),
                ]
            ),
            FrameRepresentation::new(
                Parameters::new_container(Stacked),
                vec![
                    FrameRepresentation::new_leaf(31, Stacked),
                    FrameRepresentation::new_leaf(32, Stacked),
                    FrameRepresentation::new_leaf(33, Stacked),
                ]
            ),
        ]
    );

    assertions::assert_simple_frames_spaced(&r);
    repr.assert_frames_timed(&r);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Test popping of stacked frame.
///
/// Given frame should be popped as well as its parent.
/// Frames in stacked should also be popped in spatial order.
#[test]
fn test_poping_stacked() {
    let (mut r, _, _, _, _, _, _, _, _, _, _, mut s2, _) = layouts::make_simple_frames_appending();

    r.pop_recursively(&mut s2);

    let spaced_repr = FrameRepresentation::new(
        Parameters::new_workspace(String::new(), Vertical),
        vec![
            FrameRepresentation::new(
                Parameters::new_container(Vertical),
                vec![
                    FrameRepresentation::new_leaf(11, Stacked),
                    FrameRepresentation::new_leaf(12, Stacked),
                    FrameRepresentation::new_leaf(13, Stacked),
                ]
            ),
            FrameRepresentation::new(
                Parameters::new_container(Horizontal),
                vec![
                    FrameRepresentation::new_leaf(21, Stacked),
                    FrameRepresentation::new_leaf(22, Stacked),
                    FrameRepresentation::new_leaf(23, Stacked),
                ]
            ),
            FrameRepresentation::new(
                Parameters::new_container(Stacked),
                vec![
                    FrameRepresentation::new_leaf(32, Stacked),
                    FrameRepresentation::new_leaf(31, Stacked),
                    FrameRepresentation::new_leaf(33, Stacked),
                ]
            ),
        ]
    );

    let timed_repr = FrameRepresentation::new(
        Parameters::new_workspace(String::new(), Vertical),
        vec![
            FrameRepresentation::new(
                Parameters::new_container(Stacked),
                vec![
                    FrameRepresentation::new_leaf(32, Stacked),
                    FrameRepresentation::new_leaf(31, Stacked),
                    FrameRepresentation::new_leaf(33, Stacked),
                ]
            ),
            FrameRepresentation::new(
                Parameters::new_container(Vertical),
                vec![
                    FrameRepresentation::new_leaf(11, Stacked),
                    FrameRepresentation::new_leaf(12, Stacked),
                    FrameRepresentation::new_leaf(13, Stacked),
                ]
            ),
            FrameRepresentation::new(
                Parameters::new_container(Horizontal),
                vec![
                    FrameRepresentation::new_leaf(21, Stacked),
                    FrameRepresentation::new_leaf(22, Stacked),
                    FrameRepresentation::new_leaf(23, Stacked),
                ]
            ),
        ]
    );

    spaced_repr.assert_frames_spaced(&r);
    timed_repr.assert_frames_timed(&r);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Check if leaf frame is correctly ramified.
#[test]
fn test_ramifing_leaf() {
    let (r, v, h, s, _, _, mut v3, _, _, _, _, _, _) = layouts::make_simple_frames_appending();

    let geometry = Horizontal;
    let d = v3.ramify(geometry);

    assertions::assert_frame_equal_exact(&v3.get_parent().unwrap(), &d);
    assertions::assert_frame_equal_exact(&d.get_parent().unwrap(), &v);
    assertions::assert_frame_equal_exact(&v.get_parent().unwrap(), &r);

    assert_eq!(r.count_children(), 3);
    assert_eq!(v.count_children(), 3);
    assert_eq!(h.count_children(), 3);
    assert_eq!(s.count_children(), 3);
    assert_eq!(d.count_children(), 1);

    assertions::assert_area(&d, Position::new(0, 20), Size::new(30, 10));
    assertions::assert_area(&v3, Position::new(0, 0), Size::new(30, 10));

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Check if non-leaf frame is correctly ramified.
#[test]
fn test_ramifing_nonleaf() {
    let (r, mut v, h, s, _, _, _, _, _, _, _, _, _) = layouts::make_simple_frames_appending();

    let geometry = Horizontal;
    let d = v.ramify(geometry);

    assertions::assert_frame_equal_exact(&d.get_parent().unwrap(), &r);
    assertions::assert_frame_equal_exact(&v.get_parent().unwrap(), &d);

    assert_eq!(r.count_children(), 3);
    assert_eq!(v.count_children(), 3);
    assert_eq!(h.count_children(), 3);
    assert_eq!(s.count_children(), 3);
    assert_eq!(d.count_children(), 1);

    assertions::assert_area(&d, Position::new(0, 0), Size::new(30, 30));
    assertions::assert_area(&v, Position::new(0, 0), Size::new(30, 30));

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Check if request to ramify single child will be ignored and parent will be returned.
///
/// In this case ramification would be unnecessary because structure need by ramifying operation is
/// already in place.
#[test]
fn test_ramifing_single_child() {
    let (r, mut a1, _, _, mut f, _, _, _, _, _) = layouts::make_simple_for_deramifying();

    let d = f.ramify(Horizontal);
    assertions::assert_frame_equal_exact(&d, &a1);
    assertions::assert_frame_equal_exact(&f.get_parent().unwrap(), &a1);
    assert_eq!(d.count_children(), 1);

    let d = a1.ramify(Horizontal);
    assertions::assert_frame_equal_exact(&d, &a1);
    assertions::assert_frame_equal_exact(&f.get_parent().unwrap(), &a1);
    assert_eq!(d.count_children(), 1);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Should deramify frame with single non-leaf twig.
#[test]
fn should_deramify_single_nonleaf() {
    let (r, a1, mut a2, _, _, _, c, _, _, _) = layouts::make_simple_for_deramifying();

    a2.deramify();

    assertions::assert_frame_equal_exact(&a2.get_parent().unwrap(), &r);
    assertions::assert_frame_equal_exact(&c.get_parent().unwrap(), &a2);

    assert_eq!(r.count_children(), 3);
    assert_eq!(a1.count_children(), 1);
    assert_eq!(a2.count_children(), 1);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Should deramify frame with single leaf twig.
#[test]
fn should_deramify_with_one_leaf() {
    let (r, mut a1, a2, _, _, b, c, d1, d2, d3) = layouts::make_simple_for_deramifying();

    a1.deramify();

    assertions::assert_frame_equal_exact(&a2.get_parent().unwrap(), &r);
    assertions::assert_frame_equal_exact(&a1.get_parent().unwrap(), &r);
    assertions::assert_frame_equal_exact(&b.get_parent().unwrap(), &a2);
    assertions::assert_frame_equal_exact(&c.get_parent().unwrap(), &b);
    assertions::assert_frame_equal_exact(&d1.get_parent().unwrap(), &c);
    assertions::assert_frame_equal_exact(&d2.get_parent().unwrap(), &c);
    assertions::assert_frame_equal_exact(&d3.get_parent().unwrap(), &c);

    assert_eq!(r.count_children(), 3);
    assert_eq!(a1.count_children(), 0);
    assert_eq!(a2.count_children(), 1);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Should not deramify frame with many twigs.
#[test]
fn should_not_deramify_not_single() {
    let (mut r, a1, a2, _, f, b, c, d1, d2, d3) = layouts::make_simple_for_deramifying();

    r.deramify();

    assertions::assert_frame_equal_exact(&a2.get_parent().unwrap(), &r);
    assertions::assert_frame_equal_exact(&a1.get_parent().unwrap(), &r);
    assertions::assert_frame_equal_exact(&f.get_parent().unwrap(), &a1);
    assertions::assert_frame_equal_exact(&b.get_parent().unwrap(), &a2);
    assertions::assert_frame_equal_exact(&c.get_parent().unwrap(), &b);
    assertions::assert_frame_equal_exact(&d1.get_parent().unwrap(), &c);
    assertions::assert_frame_equal_exact(&d2.get_parent().unwrap(), &c);
    assertions::assert_frame_equal_exact(&d3.get_parent().unwrap(), &c);

    assert_eq!(r.count_children(), 3);
    assert_eq!(a1.count_children(), 1);
    assert_eq!(a2.count_children(), 1);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Should not deramify frame with many leaf twigs.
#[test]
fn should_not_deramify_with_many_leafs() {
    let (r, a1, a2, _, f, mut b, c, d1, d2, d3) = layouts::make_simple_for_deramifying();

    b.deramify();

    assertions::assert_frame_equal_exact(&a2.get_parent().unwrap(), &r);
    assertions::assert_frame_equal_exact(&a1.get_parent().unwrap(), &r);
    assertions::assert_frame_equal_exact(&f.get_parent().unwrap(), &a1);
    assertions::assert_frame_equal_exact(&b.get_parent().unwrap(), &a2);
    assertions::assert_frame_equal_exact(&c.get_parent().unwrap(), &b);
    assertions::assert_frame_equal_exact(&d1.get_parent().unwrap(), &c);
    assertions::assert_frame_equal_exact(&d2.get_parent().unwrap(), &c);
    assertions::assert_frame_equal_exact(&d3.get_parent().unwrap(), &c);

    assert_eq!(r.count_children(), 3);
    assert_eq!(a1.count_children(), 1);
    assert_eq!(a2.count_children(), 1);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Check if new frame if correctly inserted before given frame.
#[test]
fn should_jumpin_before() {
    let mut sa = surface_access_mock::SurfaceAccessMock::new();
    let (r, v, h, s, v1, mut v2, v3, _, _, _, _, _, _) = layouts::make_simple_frames_appending();

    let mut f = Frame::new_leaf(SurfaceId::new(66), Stacked);
    f.jumpin(Before, &mut v2, &mut sa);

    assertions::assert_frame_equal_exact(&f.get_parent().unwrap(), &v);
    assertions::assert_frame_equal_exact(&v1.get_parent().unwrap(), &v);
    assertions::assert_frame_equal_exact(&v2.get_parent().unwrap(), &v);
    assertions::assert_frame_equal_exact(&v3.get_parent().unwrap(), &v);
    assertions::assert_frame_equal_exact(&v.get_parent().unwrap(), &r);

    assert_eq!(r.count_children(), 3);
    assert_eq!(v.count_children(), 4);
    assert_eq!(h.count_children(), 3);
    assert_eq!(s.count_children(), 3);

    let spaced_repr = FrameRepresentation::new(
        Parameters::new_container(Vertical),
        vec![
            FrameRepresentation::new_leaf(11, Stacked),
            FrameRepresentation::new_leaf(66, Stacked),
            FrameRepresentation::new_leaf(12, Stacked),
            FrameRepresentation::new_leaf(13, Stacked),
        ]
    );

    spaced_repr.assert_frames_spaced(&v);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Check if new frame if correctly inserted after given frame.
#[test]
fn should_jumpin_after() {
    let mut sa = surface_access_mock::SurfaceAccessMock::new();
    let (r, v, h, s, v1, mut v2, v3, _, _, _, _, _, _) = layouts::make_simple_frames_appending();

    let mut f = Frame::new_leaf(SurfaceId::new(66), Stacked);
    f.jumpin(After, &mut v2, &mut sa);

    assertions::assert_frame_equal_exact(&f.get_parent().unwrap(), &v);
    assertions::assert_frame_equal_exact(&v1.get_parent().unwrap(), &v);
    assertions::assert_frame_equal_exact(&v2.get_parent().unwrap(), &v);
    assertions::assert_frame_equal_exact(&v3.get_parent().unwrap(), &v);
    assertions::assert_frame_equal_exact(&v.get_parent().unwrap(), &r);

    assert_eq!(r.count_children(), 3);
    assert_eq!(v.count_children(), 4);
    assert_eq!(h.count_children(), 3);
    assert_eq!(s.count_children(), 3);

    let spaced_repr = FrameRepresentation::new(
        Parameters::new_container(Vertical),
        vec![
            FrameRepresentation::new_leaf(11, Stacked),
            FrameRepresentation::new_leaf(12, Stacked),
            FrameRepresentation::new_leaf(66, Stacked),
            FrameRepresentation::new_leaf(13, Stacked),
        ]
    );

    spaced_repr.assert_frames_spaced(&v);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Check if new frame if correctly inserted on given frame.
#[test]
fn should_jumpin_on() {
    let mut sa = surface_access_mock::SurfaceAccessMock::new();
    let (r, v, h, s, _, mut v2, _, _, _, _, _, _, _) = layouts::make_simple_frames_appending();

    let mut f = Frame::new_leaf(SurfaceId::new(66), Stacked);
    f.jumpin(On, &mut v2, &mut sa);

    assertions::assert_frame_equal_exact(&v2.get_parent().unwrap(), &f.get_parent().unwrap());
    assertions::assert_frame_equal_exact(&v2.get_parent().unwrap().get_parent().unwrap(), &v);
    assertions::assert_frame_equal_exact(&v.get_parent().unwrap(), &r);

    assert_eq!(r.count_children(), 3);
    assert_eq!(v.count_children(), 3);
    assert_eq!(h.count_children(), 3);
    assert_eq!(s.count_children(), 3);
    assert_eq!(f.get_parent().unwrap().count_children(), 2);

    let spaced_repr = FrameRepresentation::new(
        Parameters::new_container(Vertical),
        vec![
            FrameRepresentation::new_leaf(11, Stacked),
            FrameRepresentation::new(
                Parameters::new_container(Stacked),
                vec![
                    FrameRepresentation::new_leaf(66, Stacked),
                    FrameRepresentation::new_leaf(12, Stacked),
                ]
            ),
            FrameRepresentation::new_leaf(13, Stacked),
        ]
    );

    spaced_repr.assert_frames_spaced(&v);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Check if new frame if correctly inserted after given frame.
#[test]
fn should_jump_after_on_the_same_level() {
    let mut sa = surface_access_mock::SurfaceAccessMock::new();
    let (r, _, fghi, mut ghi, _, _, _, _, _, _, _, _, mut f, _, _, _) =
        layouts::make_positioned_for_jumping();

    f.jump(After, &mut ghi, &mut sa);

    assertions::assert_frame_equal_exact(&f.get_parent().unwrap(), &fghi);
    assertions::assert_frame_equal_exact(&ghi.get_parent().unwrap(), &fghi);

    assert_eq!(fghi.count_children(), 2);

    let spaced_repr = FrameRepresentation::new(
        Parameters::new_container(Vertical),
        vec![
            FrameRepresentation::new(
                Parameters::new_container(Stacked),
                vec![
                    FrameRepresentation::new_leaf(7, Stacked),
                    FrameRepresentation::new_leaf(8, Stacked),
                    FrameRepresentation::new_leaf(9, Stacked),
                ]
            ),
            FrameRepresentation::new_leaf(6, Stacked),
        ]
    );

    spaced_repr.assert_frames_spaced(&fghi);

    r.destroy();
}

//------------------------------------------------------------------------------

/// Check if new frame if correctly inserted before given frame.
#[test]
fn should_jump_before_on_the_same_level() {
    let mut sa = surface_access_mock::SurfaceAccessMock::new();
    let (r, w, mut fghi, _, mut abcde, _, _, _, _, _, _, _, _, _, _, _) =
        layouts::make_positioned_for_jumping();

    fghi.jump(Before, &mut abcde, &mut sa);

    assertions::assert_frame_equal_exact(&fghi.get_parent().unwrap(), &w);
    assertions::assert_frame_equal_exact(&abcde.get_parent().unwrap(), &w);

    assert_eq!(w.count_children(), 2);

    let spaced_repr = FrameRepresentation::new(
        Parameters::new_container(Vertical),
        vec![
            FrameRepresentation::new(
                Parameters::new_container(Vertical),
                vec![
                    FrameRepresentation::new_leaf(6, Stacked),
                    FrameRepresentation::new(
                        Parameters::new_container(Stacked),
                        vec![
                            FrameRepresentation::new_leaf(7, Stacked),
                            FrameRepresentation::new_leaf(8, Stacked),
                            FrameRepresentation::new_leaf(9, Stacked),
                        ]
                    ),
                ]
            ),
            FrameRepresentation::new(
                Parameters::new_container(Horizontal),
                vec![
                    FrameRepresentation::new_leaf(1, Stacked),
                    FrameRepresentation::new(
                        Parameters::new_container(Horizontal),
                        vec![
                            FrameRepresentation::new(
                                Parameters::new_container(Stacked),
                                vec![
                                    FrameRepresentation::new_leaf(2, Stacked),
                                    FrameRepresentation::new_leaf(3, Stacked),
                                    FrameRepresentation::new_leaf(4, Stacked),
                                ]
                            ),
                            FrameRepresentation::new_leaf(5, Stacked),
                        ]
                    )
                ]
            )
        ]
    );

    spaced_repr.assert_frames_spaced(&w);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------
