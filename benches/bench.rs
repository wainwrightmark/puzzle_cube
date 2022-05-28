#[macro_use]
extern crate bencher;

use bencher::Bencher;
use puzzle_cube::core::prelude::*;


benchmark_group!(benches, bench_create_move_table);//, bench_create_corner_slice_depth);
benchmark_main!(benches);

fn bench_create_move_table(bench: &mut Bencher) {
    bench.iter(create_move_table);    
}

// fn bench_create_corner_slice_depth(bench: &mut Bencher) {

//     let moves_source = MovesSource::create();

//     bench.iter( ||create_corner_slice_depth(&moves_source));    
// }

fn create_move_table() -> MovesSource {
    MovesSource::create()
}



// fn create_corner_slice_depth(moves_source: &MovesSource) -> [u8;40320 * 24] {
//     DataSource::create_corner_slice_depth(moves_source)
// }
