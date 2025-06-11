mod fft;

use fft::naive::fft;
use num_complex::Complex;
use std::f64::consts::PI;

fn main() {
    // Example input: 8-point sine wave (zero-padded if needed)
    let input: Vec<Complex<f64>> = (0..8)
        .map(|x| Complex::new((2.0 * PI * x as f64 / 8.0).sin(), 0.0))
        .collect();

    println!("Input:");
    for x in &input {
        println!("{:.3}", x);
    }

    let output = fft(&input);

    println!("\nFFT Output:");
    for x in &output {
        println!("{:.3}", x);
    }
}
