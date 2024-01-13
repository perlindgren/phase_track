// phase

use rand::Rng;
use std::f64::consts::PI;

fn main() {
    const SF: usize = 48000; // sample rate
    const T: usize = 1; // time

    let mut rng = rand::thread_rng();

    let mut left = vec![];
    let mut right = vec![];
    let mut wrapped = vec![];
    let mut unwrapped = vec![];

    let mut old_atan2 = 0.0;
    let mut wrap = 0.0;
    let mut freq = 5.0;
    let delta = freq / SF as f64;

    for k in 0..SF * T {
        left.push((k as f64 * freq * 2.0 * PI / SF as f64).sin() + (rng.gen::<f64>() - 0.5) * 0.50);
        right
            .push((k as f64 * freq * 2.0 * PI / SF as f64).cos() + (rng.gen::<f64>() - 0.5) * 0.50);

        freq -= delta;

        let new_atan2 = libm::atan2(left[k], right[k]);
        wrapped.push(new_atan2);
        if new_atan2 - old_atan2 > PI {
            wrap -= 1.0;
        } else if old_atan2 - new_atan2 > PI {
            wrap += 1.0;
        }
        old_atan2 = new_atan2;
        unwrapped.push(wrap * 2.0 * PI + new_atan2);
    }

    phase::plot_all(left, right, wrapped, unwrapped, SF, "phase_noise_dec");
}
