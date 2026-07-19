use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => {
            let mut prev = 1;
            let mut curr = 1;
            for _ in 2..=n {
                let next = prev + curr;
                prev = curr;
                curr = next;
            }
            curr
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci Comparison");

    group.bench_with_input("Recursive", &20, |b, n| b.iter(|| fibonacci(black_box(*n))));

    group.bench_with_input("Iterative", &20, |b, n| {
        b.iter(|| fibonacci_iterative(black_box(*n)))
    });
    // |b| b.iter(|| fibonacci(black_box(20))) 说：反复运行 fibonacci(20)，测它的执行时间
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
