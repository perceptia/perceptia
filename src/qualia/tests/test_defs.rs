// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for helper structures.

// -------------------------------------------------------------------------------------------------

extern crate qualia;

use self::qualia::defs::{Area, Position, Size};

// -------------------------------------------------------------------------------------------------

/// Test if checking point is contained in area.
#[test]
fn should_correctly_check_if_point_is_inside_area() {
    let area = Area::new(Position::new(10, 10), Size::new(10, 10));

    let inside_positions: [Position; 5] = [
            Position::new(15, 15),
            Position::new(10, 10),
            Position::new(10, 19),
            Position::new(19, 10),
            Position::new(19, 19),
        ];

    let outside_positions: [Position; 5] = [
            Position::new( 0,  0),
            Position::new( 9, 15),
            Position::new(20, 15),
            Position::new(15,  9),
            Position::new(15, 20),
        ];

    for pos in &inside_positions {
        assert!(area.contains(pos), "{:?} should contain {:?}", area, pos);
        assert!(pos.is_inside(&area), "{:?} should be inside {:?}", pos, area);
    }

    for pos in &outside_positions {
        assert!(!area.contains(pos), "{:?} should not contain {:?}", area, pos);
        assert!(!pos.is_inside(&area), "{:?} should not be inside {:?}", pos, area);
    }
}

// -------------------------------------------------------------------------------------------------

/// Check if casting point into area works correctly.
#[test]
fn should_correctly_cast_point_into_area()
{
    let area = Area::new(Position::new(10, 10), Size::new(10, 10));

    let positions: [(Position, Position); 9] = [
            (Position::new(15, 15), Position::new(15, 15)),
            (Position::new( 0, 15), Position::new(10, 15)),
            (Position::new(30, 15), Position::new(19, 15)),
            (Position::new(15,  0), Position::new(15, 10)),
            (Position::new(15, 30), Position::new(15, 19)),
            (Position::new( 0,  0), Position::new(10, 10)),
            (Position::new(30,  0), Position::new(19, 10)),
            (Position::new( 0, 30), Position::new(10, 19)),
            (Position::new(30, 30), Position::new(19, 19)),
        ];

    for pair in &positions {
        assert_eq!(pair.1,
                   pair.0.casted(&area),
                   "Casted position should be {:?} (is {:?})",
                   pair.1,
                   pair.0.casted(&area));
    }
}

// -------------------------------------------------------------------------------------------------

/// Check if point casted into area is realy in area.
#[test]
fn should_casted_be_inside_area()
{
    let area = Area::new(Position::new(10, 10), Size::new(10, 10));
    let positions: [Position; 5] = [
            Position::new(30, 30),
            Position::new(20, 20),
            Position::new(15, 15),
            Position::new(10, 10),
            Position::new( 9,  9),
        ];

    for pos in &positions {
        assert!(area.contains(&pos.casted(&area)),
                "Casted position should be inside area (area is {:?}, while point is {:?})",
                area,
                pos.casted(&area));
    }
}

// -------------------------------------------------------------------------------------------------
