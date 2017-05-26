// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for packing `Frame` functionality.

#![cfg_attr(rustfmt, rustfmt_skip)]

// -------------------------------------------------------------------------------------------------

extern crate frames;

extern crate qualia;
extern crate testing;

mod common;

use frames::packing::Packing;

use common::{assertions, layouts, surface_access_mock};

use qualia::{Position, Size};

// -------------------------------------------------------------------------------------------------

/// Test homogenizing frames.
#[test]
fn test_homogenizing() {
    let mut sa = surface_access_mock::SurfaceAccessMock::new();
    let (mut r, abcdefghi, hi, abcdef, ef, bcd, a, b, c, d, e, f, g, h, i, z) =
        layouts::make_sized_for_homogenizing();

    r.homogenize(&mut sa);

    assertions::assert_area(&r,         Position::new(  0,   0), Size::new(360, 360));
    assertions::assert_area(&abcdefghi, Position::new(  0,   0), Size::new(360, 360));
    assertions::assert_area(&abcdef,    Position::new(  0,   0), Size::new(120, 360));
    assertions::assert_area(&hi,        Position::new(240,   0), Size::new(120, 360));
    assertions::assert_area(&ef,        Position::new(  0, 300), Size::new(120,  60));
    assertions::assert_area(&bcd,       Position::new(  0, 120), Size::new(120, 180));
    assertions::assert_area(&a,         Position::new(  0,   0), Size::new(120, 120));
    assertions::assert_area(&b,         Position::new(  0,   0), Size::new(120, 180));
    assertions::assert_area(&c,         Position::new(  0,   0), Size::new(120, 180));
    assertions::assert_area(&d,         Position::new(  0,   0), Size::new(120, 180));
    assertions::assert_area(&e,         Position::new(  0,   0), Size::new( 60,  60));
    assertions::assert_area(&f,         Position::new( 60,   0), Size::new( 60,  60));
    assertions::assert_area(&g,         Position::new(120,   0), Size::new(120, 360));
    assertions::assert_area(&h,         Position::new(  0,   0), Size::new( 80, 360));
    assertions::assert_area(&i,         Position::new( 80,   0), Size::new( 40, 360));
    assertions::assert_area(&z,         Position::new( 13,  23), Size::new( 33,  43));

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Test if removing frame with siblings works correctly.
#[test]
fn test_removing_self_with_siblings() {
    let mut sa = surface_access_mock::SurfaceAccessMock::new();
    let (r, _, _, _, _, _, c, d1, mut d2, d3) = layouts::make_simple_for_deramifying();

    d2.remove_self(&mut sa);

    assertions::assert_frame_equal_exact(&d1.get_parent().unwrap(), &c);
    assertions::assert_frame_equal_exact(&d3.get_parent().unwrap(), &c);
    assert_eq!(c.count_children(), 2);

    d2.destroy();
    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Test if removing frame without siblings works correctly. If removed frame was the only child,
/// parent should also be removed.
#[test]
fn test_removing_self_without_siblings() {
    let mut sa = surface_access_mock::SurfaceAccessMock::new();
    let (r, a1, a2, a3, mut f, _, _, _, _, _) = layouts::make_simple_for_deramifying();

    f.remove_self(&mut sa);

    assertions::assert_frame_equal_exact(&a2.get_parent().unwrap(), &r);
    assertions::assert_frame_equal_exact(&a3.get_parent().unwrap(), &r);
    assert_eq!(r.count_children(), 2);

    a1.destroy();
    f.destroy();
    r.destroy();
}

// -------------------------------------------------------------------------------------------------
