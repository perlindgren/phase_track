// phase

use std::f64::consts::PI;

fn main() {
    const SF: usize = 48000; // sample rate
    const F: f64 = 2.0; // frequency
    const T: usize = 1; // time

    let mut left = vec![];
    let mut right = vec![];
    let mut wrapped = vec![];
    let mut unwrapped = vec![];

    let mut old_atan2 = 0.0;
    let mut wrap = 0.0;

    for k in 0..SF * T {
        left.push((k as f64 * 2.0 * PI * F / SF as f64).sin());
        right.push((k as f64 * 2.0 * PI * F / SF as f64).cos());

        let new_atan2 = libm::atan2(left[k], right[k]);
        wrapped.push(new_atan2);
        if old_atan2 - new_atan2 > PI {
            wrap += 1.0;
        }
        old_atan2 = new_atan2;
        unwrapped.push(wrap * 2.0 * PI + new_atan2);
    }

    phase::plot_all(left, right, wrapped, unwrapped.clone(), SF, "phase");

    phase::quality(unwrapped, "octave/phase.csv");
}
