use std::process::Termination;

use bencher::{benchmark_group, benchmark_main, Bencher};

extern crate bencher;
extern crate test_bin;

fn iter(bench: &mut Bencher) {
    bench.iter(|| {
        test_bin::get_test_bin("neosuggest")
            .args(["cd", "s"])
            .output()
            .expect("Benchmark failed");
    });
}

fn iter2(bench: &mut Bencher) {
    bench.iter(|| {
        test_bin::get_test_bin("neosuggest")
            .args(["cd", "/h"])
            .output()
            .expect("Benchmark failed");
    });
}

fn iter3(bench: &mut Bencher) {
    bench.iter(|| {
        test_bin::get_test_bin("neosuggest")
            .args(["cd", "/usr/local/s"])
            .output()
            .expect("Benchmark failed");
    });
}

benchmark_group!(benches, iter, iter2, iter3);
benchmark_main!(benches);
