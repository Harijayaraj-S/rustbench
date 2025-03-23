# Rustbench

`rustbench` is a lightweight Rust procedural macro for benchmarking function execution time.

## Installation

To use `rustbench`, add it to your `Cargo.toml`:

```toml
[dependencies]
rustbench = "0.1.1"
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
    fn example_sum() {
        let _: u64 = (1..=1_000_000).sum();
    }

    #[benchmark(50)]
    fn example_sum_iteration() {
        let _: u64 = (1..=1_000_000).sum();
    }

    fn main() {
        example_sum_iteration();
        example_sum();
    }
```

### Output
```
Function 'example_sum' executed in 7053000 ns

Iteration took: 11.716916ms
Iteration took: 10.26025ms
Iteration took: 9.13725ms
Iteration took: 8.560416ms
Iteration took: 7.905875ms
Function 'example_sum_iteration' executed 5 times. Avg time: 9516141 ns

```

## Features (Planned)
- **Comparison Mode**: Compare execution time of two functions.
- **Output Formats**: JSON, CSV, or plain text.
- **Memory Profiling**: Measure memory usage during execution.
- **Integration with Logging**: Store benchmark results in logs.
