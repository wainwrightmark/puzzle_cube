#[macro_use]
extern crate bencher;

use bencher::Bencher;
use puzzle_cube::core::prelude::*;

benchmark_group!(
    benches,
    bench_create_move_table,
    bench_create_corner_slice_depth,
    bench_create_ud_edges_conjugation,
    bench_create_corner_symmetries,
    bench_create_flip_slice,
    bench_create_phase_2_pruning
);
benchmark_main!(benches);

fn bench_create_move_table(bench: &mut Bencher) {
    bench.iter(create_move_table);
}

fn bench_create_corner_symmetries(bench: &mut Bencher) {
    bench.iter(create_corner_symmetries);
}

fn bench_create_flip_slice(bench: &mut Bencher) {
    bench.iter(create_flip_slice);
}

fn bench_create_ud_edges_conjugation(bench: &mut Bencher) {
    bench.iter(create_ud_edges_conjugation);
}

fn bench_create_corner_slice_depth(bench: &mut Bencher) {
    let moves_source = MovesSource::create();
    bench.iter(|| create_corner_slice_depth(&moves_source));
}

fn bench_create_phase_2_pruning(bench: &mut Bencher) {
    let moves_source = MovesSource::create();
    let corner_source = CornerSymmetriesSource::create();
    bench.iter(|| create_phase_2_pruning(&moves_source, &corner_source));
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
