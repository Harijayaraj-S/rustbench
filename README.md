# Rustbench

`rustbench` is a lightweight Rust procedural macro for benchmarking function execution time.

## Installation

To use `rustbench`, add it to your `Cargo.toml`:

```toml
[dependencies]
rustbench = "0.1.0"
```

Then, add the macro as a procedural macro dependency:

```toml
[lib]
proc-macro = true
```

## Usage

Simply annotate any function with `#[benchmark]` to measure its execution time.

```rust
use rustbench::benchmark;

#[benchmark]
fn sum() {
    let sum: u64 = (1..=1_000_000).sum();
    println!("Sum: {}", sum);
}

fn main() {
    sum();
}
```

### Output

When `sum` runs, it prints the execution time:

```
Function sum executed in 12.3ms
```

## Features (Planned)
- **Custom Iterations**: Run the function multiple times for better measurement.
- **Comparison Mode**: Compare execution time of two functions.
- **Output Formats**: JSON, CSV, or plain text.
- **Memory Profiling**: Measure memory usage during execution.
- **Integration with Logging**: Store benchmark results in logs.
