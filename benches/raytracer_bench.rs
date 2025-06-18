use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image::{ImageBuffer, Rgba};
use raytracer::render_context::RenderContext;
use std::fs;
use std::time::Duration;

fn save_render_to_png(ctx: &RenderContext, filename: &str) {
    let width = ctx.get_width();
    let height = ctx.get_height();
    let buffer_ptr = ctx.get_image_buffer_pointer();
    let buffer_len = (width * height * 4) as usize;

    // Create a safe slice from the raw pointer
    let buffer_slice = unsafe { std::slice::from_raw_parts(buffer_ptr, buffer_len) };

    // Create an ImageBuffer from the RGBA data
    let img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, buffer_slice.to_vec())
            .expect("Failed to create image buffer");

    // Ensure output directory exists
    fs::create_dir_all("benchmark_output").ok();

    // Save as PNG
    let filepath = format!("benchmark_output/{}", filename);
    img_buffer.save(&filepath).expect("Failed to save PNG");
    println!("Saved render to {}", filepath);
}

fn benchmark_render_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("small_renders");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(10);

    let mut ctx = RenderContext::new(50, 50);

    group.bench_function("render_50x50", |b| {
        b.iter(|| {
            ctx.render(black_box(0.016)); // 60 FPS frame time
        })
    });

    group.finish();

    // Save a sample render after benchmarking
    let mut ctx = RenderContext::new(50, 50);
    ctx.render(0.016);
    save_render_to_png(&ctx, "render_50x50_sample.png");
}

fn benchmark_render_medium(c: &mut Criterion) {
    let mut group = c.benchmark_group("medium_renders");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(10);

    let mut ctx = RenderContext::new(100, 100);

    group.bench_function("render_100x100", |b| {
        b.iter(|| {
            ctx.render(black_box(0.016)); // 60 FPS frame time
        })
    });

    group.finish();

    // Save a sample render after benchmarking
    let mut ctx = RenderContext::new(100, 100);
    ctx.render(0.016);
    save_render_to_png(&ctx, "render_100x100_sample.png");
}

fn benchmark_render_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_renders");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(10);

    let mut ctx = RenderContext::new(200, 200);

    group.bench_function("render_200x200", |b| {
        b.iter(|| {
            ctx.render(black_box(0.016)); // 60 FPS frame time
        })
    });

    group.finish();

    // Save a sample render after benchmarking
    let mut ctx = RenderContext::new(200, 200);
    ctx.render(0.016);
    save_render_to_png(&ctx, "render_200x200_sample.png");
}

criterion_group!(
    benches,
    benchmark_render_small,
    benchmark_render_medium,
    benchmark_render_large,
);
criterion_main!(benches);
