use fft_intel::fft::naive::fft;
use num_complex::Complex;
use std::f64::consts::PI;

/// Compare two vectors of Complex numbers with a small tolerance
fn approx_eq(a: &[Complex<f64>], b: &[Complex<f64>], tol: f64) -> bool {
    a.iter()
        .zip(b)
        .all(|(x, y)| (x - y).norm() < tol)
}

#[test]
fn test_fft_delta() {
    // delta function: first sample is 1, rest are 0
    let input: Vec<Complex<f64>> = vec![
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
    ];

    // FFT of a delta function is a constant function (all 1s)
    let expected: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); 8];

    let output = fft(&input);
    assert!(approx_eq(&output, &expected, 1e-6));
}

#[test]
fn test_fft_sine_wave() {
    let n = 8;
    let input: Vec<Complex<f64>> = (0..n)
        .map(|x| Complex::new((2.0 * PI * x as f64 / n as f64).sin(), 0.0))
        .collect();

    let output = fft(&input);

    // We're not asserting exact values, just that the norm makes sense
    let power: f64 = output.iter().map(|x| x.norm_sqr()).sum();
    let expected_power: f64 = input.iter().map(|x| x.norm_sqr()).sum();

    assert!((power - expected_power).abs() < 1e-6);
}
