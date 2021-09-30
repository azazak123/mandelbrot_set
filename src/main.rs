use std::{
    ops::Range,
    sync::{Arc, Mutex},
};

use piston_window::*;
use plotters::prelude::*;
use plotters_piston::{draw_piston_window, PistonBackend};

const RESOLUTION_Y: usize = 1000;
const RESOLUTION_X: usize = RESOLUTION_Y / THREAD_NUMBER;
const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const N: u32 = 3000;
const THREAD_NUMBER: usize = 250;
const FPS: u64 = 10;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Real Time CPU Usage", [WIDTH - 200, HEIGHT - 200])
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap();
    window.set_max_fps(FPS);

    let mut ZOOM: f32 = 1.0;
    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut generate = true;
    loop {
        if let Some(e) = events.next(&mut window) {
            if let Some(Button::Keyboard(key)) = e.press_args() {
                match key {
                    Key::C => {
                        generate = true;
                    }
                    Key::A => {
                        return;
                    }
                    Key::Space => {
                        println!("click");
                    }
                    Key::Plus => {
                        ZOOM += 1.0 / 10.0;
                        generate = true;
                    }
                    Key::Minus => {
                        ZOOM -= 1.0 / 10.0;
                        if ZOOM == 0.0 {
                            ZOOM = 1.0 / 10.0
                        }
                        generate = true;
                    }
                    _ => {}
                }
            }
        }
        if !generate {
            continue;
        }
        if let Some(_) = draw_piston_window(&mut window, |b| {
            println!("generate");
            let root = b.into_drawing_area();
            root.fill(&WHITE)?;
            let mut chart = ChartBuilder::on(&root)
                .build_cartesian_2d((-1f32 / ZOOM)..(1f32 / ZOOM), (-1f32 / ZOOM)..(1f32 / ZOOM))?;

            let set = mandelbrot_generate_threading(ZOOM);
            let coords = set.into_iter();

            chart.draw_series(coords.map(|coord| Pixel::new(coord, &BLACK)))?;
            generate = false;
            Ok(())
        }) {}
    }
}

fn mandelbrot_generate_threading(ZOOM: f32) -> Vec<(f32, f32)> {
    let mut handles = Vec::with_capacity(THREAD_NUMBER);
    let set_vectors = Arc::new(Mutex::new(vec![
        Vec::<(f32, f32)>::with_capacity(
            RESOLUTION_X * RESOLUTION_Y
        );
        THREAD_NUMBER
    ]));
    for i in 0..THREAD_NUMBER {
        let d = 2.0 * (1f32 / ZOOM) / THREAD_NUMBER as f32;
        let x_range = ((-1f32 / ZOOM) + d * (i as f32))..((-1f32 / ZOOM) + d * ((i + 1) as f32));
        let y_range = (-1f32 / ZOOM)..(1f32 / ZOOM);
        let set_vectors = Arc::clone(&set_vectors);
        let handle = std::thread::spawn(move || {
            let mandelbrot_part = mandelbrot_generate(x_range, y_range);
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

fn mandelbrot_generate(x_range: Range<f32>, y_range: Range<f32>) -> Vec<(f32, f32)> {
    let dx = x_range.end - x_range.start;
    let dy = y_range.end - y_range.start;
    let mut result: Vec<(f32, f32)> = Vec::with_capacity(RESOLUTION_X * RESOLUTION_Y);
    let mut y = y_range.start;
    for i in 0..RESOLUTION_Y {
        let mut x = x_range.start;
        for j in 0..RESOLUTION_X {
            let mut x_current = 0f32;
            let mut y_current = 0f32;
            let p = x;
            let q = y;
            let mut n = 0;
            while n < N && (x_current * x_current - y_current * y_current) < 2.0 {
                let x_prev = x_current;
                let y_prev = y_current;
                x_current = x_prev * x_prev - y_prev * y_prev + p;
                y_current = 2.0 * x_prev * y_prev + q;
                n += 1;
            }
            if n == N {
                result.push((p, q));
            }
            x += dx / RESOLUTION_X as f32;
        }
        y += dy / RESOLUTION_Y as f32;
    }
    result
}
