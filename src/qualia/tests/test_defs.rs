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
fn should_correctly_cast_point_into_area() {
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

/// Check if point casted into area is really in area.
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

/// Check if point casted to zero area lays in origin.
#[test]
fn should_casted_to_zero_area_be_in_origin()
{
    let position = Position::new(5, 5);
    let origin = Position::new(10, 10);
    let area = Area::new(origin, Size::new(0, 0));

    let casted = position.casted(&area);
    assert!(position.casted(&area) == origin,
            "Casted poition should be in origin (origin is {:?}, while point is {:?})",
            origin,
            casted);
}

// -------------------------------------------------------------------------------------------------

/// Check if inflated area has correct size.
#[test]
fn should_inflate_area() {
    let tests: [(Area, Area); 10] = [
            (Area::create( 0,  0, 20, 20), Area::create( 0,  0, 40, 40)),
            (Area::create(20,  0, 10, 20), Area::create(10,  0, 30, 40)),
            (Area::create(30,  0, 20, 20), Area::create(10,  0, 40, 40)),
            (Area::create( 0, 20, 20, 10), Area::create( 0, 10, 40, 30)),
            (Area::create(20, 20, 10, 10), Area::create(10, 10, 30, 30)),
            (Area::create(30, 20, 20, 10), Area::create(10, 10, 40, 30)),
            (Area::create( 0, 30, 20, 20), Area::create( 0, 10, 40, 40)),
            (Area::create(20, 30, 10, 20), Area::create(10, 10, 30, 40)),
            (Area::create(30, 30, 20, 20), Area::create(10, 10, 40, 40)),
            (Area::create( 0,  0, 50, 50), Area::create( 0,  0, 50, 50)),
        ];

    for test in &tests {
        let mut area = Area::create(10, 10, 30, 30);
        area.inflate(&test.0);
        assert!(area == test.1,
                "Area inflated by {:?} should be {:?}, is {:?}",
                test.0,
                test.1,
                area);
    }
}

// -------------------------------------------------------------------------------------------------
