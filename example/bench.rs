use rustbench::benchmark;

#[benchmark]
fn sum() {
    let sum: u64 = (1..=1_000_000).sum();
    println!("Sum: {}", sum);
}

fn main() {
    sum();
}
