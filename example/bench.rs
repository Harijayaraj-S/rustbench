use rustbench::benchmark;

#[benchmark]
fn example_sum() {
    let _: u64 = (1..=1_000_000).sum();
}

#[benchmark(5)]
fn example_sum_iteration() {
    let _: u64 = (1..=1_000_000).sum();
}

fn main() {
    example_sum_iteration();
    example_sum();
}
