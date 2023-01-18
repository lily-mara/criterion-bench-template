use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    {% if async %}
    c.bench_function("async_do_something", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap()).iter(|| async_do_something());
    });
    {% else %}
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    {% endif %}
}

{% if async %}
async fn async_do_something() {
    tokio::time::sleep(std::time::Duration::from_millis(2)).await;
}
{% else %}
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}
{% endif %}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
