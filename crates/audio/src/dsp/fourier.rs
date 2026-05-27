use std::{f32::consts::PI, vec};

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

pub fn idft(freq_samples: &Vec<Complex>) -> Vec<f32> 
{
    let n = freq_samples.len();
    let mut reconstructed_freq: Vec<f32> = vec![0.0; n];

    for k in 0..n {
        let mut sum_real = 0.0;

        for n_idx in 0..n {
            let angle = (2.0 * PI * (k as f32) * (n_idx as f32)) / (n as f32);

            sum_real += (freq_samples[n_idx].re * angle.cos()) - (freq_samples[n_idx].im * angle.sin());
        }

        reconstructed_freq[k] = sum_real / (n as f32);
    }

    return reconstructed_freq;
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
