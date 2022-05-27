use std::default;

use puzzle_cube::{core::prelude::{CubieCube, Move}, web::prelude::Cube};

use ntest::test_case;
use rand::{SeedableRng, prelude::StdRng, Rng};

#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
#[test_case(4)]
#[test_case(5)]
fn test_make_random_cube(seed: u64) {
    let mut cube = CubieCube::default();

    let mut rng: StdRng = rand::SeedableRng::seed_from_u64(seed);
    let expected_edges = rng.gen_range(0..479001600);

    cube.set_edges(expected_edges);
    let edge_parity = cube.get_edge_parity();
    let mut expected_corners:u16 = Default::default();

    loop {
        expected_corners =rng.gen_range(0..40320);
        cube.set_corners(expected_corners);
        let corner_parity = cube.get_corner_parity();
        if edge_parity == corner_parity {
            break;
        }
    }

    let expected_flip = rng.gen_range(0..2048);
    cube.set_flip(expected_flip);

    let expected_twist = rng.gen_range(0..2187);

    cube.set_twist(expected_twist);

    //TODO actual edges
    let actual_corners = cube.get_corners();
    let actual_flip = cube.get_flip();
    let actual_twist = cube.get_twist();

    assert_eq!(actual_corners, expected_corners);
    assert_eq!(actual_flip, expected_flip);
    assert_eq!(actual_twist, expected_twist);

    
}


#[test_case(0)]
#[test_case(3)]
#[test_case(6)]
#[test_case(9)]
#[test_case(12)]
#[test_case(15)]
fn any_move_four_times_returns_same(u: u8){

    let m = Move::from_repr(u).unwrap();
    let base_cube = CubieCube::random_cube(123);
    let mut current = base_cube.clone();
    for _ in 0..4 {
        current = m.apply(&current);
    }

    assert_eq!(base_cube, current, "Move {}", m);
}