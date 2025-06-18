use clap::Parser;
use image::{ImageBuffer, Rgba};
use raytracer::{camera::Camera, transformations::view_transform, tuple::Tuple, world::World};
use std::fs;
use std::path::Path;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "raytracer-cli")]
#[command(about = "A CLI raytracer for rendering single frames")]
#[command(version = "0.1.0")]
struct Args {
    /// Output filename (PNG format)
    #[arg(short, long, default_value = "output.png")]
    output: String,

    /// Image width in pixels
    #[arg(short, long, default_value = "800")]
    width: usize,

    /// Image height in pixels  
    #[arg(short = 'H', long, default_value = "600")]
    height: usize,

    /// Scene to render (default, test, third)
    #[arg(short, long, default_value = "third")]
    scene: String,

    /// Field of view in degrees
    #[arg(short, long, default_value = "60")]
    fov: f64,

    /// Camera position (x,y,z)
    #[arg(long, value_delimiter = ',', num_args = 3)]
    camera_pos: Option<Vec<f64>>,

    /// Camera look-at point (x,y,z)
    #[arg(long, value_delimiter = ',', num_args = 3)]
    camera_target: Option<Vec<f64>>,

    /// Camera up vector (x,y,z)
    #[arg(long, value_delimiter = ',', num_args = 3)]
    camera_up: Option<Vec<f64>>,
}

fn main() {
    let args = Args::parse();

    println!("Starting raytracer...");
    println!("Resolution: {}x{}", args.width, args.height);
    println!("Scene: {}", args.scene);
    println!("Output: {}", args.output);

    // Create the world based on the scene parameter
    let world = match args.scene.as_str() {
        "default" => World::default_world(),
        "test" => World::test_world(),
        "third" => World::third_world(),
        _ => {
            eprintln!("Unknown scene '{}'. Using 'third' scene.", args.scene);
            World::third_world()
        }
    };

    // Create camera
    let mut camera = Camera::new(args.width, args.height, args.fov.to_radians());

    // Set up camera position and orientation
    let camera_pos = args
        .camera_pos
        .as_ref()
        .map(|pos| {
            if pos.len() == 3 {
                Tuple::point(pos[0], pos[1], pos[2])
            } else {
                eprintln!("Camera position must have exactly 3 values (x,y,z). Using default.");
                Tuple::point(0.0, 1.5, -5.0)
            }
        })
        .unwrap_or_else(|| Tuple::point(0.0, 1.5, -5.0));

    let camera_target = args
        .camera_target
        .as_ref()
        .map(|target| {
            if target.len() == 3 {
                Tuple::point(target[0], target[1], target[2])
            } else {
                eprintln!("Camera target must have exactly 3 values (x,y,z). Using default.");
                Tuple::point(0.0, 1.0, 0.0)
            }
        })
        .unwrap_or_else(|| Tuple::point(0.0, 1.0, 0.0));

    let camera_up = args
        .camera_up
        .as_ref()
        .map(|up| {
            if up.len() == 3 {
                Tuple::vector(up[0], up[1], up[2])
            } else {
                eprintln!("Camera up vector must have exactly 3 values (x,y,z). Using default.");
                Tuple::vector(0.0, 1.0, 0.0)
            }
        })
        .unwrap_or_else(|| Tuple::vector(0.0, 1.0, 0.0));

    println!(
        "Camera position: ({:.2}, {:.2}, {:.2})",
        camera_pos.x, camera_pos.y, camera_pos.z
    );
    println!(
        "Camera target: ({:.2}, {:.2}, {:.2})",
        camera_target.x, camera_target.y, camera_target.z
    );
    println!(
        "Camera up: ({:.2}, {:.2}, {:.2})",
        camera_up.x, camera_up.y, camera_up.z
    );

    camera.set_transform(view_transform(camera_pos, camera_target, camera_up));

    // Render the scene
    println!("Rendering...");
    let start_time = Instant::now();

    let canvas = camera.render(&world);

    let render_time = start_time.elapsed();
    println!("Render completed in {:.2}s", render_time.as_secs_f64());

    // Convert canvas to image buffer
    println!("Converting to image format...");
    let mut img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(args.width as u32, args.height as u32);

    for y in 0..args.height {
        for x in 0..args.width {
            let colour = canvas.pixel_at(x, y);
            let r = (colour.r.clamp(0.0, 1.0) * 255.0) as u8;
            let g = (colour.g.clamp(0.0, 1.0) * 255.0) as u8;
            let b = (colour.b.clamp(0.0, 1.0) * 255.0) as u8;
            let a = 255u8;

            img_buffer.put_pixel(x as u32, y as u32, Rgba([r, g, b, a]));
        }
    }

    // Create output directory if it doesn't exist
    if let Some(parent) = Path::new(&args.output).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Failed to create output directory");
        }
    }

    // Save the image
    println!("Saving image to {}...", args.output);
    img_buffer.save(&args.output).expect("Failed to save image");

    let total_time = start_time.elapsed();
    println!("Total time: {:.2}s", total_time.as_secs_f64());
    println!("Image saved successfully!");
}
