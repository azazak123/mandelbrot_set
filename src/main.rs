use std::{
    ops::Range,
    sync::{Arc, Mutex},
};

use plotters::prelude::*;

const ZOOM: f32 = 1.0 / 2.0;
const RESOLUTION_X: usize = 1000;
const RESOLUTION_Y: usize = 1000;
const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const N: u32 = 5000;
const THREAD_NUMBER: usize = 1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/0.png", (WIDTH, HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d((-1f32 / ZOOM)..(1f32 / ZOOM), (-1f32 / ZOOM)..(1f32 / ZOOM))?;

    let mut handles = Vec::with_capacity(THREAD_NUMBER);
    let set_vectors = Arc::new(Mutex::new(vec![
        Vec::<(f32, f32)>::with_capacity(
            RESOLUTION_X * RESOLUTION_Y
        );
        THREAD_NUMBER
    ]));
    for i in 0..THREAD_NUMBER {
        let d = 4f32 / THREAD_NUMBER as f32;
        let x_range = (-2f32 + d * (i as f32))..(-2f32 + d * ((i + 1) as f32));
        let y_range = -2f32..2f32;
        let set_vec = Arc::clone(&set_vectors);
        let handle = std::thread::spawn(move || {
            let mut set = set_vec.lock().unwrap();
            set[i] = mandelbrot_generate(x_range, y_range);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let mut set = Vec::with_capacity(THREAD_NUMBER * RESOLUTION_X * RESOLUTION_Y);
    // println!("{:?}", set_vectors[0].lock().unwrap().len());
    for i in &mut *set_vectors.lock().unwrap() {
        set.append(i);
    }
    let coords = set.into_iter();

    chart.draw_series(coords.map(|coord| Pixel::new(coord, &BLACK)))?;

    Ok(())
}

fn mandelbrot_generate(x_range: Range<f32>, y_range: Range<f32>) -> Vec<(f32, f32)> {
    let dx = x_range.end - x_range.start;
    let dy = y_range.end - y_range.start;
    let mut arr = vec![(0f32, 0f32); RESOLUTION_X * RESOLUTION_Y];
    let mut y = y_range.start;
    for i in 0..RESOLUTION_Y {
        let mut x = x_range.start;
        for j in 0..RESOLUTION_X {
            arr[i * RESOLUTION_X + j] = (x, y);
            x += dx / RESOLUTION_X as f32;
        }
        y += dy / RESOLUTION_Y as f32;
    }

    let mut result: Vec<(f32, f32)> = Vec::with_capacity(RESOLUTION_X * RESOLUTION_Y);
    for i in 0..RESOLUTION_Y {
        for j in 0..RESOLUTION_X {
            let mut x_current = 0f32;
            let mut y_current = 0f32;
            let (p, q) = arr[i * RESOLUTION_X + j];
            let mut n = 0;
            while n < N && (x_current * x_current - y_current * y_current) < 10f32.powf(2.0) {
                let x_prev = x_current;
                let y_prev = y_current;
                x_current = x_prev * x_prev - y_prev * y_prev + p;
                y_current = 2.0 * x_prev * y_prev + q;
                n += 1;
            }
            if n == N {
                result.push((p, q));
            }
            // println!("{}", i as f32 / RESOLUTION as f32);
        }
    }
    result
}
