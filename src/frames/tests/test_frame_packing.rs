// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for packing `Frame` functionality.

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
    let (r, mut abcdefghi, hi, abcdef, ef, bcd, a, b, c, d, e, f, g, h, i) =
        layouts::make_sized_for_homogenizing();

    abcdefghi.homogenize(&mut sa);

    assertions::assert_area(&r,         Position::new(  0,   0), Size::new(360, 360));
    assertions::assert_area(&abcdefghi, Position::new(  0,   0), Size::new(360, 360));
    assertions::assert_area(&abcdef,    Position::new(  0,   0), Size::new(120, 360));
    assertions::assert_area(&hi,        Position::new(240,   0), Size::new(120, 360));
    assertions::assert_area(&ef,        Position::new(  0, 300), Size::new(120,  60));
    assertions::assert_area(&bcd,       Position::new(  0, 120), Size::new(120, 180));
    assertions::assert_area(&a,         Position::new(  0,   0), Size::new(120, 120));
    assertions::assert_area(&b,         Position::new(  0, 120), Size::new(120, 180));
    assertions::assert_area(&c,         Position::new(  0, 120), Size::new(120, 180));
    assertions::assert_area(&d,         Position::new(  0, 120), Size::new(120, 180));
    assertions::assert_area(&e,         Position::new(  0, 300), Size::new( 60,  60));
    assertions::assert_area(&f,         Position::new( 60, 300), Size::new( 60,  60));
    assertions::assert_area(&g,         Position::new(120,   0), Size::new(120, 360));
    assertions::assert_area(&h,         Position::new(240,   0), Size::new( 80, 360));
    assertions::assert_area(&i,         Position::new(320,   0), Size::new( 40, 360));

    r.destroy();
}

// -------------------------------------------------------------------------------------------------
