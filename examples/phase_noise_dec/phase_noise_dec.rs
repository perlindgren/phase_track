// phase

use plotters::prelude::*;
use rand::Rng;
use std::f64::consts::PI;

fn main() {
    let sf = 48000; // sample rate
    let t = 1; // time
    let mut rng = rand::thread_rng();

    let mut left = vec![];
    let mut right = vec![];
    let mut wrapped = vec![];
    let mut unwrapped = vec![];

    let mut old_atan2 = 0.0;
    let mut wrap = 0.0;
    let mut freq = 5.0;
    let delta = freq / sf as f64;

    for k in 0..sf * t {
        left.push((k as f64 * freq * 2.0 * PI / sf as f64).sin() + (rng.gen::<f64>() - 0.5) * 0.50);
        right
            .push((k as f64 * freq * 2.0 * PI / sf as f64).cos() + (rng.gen::<f64>() - 0.5) * 0.50);

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

    // plot left, right
    let root_area =
        SVGBackend::new("examples/phase_noise_dec/left_right.svg", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("left right", ("sans-serif", 40))
        .build_cartesian_2d(0..sf, -2.0..2.0)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new((0..sf).map(|x| (x, left[x])), &BLUE))
        .unwrap();

    ctx.draw_series(LineSeries::new((0..sf).map(|x| (x, right[x])), &RED))
        .unwrap();

    // plot atan2
    let root_area =
        SVGBackend::new("examples/phase_noise_dec/atan2.svg", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("atan2", ("sans-serif", 40))
        .build_cartesian_2d(0..sf, -PI..PI)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new((0..sf).map(|x| (x, wrapped[x])), &RED))
        .unwrap();

    // plot unwrapped
    let root_area =
        SVGBackend::new("examples/phase_noise_dec/unwrapped.svg", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("unwrapped", ("sans-serif", 40))
        .build_cartesian_2d(
            0..sf,
            -PI..(unwrapped
                .clone()
                .into_iter()
                .fold(f64::NEG_INFINITY, f64::max)),
        )
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new((0..sf).map(|x| (x, unwrapped[x])), &RED))
        .unwrap();

    ctx.draw_series(LineSeries::new((0..sf).map(|x| (x, wrapped[x])), &BLUE))
        .unwrap();
}
