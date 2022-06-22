use criterion::{criterion_group, criterion_main, Criterion};
use std::rc::Rc;

use puzzle_cube::core::prelude::*;

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100).measurement_time(instant::Duration::new(5, 0));

    targets=

    bench_create_phase_1_pruning,
    bench_create_phase_2_pruning,
    bench_solver,
    bench_create_move_table,
    bench_create_corner_slice_depth,
    bench_create_ud_edges_conjugation,
    bench_create_corner_symmetries,
    bench_create_flip_slice,
    
    
);
criterion_main!(benches);

fn bench_create_move_table(c: &mut Criterion) {
    c.bench_function("create move table", |bench|bench.iter(create_move_table));    
}

fn bench_create_corner_symmetries(c: &mut Criterion) {
    c.bench_function("create corner symmetries", |bench|bench.iter(create_corner_symmetries));      
}

fn bench_create_flip_slice(c: &mut Criterion) {
    c.bench_function("create flip slice", |bench|bench.iter(create_flip_slice));  
}

fn bench_create_ud_edges_conjugation(c: &mut Criterion) {
    c.bench_function("create ud edges conjugation", |bench|bench.iter(create_ud_edges_conjugation));    
}

fn bench_create_corner_slice_depth(c: &mut Criterion) {
    let moves_source = MovesSource::create();
    c.bench_function("create corner slice depth", |bench|bench.iter(||create_corner_slice_depth(&moves_source)));
}

fn bench_create_phase_2_pruning(c: &mut Criterion) {
    let moves_source = MovesSource::create();
    let corner_source = CornerSymmetriesSource::create();
    c.bench_function("create phase 2 pruning", |bench|bench.iter(||create_phase_2_pruning(&moves_source, &corner_source)));    
}

fn bench_create_phase_1_pruning(c: &mut Criterion) {
    let moves_source = MovesSource::create();
    let flip_slice_source = FlipSliceSource::create();

    let mut group = c.benchmark_group("phase one pruning");
    group.sample_size(10);
    group.measurement_time(instant::Duration::new(100, 0));
    group.bench_function("create phase one pruning slow", |bench|bench.iter(||create_phase_1_pruning(false,&moves_source, &flip_slice_source)));
    group.bench_function("create phase one pruning quick", |bench|bench.iter(||create_phase_1_pruning(true,&moves_source, &flip_slice_source)));
    group.finish()
}

fn bench_solver(c: &mut Criterion) {
    let data_source = Rc::new(DataSource::create(true));

    let mut group = c.benchmark_group("solver");
    group.sample_size(10);
    group.bench_function("solve 100 random cubes", |bench|bench.iter(||solve(data_source.clone(), 100)));
    group.finish()
}

fn create_move_table() -> MovesSource {
    MovesSource::create()
}

fn create_corner_slice_depth(moves_source: &MovesSource) -> Vec<u8> {
    DataSource::create_corner_slice_depth(moves_source)
}

fn create_ud_edges_conjugation() -> Vec<u16> {
    DataSource::create_up_down_edges_conjugation()
}

fn create_corner_symmetries() -> CornerSymmetriesSource {
    CornerSymmetriesSource::create()
}

fn create_flip_slice() -> FlipSliceSource {
    FlipSliceSource::create()
}

fn create_phase_2_pruning(
    move_source: &MovesSource,
    corner_source: &CornerSymmetriesSource,
) -> Vec<u32> {
    DataSource::create_phase_2_pruning(move_source, corner_source)
}

fn create_phase_1_pruning(
    quick: bool,
    move_source: &MovesSource,
    flip_slice_source: &FlipSliceSource,
) -> Vec<u32> {
    DataSource::create_phase_1_pruning(quick, move_source, flip_slice_source)
}

fn solve(data_source: Rc<DataSource>, number: u64) {
    for seed in 0..number {
        let base_cube = CubieCube::random_cube(seed);
        let solution = Solver::get_solution(base_cube, data_source.clone(), SolveSettings::default());
        assert!(solution.is_some());
    }
}
