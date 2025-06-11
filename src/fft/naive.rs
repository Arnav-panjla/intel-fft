use num_complex::Complex;
use std::f64::consts::PI;

pub fn fft(input: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = input.len();
    if n <= 1 {
        return input.to_vec();
    }

    if n % 2 != 0 {
        panic!("Input size must be a power of 2");
    }

    // Split into even and odd parts
    let even = fft(&input.iter().step_by(2).cloned().collect::<Vec<_>>());
    let odd = fft(&input.iter().skip(1).step_by(2).cloned().collect::<Vec<_>>());

    let mut result = vec![Complex::new(0.0, 0.0); n];
    for k in 0..n / 2 {
        let twiddle = Complex::from_polar(
            1.0,
            -2.0 * PI * k as f64 / n as f64,
        );
        result[k] = even[k] + twiddle * odd[k];
        result[k + n / 2] = even[k] - twiddle * odd[k];
    }

    result
}
