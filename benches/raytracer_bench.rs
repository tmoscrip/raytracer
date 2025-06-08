use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raytracer::render_context::RenderContext;

fn benchmark_render_small(c: &mut Criterion) {
    let mut ctx = RenderContext::new(50, 50);

    c.bench_function("render_50x50", |b| {
        b.iter(|| {
            ctx.render(black_box(0.016)); // 60 FPS frame time
        })
    });
}

fn benchmark_render_medium(c: &mut Criterion) {
    let mut ctx = RenderContext::new(100, 100);

    c.bench_function("render_100x100", |b| {
        b.iter(|| {
            ctx.render(black_box(0.016)); // 60 FPS frame time
        })
    });
}

fn benchmark_render_large(c: &mut Criterion) {
    let mut ctx = RenderContext::new(200, 200);

    c.bench_function("render_200x200", |b| {
        b.iter(|| {
            ctx.render(black_box(0.016)); // 60 FPS frame time
        })
    });
}

criterion_group!(
    benches,
    benchmark_render_small,
    benchmark_render_medium,
    benchmark_render_large
);
criterion_main!(benches);
