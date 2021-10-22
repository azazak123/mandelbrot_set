use minifb::*;
use plotters::prelude::*;
use std::fs::File;
use std::{
    ops::Range,
    sync::{Arc, Mutex},
};

// const ZOOM: f64 = 1.0 / 2.0;
const RESOLUTION_Y: usize = 500;
const RESOLUTION_X: usize = RESOLUTION_Y / THREAD_NUMBER;
const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const N: u32 = 1000;
const THREAD_NUMBER: usize = 50;

fn main() {
    let mut window = Window::new(
        "Test - Press ESC to exit",
        WIDTH as usize,
        HEIGHT as usize,
        WindowOptions::default(),
    )
    .expect("Unable to open Window");

    let mut generate = true;
    let mut u32_buffer = vec![0; (WIDTH * HEIGHT) as usize];
    let mut zoom = 1.1;
    let mut x0 = 0.0;
    let mut y0 = 0.0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Space) {}
        if window.is_key_down(Key::Z) {
            zoom += zoom / 10.0;
            generate = true;
        }
        if window.is_key_down(Key::X) {
            zoom -= zoom / 10.0;
            if zoom < 0.0 {
                zoom = 0.0;
            }
            generate = true;
        }
        if window.is_key_down(Key::Right) {
            x0 += 0.5 / zoom;
            generate = true;
        }
        if window.is_key_down(Key::Left) {
            x0 -= 0.5 / zoom;
            generate = true;
        }
        if window.is_key_down(Key::Up) {
            y0 += 0.5 / zoom;
            generate = true;
        }
        if window.is_key_down(Key::Down) {
            y0 -= 0.5 / zoom;
            generate = true;
        }

        if generate {
            draw(zoom, x0, y0).expect("Unable to draw");
            let decoder = png::Decoder::new(File::open("plotters-doc-data/0.png").unwrap());
            let mut reader = decoder.read_info().unwrap();
            let mut buf = vec![0; reader.info().raw_bytes()];
            reader.next_frame(&mut buf).expect("Unable to read picture");
            u32_buffer = buf
                .chunks(3)
                .filter(|v| v.len() == 3)
                .map(|v| {
                    //println!("{:?}", v);
                    ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32
                })
                .collect::<Vec<u32>>();
            window
                .update_with_buffer(&u32_buffer, WIDTH as usize, HEIGHT as usize)
                .expect("Unable to update Window");
            generate = !generate;
        } else {
            window
                .update_with_buffer(&u32_buffer, WIDTH as usize, HEIGHT as usize)
                .expect("Unable to update Window");
        }
    }
}

fn draw(zoom: f64, x0: f64, y0: f64) -> Result<()> {
    let root = BitMapBackend::new("plotters-doc-data/0.png", (WIDTH, HEIGHT)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(
            (x0 - 1f64 / zoom)..(x0 + 1f64 / zoom),
            (y0 - 1f64 / zoom)..(y0 + 1f64 / zoom),
        )
        .unwrap();

    let set = mandelbrot_generate_threading(zoom, x0, y0);
    let coords = set.into_iter();

    chart
        .draw_series(coords.map(|coord| Pixel::new(coord, &BLACK)))
        .unwrap();
    Ok(())
}

fn mandelbrot_generate_threading(zoom: f64, x0: f64, y0: f64) -> Vec<(f64, f64)> {
    let mut handles = Vec::with_capacity(THREAD_NUMBER);
    let set_vectors = Arc::new(Mutex::new(vec![
        Vec::<(f64, f64)>::with_capacity(
            RESOLUTION_X * RESOLUTION_Y
        );
        THREAD_NUMBER
    ]));
    for i in 0..THREAD_NUMBER {
        let d = 2.0 * (1f64 / zoom) / THREAD_NUMBER as f64;
        let x_range =
            (x0 + (-1f64 / zoom) + d * (i as f64))..(x0 + (-1f64 / zoom) + d * ((i + 1) as f64));
        let y_range = (y0 - 1f64 / zoom)..(y0 + 1f64 / zoom);
        let set_vectors = Arc::clone(&set_vectors);
        let handle = std::thread::spawn(move || {
            let mandelbrot_part =
                mandelbrot_generate(x_range, y_range, N + zoom.sqrt().ln() as u32);
            {
                let mut set_vectors = set_vectors.lock().unwrap();
                set_vectors[i] = mandelbrot_part;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let mut set = Vec::with_capacity(THREAD_NUMBER * RESOLUTION_X * RESOLUTION_Y);
    for i in &mut *set_vectors.lock().unwrap() {
        set.append(i);
    }
    set
}

fn mandelbrot_generate(x_range: Range<f64>, y_range: Range<f64>, n_var: u32) -> Vec<(f64, f64)> {
    let dx = x_range.end - x_range.start;
    let dy = y_range.end - y_range.start;
    let mut result: Vec<(f64, f64)> = Vec::with_capacity(RESOLUTION_X * RESOLUTION_Y);
    let mut y = y_range.start;
    for _i in 0..RESOLUTION_Y {
        let mut x = x_range.start;
        for _j in 0..RESOLUTION_X {
            let mut x_current = 0f64;
            let mut y_current = 0f64;
            let p = x;
            let q = y;
            let mut n = 0;
            while n < n_var && (x_current * x_current - y_current * y_current) < 2.0 {
                let x_prev = x_current;
                let y_prev = y_current;
                x_current = x_prev * x_prev - y_prev * y_prev + p;
                y_current = 2.0 * x_prev * y_prev + q;
                n += 1;
            }
            if n == n_var {
                result.push((p, q));
            }
            x += dx / RESOLUTION_X as f64;
        }
        y += dy / RESOLUTION_Y as f64;
    }
    println!("{:?} {:?}", x_range, y_range);
    result
}
