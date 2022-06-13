use itertools::Itertools;

use puzzle_cube::core::prelude::*;

use ntest::test_case;
use rand::{prelude::StdRng, Rng};

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
    let mut expected_corners: u16;

    loop {
        expected_corners = rng.gen_range(0..40320);
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

    let verify_result = cube.verify();
    assert_eq!(verify_result, Ok(()));
}

#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
#[test_case(4)]
#[test_case(5)]
fn test_invert(seed: u64) {
    let cube = CubieCube::random_cube(seed);

    let inverse = cube.invert();

    let mult = cube.multiply(&inverse);

    assert_eq!(CubieCube::default(), mult);
}

#[test]
fn test_default_to_facelet_cube() {
    let cube = CubieCube::default();

    let fc: FaceletCube = cube.clone().into();

    let cube2: CubieCube = fc.try_into().unwrap();

    assert_eq!(cube, cube2);
}

#[test]
fn test_default_up_to_facelet_cube() {
    let cube = CubieCube::default().multiply(&UP_DEFAULT_CUBE);

    let fc: FaceletCube = cube.clone().into();

    let cube2: CubieCube = fc.try_into().unwrap();

    assert_eq!(cube, cube2);
}

#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
#[test_case(4)]
#[test_case(5)]
fn test_to_facelet_cube(seed: u64) {
    let cube = CubieCube::random_cube(seed);

    let fc: FaceletCube = cube.clone().into();

    let cube2: CubieCube = fc.try_into().unwrap();

    assert_eq!(cube, cube2);
}

#[test]
fn any_move_four_times_returns_same() {
    for m in Move::ALLMOVES {
        let base_cube = CubieCube::random_cube(123);
        let mut current = base_cube.clone();
        for _ in 0..4 {
            current = m.apply(&current);
        }

        assert_eq!(base_cube, current, "Move {}", m);
    }
}

#[test]
fn any_move_inverse_is_same_as_three_times() {
    for m in Move::ALLMOVES {
        let base_cube = CubieCube::random_cube(123);
        let mut current = base_cube.clone();
        for _ in 0..3 {
            current = m.apply(&current);
        }

        let inverse = m.apply(&CubieCube::default()).invert();
        let inverse_applied = base_cube.multiply(&inverse);

        assert_eq!(current, inverse_applied, "Move {}", m);
    }
}

#[test_case(0)]
#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
fn test_ud_edges(seed: u8) {
    let mut cube = CubieCube::default();

    let mut rng: StdRng = rand::SeedableRng::seed_from_u64(seed);
    let expected_edges = rng.gen_range(0..40320);

    cube.set_ud_edges(expected_edges);

    let actual = cube.get_ud_edges();
    assert!(actual.is_some());
    assert_eq!(actual.unwrap_or_default(), expected_edges);

    for m in Move::PHASE2MOVES {
        let new_cube = m.apply(&cube);

        let new_actual = new_cube.get_ud_edges();
        assert!(new_actual.is_some());
    }
}

#[test]
fn test_create_corners() {
    let r = CornersProperty::create(&CornersProperty {});

    for c in r {
        assert!(c < 40320)
    }
}

#[test]
fn test_corner_slice_depth() {
    let moves_source = MovesSource::create();

    let csd = DataSource::create_corner_slice_depth(&moves_source);

    for c in csd {
        assert_ne!(c, u8::MAX)
    }
}

#[test]
fn test_phase_two_pruning() {
    let moves_source = MovesSource::create();
    let corner_source = CornerSymmetriesSource::create();

    let table = DataSource::create_phase_2_pruning(&moves_source, &corner_source);

    let expected = vec![
        1040187196, 3738173439, 3148869435, 3270983646, 1022771197, 4286573750, 4286009295,
        3354899262, 4293918719, 4091533311,
    ];

    let sub_table = table.into_iter().take(10).collect_vec();
    assert_eq!(sub_table, expected)
}

#[test]
fn test_phase_one_pruning() {
    let moves_source = MovesSource::create();
    let flip_slice_source = FlipSliceSource::create();

    let table = DataSource::create_phase_1_pruning(&moves_source, &flip_slice_source);

    let expected = vec![
        1704289684, 1494853018, 1503286594, 2439088264, 2551548309, 2757072154, 1704232488,
        2594350426, 2774941993, 2321846938,
    ];

    let sub_table = table.into_iter().take(10).collect_vec();
    assert_eq!(sub_table, expected)
}

#[test]
fn test_phase_two_ud_edge_merge() {
    let table = DataSource::create_phase_2_edge_merge();

    let sub_table = table.into_iter().take(10).collect_vec();
    let expected = vec![
        20160, 20784, 24000, 23904, 24504, 24600, 7200, 7824, 6000, 5904,
    ];
    assert_eq!(sub_table, expected);
}

#[test]
fn test_up_down_edges_conjugation() {
    let table = DataSource::create_up_down_edges_conjugation();

    let expected = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 5, 10, 1, 20, 10, 5, 20, 4440, 624,
        36000, 4440, 26664, 36000, 624, 26664, 2, 4, 11, 14, 7, 23, 16, 19, 4344,
    ];
    let sub_table = table.into_iter().take(expected.len()).collect_vec();

    assert_eq!(sub_table, expected);
}

#[test]
fn test_create_corner_symmetries() {
    let css = CornerSymmetriesSource::create();

    for class_index in css.corner_class_index {
        assert!(class_index < 2768)
    }
}

#[test]
fn test_create_flip_slice_symmetries() {
    let fss = FlipSliceSource::create();

    for class_index in fss.flip_slice_class_index {
        assert!(class_index < 64430)
    }
}

#[test]
fn test_basic_cubes() {
    itertools::assert_equal(
        SYMMETRY_CUBES
            .into_iter()
            .map(CoordinateCube::from)
            .duplicates(),
        vec![],
    );
}

#[test]
fn test_inverse_moves() {
    for m in Move::ALLMOVES {
        let cube = m.get_cube();
        let inverse = cube.invert();

        let product = cube.multiply(&inverse);

        assert_eq!(product, CubieCube::default());
    }
}

#[test]
fn test_urf_3() {
    let cube = URF3_SYMMETRY;
    let cube2 = cube.multiply(&cube);
    let cube3 = cube2.multiply(&cube);
    let inverse = cube.invert();
    let inverse2 = cube2.invert();

    assert_eq!(cube3, CubieCube::default(), "Multiplied by self twice");
    assert_eq!(inverse, cube2, "Inverse");
    assert_eq!(inverse2, cube, "Inverse2");
}

#[test]
fn test_inverse_cubes() {
    for i in 0..48 {
        let product = SYMMETRY_CUBES[i].multiply(&SYMMETRY_CUBES_INVERTED[i]);

        assert_eq!(product, CubieCube::default(), "Cube {}", i);
    }
}

#[test]
fn test_inverse_cubes2() {
    itertools::assert_equal(
        SYMMETRY_CUBES_INVERTED
            .into_iter()
            .map(CoordinateCube::from)
            .duplicates(),
        vec![],
    );
    let basic_cubes =
        std::collections::BTreeSet::from_iter(SYMMETRY_CUBES.into_iter().map(CoordinateCube::from));

    for i in 0..SYMMETRY_CUBES_INVERTED.len() {
        let inverted_cube = SYMMETRY_CUBES_INVERTED[i].clone().into();

        assert!(basic_cubes.contains(&inverted_cube), "Cube {}", i);
    }
}

#[test]
fn test_solve_cube(){

    let data_source = DataSource::create();
    let base_cube = CubieCube::random_cube(100);

    let solution = Solver::get_solution(base_cube.clone(), data_source);

    assert!(solution.is_some());

    let mut solved_cube = base_cube;
    for m in solution.unwrap(){
        solved_cube = m.apply(&solved_cube);
    }
    assert_eq!(solved_cube, CubieCube::default());

    
}

#[test]
fn test_symmetry_cubes(){
    assert_eq!(SYMMETRY_CUBES[0], CubieCube::default()) ;

    assert_eq!(SYMMETRY_CUBES[1], MIRROR_LR2_SYMMETRY) ;
    assert_eq!(SYMMETRY_CUBES[2], U4_SYMMETRY) ;
    assert_eq!(SYMMETRY_CUBES[8], F2_SYMMETRY) ;
    assert_eq!(SYMMETRY_CUBES[16], URF3_SYMMETRY) ;


    assert_eq!(SYMMETRY_CUBES[3], U4_SYMMETRY.multiply(&MIRROR_LR2_SYMMETRY)) ;
    assert_eq!(SYMMETRY_CUBES[17], URF3_SYMMETRY.multiply(&MIRROR_LR2_SYMMETRY)) ;

}