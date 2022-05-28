#[macro_use]
extern crate bencher;

use bencher::Bencher;
use puzzle_cube::core::prelude::MovesSource;


benchmark_group!(benches, bench_create_move_table);
benchmark_main!(benches);

fn bench_create_move_table(bench: &mut Bencher) {
    bench.iter(create_move_table);    
}


fn create_move_table() {
    MovesSource::create();
}
