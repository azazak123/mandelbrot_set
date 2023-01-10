use std::{
    ops::Range,
    sync::{Arc, Mutex},
};

const RESOLUTION_Y: usize = 400;
const RESOLUTION_X: usize = RESOLUTION_Y / THREAD_NUMBER;
const N: u32 = 1000;
const THREAD_NUMBER: usize = 50;

pub(crate) fn mandelbrot_generate_threading(zoom: f64, x0: f64, y0: f64) -> Vec<(f64, f64)> {
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

    result
}
