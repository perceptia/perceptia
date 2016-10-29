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

use qualia::{Direction, Position, Size, SurfaceId};

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
///  - Top for any normal frame should be first parent with mode not `Leaf` and not `Container`.
///  - Top for any special frame should be itself.
#[test]
fn test_find_top() {
    let mut r = Frame::new_root();
    let mut s1 = Frame::new_workspace("".to_string());
    let mut s2 = Frame::new_workspace("".to_string());
    let mut c1 = Frame::new_container(Horizontal);
    let mut c2 = Frame::new_container(Vertical);
    let mut l = Frame::new_leaf(SurfaceId::new(1), Stacked);

    r.append(&mut s1);
    s1.append(&mut s2);
    s2.append(&mut c1);
    c1.append(&mut c2);
    c2.append(&mut l);

    assertions::assert_frame_equal_exact(&r.find_top().unwrap(), &r);
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

/// Find contiguous frame on the same level.
///
///  - 0*South from A should be A.
///  - 1*South from A should be B.
///  - 1*North from B should be A.
///  - 1*South from B should be NULL.
///
///
///     ┌─────┐
///     │  A  │
///     ├─────┤
///     │  B  │
///     └─────┘
///
#[test]
fn test_find_contiguous_on_the_same_level_one_further() {
    let mut r = Frame::new_root();
    let mut v = Frame::new_container(Vertical);
    let mut a = Frame::new_leaf(SurfaceId::new(1), Stacked);
    let mut b = Frame::new_leaf(SurfaceId::new(2), Stacked);
    r.append(&mut v);
    v.append(&mut a);
    v.append(&mut b);

    // 0*South from A should be A
    let mut p = a.find_contiguous(Direction::South, 0);
    assertions::assert_frame_equal_exact(&p.unwrap(), &a);

    // 1*South from A should be B
    p = a.find_contiguous(Direction::South, 1);
    assertions::assert_frame_equal_exact(&p.unwrap(), &b);

    // 1*North from B should be A
    p = b.find_contiguous(Direction::North, 1);
    assertions::assert_frame_equal_exact(&p.unwrap(), &a);

    // 1*South from B should be None;
    p = b.find_contiguous(Direction::South, 1);
    assert!(p.is_none());

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Find contiguous frame on the same level.
///
/// - 3*East from B should be E.
/// - 5*West from F should be A.
///
///
///     ┌─────┬─────┬─────┬─────┬─────┬─────┐
///     │  A  │  B  │  C  │  D  │  E  │  F  │
///     └─────┴─────┴─────┴─────┴─────┴─────┘
///
#[test]
fn test_find_contiguous_on_the_same_level_many_further() {
    let mut r = Frame::new_root();
    let mut h = Frame::new_container(Horizontal);
    let mut a = Frame::new_leaf(SurfaceId::new(1), Stacked);
    let mut b = Frame::new_leaf(SurfaceId::new(2), Stacked);
    let mut c = Frame::new_leaf(SurfaceId::new(3), Stacked);
    let mut d = Frame::new_leaf(SurfaceId::new(4), Stacked);
    let mut e = Frame::new_leaf(SurfaceId::new(5), Stacked);
    let mut f = Frame::new_leaf(SurfaceId::new(6), Stacked);
    r.append(&mut h);
    h.append(&mut a);
    h.append(&mut b);
    h.append(&mut c);
    h.append(&mut d);
    h.append(&mut e);
    h.append(&mut f);

    // 3*West from B should be E
    let mut p = b.find_contiguous(Direction::East, 3);
    assertions::assert_frame_equal_exact(&p.unwrap(), &e);

    // 5*East from F should be A
    p = f.find_contiguous(Direction::West, 5);
    assertions::assert_frame_equal_exact(&p.unwrap(), &a);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Find contiguous frame on the second level and going from vertcal to horizontal.
///
///  - 1*East from B should be D.
///  - 1*East from A should be BC.
///  - 1*West from C should be A.
///  - 2*East from A should be D
///  - 1*Trunk from C should be BC.
///
///
///     ┌───────┬───────┬───────┐
///     │       │┌─────┐│       │
///     │       ││  B  ││       │
///     │   A   │├─────┤│   D   │
///     │       ││  C  ││       │
///     │       │└─────┘│       │
///     └───────┴───────┴───────┘
///
#[test]
fn test_find_contiguous_on_the_second_level_across() {
    let mut r    = Frame::new_root();
    let mut abcd = Frame::new_container(Horizontal);
    let mut bc   = Frame::new_container(Vertical);
    let mut a    = Frame::new_leaf(SurfaceId::new(1), Stacked);
    let mut b    = Frame::new_leaf(SurfaceId::new(2), Stacked);
    let mut c    = Frame::new_leaf(SurfaceId::new(3), Stacked);
    let mut d    = Frame::new_leaf(SurfaceId::new(4), Stacked);
    r.append(&mut abcd);
    bc.append(&mut b);
    bc.append(&mut c);
    abcd.append(&mut a);
    abcd.append(&mut bc);
    abcd.append(&mut d);

    // 1*East from B should be D
    let mut p = b.find_contiguous(Direction::East, 1);
    assertions::assert_frame_equal_exact(&p.unwrap(), &d);

    // 1*West from A should be BC
    p = a.find_contiguous(Direction::East, 1);
    assertions::assert_frame_equal_exact(&p.unwrap(), &bc);

    // 1*East from C should be A
    p = c.find_contiguous(Direction::West, 1);
    assertions::assert_frame_equal_exact(&p.unwrap(), &a);

    // 2*West from A should be D
    p = a.find_contiguous(Direction::East, 2);
    assertions::assert_frame_equal_exact(&p.unwrap(), &d);

    // 1*Trunk from C should be BC
    p = c.find_contiguous(Direction::Up, 1);
    assertions::assert_frame_equal_exact(&p.unwrap(), &bc);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Find contiguous frame on the third level and going from horizontal to horizontal
///
///  - 1*East from C should be D.
///
///
///     ┌─────────────────────┬─────┐
///     │┌─────┬─────────────┐│     │
///     ││     │┌─────┬─────┐││     │
///     ││  A  ││  B  │  C  │││  D  │
///     ││     │└─────┴─────┘││     │
///     │└─────┴─────────────┘│     │
///     └─────────────────────┴─────┘
///
#[test]
fn test_find_contiguous_on_the_third_level_along() {
    let mut r    = Frame::new_root();
    let mut abcd = Frame::new_container(Horizontal);
    let mut abc  = Frame::new_container(Horizontal);
    let mut bc   = Frame::new_container(Horizontal);
    let mut a    = Frame::new_leaf(SurfaceId::new(1), Stacked);
    let mut b    = Frame::new_leaf(SurfaceId::new(2), Stacked);
    let mut c    = Frame::new_leaf(SurfaceId::new(3), Stacked);
    let mut d    = Frame::new_leaf(SurfaceId::new(4), Stacked);
    r.append(&mut abcd);
    abcd.append(&mut abc);
    abcd.append(&mut d);
    abc.append(&mut a);
    abc.append(&mut bc);
    bc.append(&mut b);
    bc.append(&mut c);

    // 1*East from C should be D
    let p = c.find_contiguous(Direction::East, 1);
    assertions::assert_frame_equal_exact(&p.unwrap(), &d);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// When searching inside ABCDEF and point is inside ABC and ABC is stacked, with A as most top
/// frame, A should be found.
///
///
///     ┏━━━━━━━━━━━━━━━━━━━━━━━━━━┓
///     ┃┌──────────┬─────────────┐┃
///     ┃│ ▛▀▀▀▀▀▀▜ │┌─────┬─────┐│┃
///     ┃│ ▌ ABC ×▐ ││  D  │  E  ││┃
///     ┃│ ▙▄▄▄▄▄▄▟ │└─────┴─────┘│┃
///     ┃└──────────┴─────────────┘┃
///     ┠──────────────────────────┨
///     ┃┌─────────────────┐       ┃
///     ┃│         F       │       ┃
///     ┃└─────────────────┘       ┃
///     ┗━━━━━━━━━━━━━━━━━━━━━━━━━━┛
///
#[test]
fn test_find_stacked_pointed_inside() {
    let (r, _, _, _, a, _, _, _, _, _) = layouts::make_positioned_for_searching();

    let point = Position::new(10, 10);
    let p = r.find_pointed(point);
    assertions::assert_frame_equal_exact(&p, &a);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// When searching inside ABCDEF and point is inside D and D is inside flat frame, D should be
/// found.
///
///
///     ┏━━━━━━━━━━━━━━━━━━━━━━━━┓
///     ┃┌───────┬──────────────┐┃
///     ┃│┌─────┐│ ▛▀▀▀▀▀▜─────┐│┃
///     ┃││ ABC ││ ▌  D ×▐  E  ││┃
///     ┃│└─────┘│ ▙▄▄▄▄▄▟─────┘│┃
///     ┃└───────┴──────────────┘┃
///     ┠────────────────────────┨
///     ┃┌────────────────┐      ┃
///     ┃│        F       │      ┃
///     ┃└────────────────┘      ┃
///     ┗━━━━━━━━━━━━━━━━━━━━━━━━┛
///
#[test]
fn test_find_flat_pointed_inside() {
    let (r, _, _, _, _, _, _, d, _, _) = layouts::make_positioned_for_searching();

    let point = Position::new(50, 10);
    let p = r.find_pointed(point);
    assertions::assert_frame_equal_exact(&p, &d);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// When searching inside ABCDEF and point is outside ABCDEF and directly above
/// ABC and ABC is stacked, with A as most top frame, A should be found.
///
///
///           ×
///     ┏━━━━━━━━━━━━━━━━━━━━━━━━━┓
///     ┃┌─────────┬─────────────┐┃
///     ┃│ ▛▀▀▀▀▀▜ │┌─────┬─────┐│┃
///     ┃│ ▌ ABC ▐ ││  D  │  E  ││┃
///     ┃│ ▙▄▄▄▄▄▟ │└─────┴─────┘│┃
///     ┃└─────────┴─────────────┘┃
///     ┠─────────────────────────┨
///     ┃┌────────────────┐       ┃
///     ┃│        F       │       ┃
///     ┃└────────────────┘       ┃
///     ┗━━━━━━━━━━━━━━━━━━━━━━━━━┛
///
#[test]
fn test_find_stacked_pointed_ouside() {
    let (r, _, _, _, a, _, _, _, _, _) = layouts::make_positioned_for_searching();

    let point = Position::new(20, -10);
    let p = r.find_pointed(point);
    assertions::assert_frame_equal_exact(&p, &a);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// When searching inside ABCDEF and point is outside ABCDEF and directly above
/// D and D is inside flat frame, D should be found.
///
///
///                   ×
///     ┏━━━━━━━━━━━━━━━━━━━━━━━━┓
///     ┃┌───────┬──────────────┐┃
///     ┃│┌─────┐│ ▛▀▀▀▀▀▜─────┐│┃
///     ┃││ ABC ││ ▌  D  ▐  E  ││┃
///     ┃│└─────┘│ ▙▄▄▄▄▄▟─────┘│┃
///     ┃└───────┴──────────────┘┃
///     ┠────────────────────────┨
///     ┃┌────────────────┐      ┃
///     ┃│        F       │      ┃
///     ┃└────────────────┘      ┃
///     ┗━━━━━━━━━━━━━━━━━━━━━━━━┛
///
#[test]
fn test_find_flat_pointed_outside() {
    let (r, _, _, _, _, _, _, d, _, _) = layouts::make_positioned_for_searching();

    let point = Position::new(60, -10);
    let p = r.find_pointed(point);
    assertions::assert_frame_equal_exact(&p, &d);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// When searching inside ABCDE and point is outside ABCDE and below D, but
/// inside another frame not contained in ABCDE, D should be found.
///
///
///     ┌────────────────────────┐
///     │┏━━━━━━━┯━━━━━━━━━━━━━━┓│
///     │┃┌─────┐│ ▛▀▀▀▀▀▜─────┐┃│
///     │┃│ ABC ││ ▌  D  ▐  E  │┃│
///     │┃└─────┘│ ▙▄▄▄▄▄▟─────┘┃│
///     │┗━━━━━━━┷━━━━━━━━━━━━━━┛│
///     ├────────────────────────┤
///     │┌────────────────┐      │
///     ││        F   ×   │      │
///     │└────────────────┘      │
///     └────────────────────────┘
///
#[test]
fn test_find_frame_over_another() {
    let (r, abcde, _, _, _, _, _, d, _, _) = layouts::make_positioned_for_searching();

    let point = Position::new(50, 70);
    let p = abcde.find_pointed(point);
    assertions::assert_frame_equal_exact(&p, &d);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// When searching inside ABCDEF and point is outside of any frame contained in
/// ABCDEF, ABCDEF should be found.
///
///
///     ▐▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▌
///     ▐ ┌───────┬─────────────┐ ▌
///     ▐ │┌─────┐│┌─────┬─────┐│ ▌
///     ▐ ││ ABC │││  D  │  E  ││ ▌
///     ▐ │└─────┘│└─────┴─────┘│ ▌
///     ▐ └───────┴─────────────┘ ▌
///     ▐─────────────────────────▌
///     ▐ ┌────────────────┐      ▌
///     ▐ │        F       │  ×   ▌
///     ▐ └────────────────┘      ▌
///     ▐▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▌
///
#[test]
fn test_find_empty_space() {
    let (r, _, _, _, _, _, _, _, _, _) = layouts::make_positioned_for_searching();

    let point = Position::new(80, 80);
    let p = r.find_pointed(point);
    assertions::assert_frame_equal_exact(&p, &r);

    r.destroy();
}

// -------------------------------------------------------------------------------------------------

/// Find adjacent frames.
///
/// - 1*South from A should be C
/// - 1*South from C should be E
/// - 2*South from A should be F
/// - 1*South from CD should be F
/// - 1*North from AB should be NULL
///
///
///     ┌─────────────────────┐
///     │┌─────────────┬─────┐│
///     ││      A      │  B  ││
///     │└─────────────┴─────┘│
///     ├─────────────────────┤
///     │┌─────────┬─────────┐│
///     ││    C    │    D    ││
///     │└─────────┴─────────┘│
///     ├─────────────────────┤
///     │┌─────┬─────────────┐│
///     ││  E  │      F      ││
///     │└─────┴─────────────┘│
///     └─────────────────────┘
///
#[test]
fn test_find_adjacent_frames() {
    let mut r  = Frame::new_root();
    let mut v  = Frame::new_container(Vertical);
    let mut ab = Frame::new_container(Horizontal);
    let mut cd = Frame::new_container(Horizontal);
    let mut ef = Frame::new_container(Horizontal);
    let mut a  = Frame::new_leaf(SurfaceId::new(1), Stacked);
    let mut b  = Frame::new_leaf(SurfaceId::new(2), Stacked);
    let mut c  = Frame::new_leaf(SurfaceId::new(3), Stacked);
    let mut d  = Frame::new_leaf(SurfaceId::new(4), Stacked);
    let mut e  = Frame::new_leaf(SurfaceId::new(5), Stacked);
    let mut f  = Frame::new_leaf(SurfaceId::new(6), Stacked);
    r.append(&mut v);
    v.append(&mut ab);
    v.append(&mut cd);
    v.append(&mut ef);
    ab.append(&mut a);
    ab.append(&mut b);
    cd.append(&mut c);
    cd.append(&mut d);
    ef.append(&mut e);
    ef.append(&mut f);
    r. set_plumbing_position_and_size(Position::new( 0,  0), Size::new(100, 10));
    v. set_plumbing_position_and_size(Position::new( 0,  0), Size::new(100, 10));
    ab.set_plumbing_position_and_size(Position::new( 0,  0), Size::new(100, 10));
    cd.set_plumbing_position_and_size(Position::new( 0, 10), Size::new(100, 10));
    ef.set_plumbing_position_and_size(Position::new( 0, 20), Size::new(100, 10));
    a. set_plumbing_position_and_size(Position::new( 0,  0), Size::new( 70, 10));
    b. set_plumbing_position_and_size(Position::new(70,  0), Size::new( 30, 10));
    c. set_plumbing_position_and_size(Position::new( 0, 10), Size::new( 50, 10));
    d. set_plumbing_position_and_size(Position::new(50, 10), Size::new( 50, 10));
    e. set_plumbing_position_and_size(Position::new( 0, 20), Size::new( 30, 10));
    f. set_plumbing_position_and_size(Position::new(30, 20), Size::new( 70, 10));

    // 1*South from A should be C
    let mut p = a.find_adjacent(Direction::South, 1);
    assertions::assert_frame_equal_exact(&p.unwrap(), &c);

    // 1*South from C should be E
    p = c.find_adjacent(Direction::South, 1);
    assertions::assert_frame_equal_exact(&p.unwrap(), &e);

    // 2*South from A should be F
    p = a.find_adjacent(Direction::South, 2);
    assertions::assert_frame_equal_exact(&p.unwrap(), &f);

    // 1*South from CD should be F
    p = cd.find_adjacent(Direction::South, 1);
    assertions::assert_frame_equal_exact(&p.unwrap(), &f);

    // 1*North from AB should be None
    p = ab.find_adjacent(Direction::North, 1);
    assert!(p.is_none());

    r.destroy();
}

// -------------------------------------------------------------------------------------------------
