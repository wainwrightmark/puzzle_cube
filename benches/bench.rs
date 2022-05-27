#[macro_use]
extern crate bencher;

use bencher::Bencher;
use puzzle_cube::core::prelude::*;

benchmark_group!(benches, bench_solve);
benchmark_main!(benches);

fn bench_solve(bench: &mut Bencher) {
    bench.iter(|| solve());
}


fn solve() {
    
}
