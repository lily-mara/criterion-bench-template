# Criterion Benchmark Template

This is a
[cargo-generate](https://cargo-generate.github.io/cargo-generate/index.html)
template for quickly creating benchmarks using the
[Criterion](https://bheisler.github.io/criterion.rs/book/criterion_rs.html)
benchmarking framework.

## Usage

```
$ cargo generate lily-mara/criterion-bench-template
```

The defaults will give you a `benches/bench.rs` file that you can open and edit to suit your needs.

## Async support

By setting the `async` property to `true` when you're generating your project,
criterion will be pre-configured for [async
benchmarks](https://bheisler.github.io/criterion.rs/book/user_guide/benchmarking_async.html)
with [Tokio](https://docs.rs/tokio/latest/tokio/) support.
