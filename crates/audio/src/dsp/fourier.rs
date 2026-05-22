use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    pub fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }
    pub fn magnitude(&self) -> f32 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
}

pub fn dft(samples: &[f32]) -> Vec<Complex> {
    let n = samples.len();
    let mut spectrum = vec![Complex::zero(); n];

    for k in 0..n {
        let mut sum_re = 0.0;
        let mut sum_im = 0.0;

        for n_idx in 0..n {
            let angle = (2.0 * PI * (k as f32) * (n_idx as f32)) / (n as f32);

            sum_re += samples[n_idx] * angle.cos();
            sum_im -= samples[n_idx] * angle.sin();
        }

        spectrum[k] = Complex {
            re: sum_re,
            im: sum_im,
        };
    }

    return spectrum;
}
