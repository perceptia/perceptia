// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for searching `Frame` functionality.

// -------------------------------------------------------------------------------------------------

extern crate frames;
extern crate qualia;

mod common;

use frames::Frame;
use frames::Geometry::{Horizontal, Stacked, Vertical};
use frames::searching::Searching;

use common::{assertions, layouts};

use qualia::SurfaceId;

// -------------------------------------------------------------------------------------------------

/// Test finding buildable frame.
///
///  - Buildable for leaf should be its parent.
///  - Buildable for container should be itself.
#[test]
fn test_find_buildable() {
    let mut r = Frame::new_root();
    let mut c = Frame::new_container(Vertical);
    let mut l = Frame::new_leaf(SurfaceId::new(1), Stacked);

    r.append(&mut c);
    c.append(&mut l);

    assertions::assert_frame_equal_exact(&l.find_buildable().unwrap(), &c);
    assertions::assert_frame_equal_exact(&c.find_buildable().unwrap(), &c);
}

// -------------------------------------------------------------------------------------------------

/// Test finding top frame.
///
///  - Top for any normal frame should be first parent with mode `Special`.
///  - Top for any special frame should be itself.
#[test]
fn test_find_top() {
    let mut r = Frame::new_root();
    let mut s1 = Frame::new_workspace();
    let mut s2 = Frame::new_workspace();
    let mut c1 = Frame::new_container(Horizontal);
    let mut c2 = Frame::new_container(Vertical);
    let mut l = Frame::new_leaf(SurfaceId::new(1), Stacked);

    r.append(&mut s1);
    s1.append(&mut s2);
    s2.append(&mut c1);
    c1.append(&mut c2);
    c2.append(&mut l);

    assert!(&r.find_top().is_none());
    assertions::assert_frame_equal_exact(&s1.find_top().unwrap(), &s1);
    assertions::assert_frame_equal_exact(&s2.find_top().unwrap(), &s2);
    assertions::assert_frame_equal_exact(&c1.find_top().unwrap(), &s2);
    assertions::assert_frame_equal_exact(&c2.find_top().unwrap(), &s2);
    assertions::assert_frame_equal_exact(&l.find_top().unwrap(), &s2);
}

// -------------------------------------------------------------------------------------------------

/// Check finding frame with sid.
#[test]
fn test_find_with_sid() {
    let (r, _, _, s, v1, v2, v3, h1, h2, h3, s1, s2, s3) = layouts::make_simple_frames_appending();

    assert!(&r.find_with_sid(SurfaceId::new(666)).is_none());

    assertions::assert_frame_equal_exact(&r.find_with_sid(SurfaceId::new(11)).unwrap(), &v1);
    assertions::assert_frame_equal_exact(&r.find_with_sid(SurfaceId::new(12)).unwrap(), &v2);
    assertions::assert_frame_equal_exact(&r.find_with_sid(SurfaceId::new(13)).unwrap(), &v3);
    assertions::assert_frame_equal_exact(&r.find_with_sid(SurfaceId::new(21)).unwrap(), &h1);
    assertions::assert_frame_equal_exact(&r.find_with_sid(SurfaceId::new(22)).unwrap(), &h2);
    assertions::assert_frame_equal_exact(&r.find_with_sid(SurfaceId::new(23)).unwrap(), &h3);
    assertions::assert_frame_equal_exact(&r.find_with_sid(SurfaceId::new(31)).unwrap(), &s1);
    assertions::assert_frame_equal_exact(&r.find_with_sid(SurfaceId::new(32)).unwrap(), &s2);
    assertions::assert_frame_equal_exact(&r.find_with_sid(SurfaceId::new(33)).unwrap(), &s3);

    assert!(&s.find_with_sid(SurfaceId::new(11)).is_none());
    assert!(&s.find_with_sid(SurfaceId::new(12)).is_none());
    assert!(&s.find_with_sid(SurfaceId::new(22)).is_none());

    r.destroy();
}

// -------------------------------------------------------------------------------------------------
