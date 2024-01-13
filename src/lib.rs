// lib
use plotters::prelude::*;
use std::error::Error;
use std::f64::consts::PI;

fn read_file(path: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)
        .unwrap();

    let result = rdr
        .deserialize()
        .next()
        .unwrap_or_else(|| panic!("parse failure in {}", path));
    Ok(result?)
}

pub fn quality(rust: Vec<f64>, path: &str) {
    let octave = read_file(path).unwrap();
    assert_eq!(rust.len(), octave.len());

    let (acc, mse) = octave
        .iter()
        .zip(rust.iter())
        .fold((0.0, 0.0), |(acc, mse), (r, w)| {
            (acc + (r - w).abs(), mse + (r - w).powi(2))
        });

    println!("accumulated error {}", acc);
    println!("min square error  {}", mse / rust.len() as f64);
}

// plot left, right
pub fn plot_left_right(left: Vec<f64>, right: Vec<f64>, sf: usize, path: &str) {
    let path = &format!("examples/{}/left_right.svg", path);
    let root_area = SVGBackend::new(path, (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("left right", ("sans-serif", 40))
        .build_cartesian_2d(0..sf, -1.0..1.0)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new((0..sf).map(|x| (x, left[x])), &BLUE))
        .unwrap();

    ctx.draw_series(LineSeries::new((0..sf).map(|x| (x, right[x])), &RED))
        .unwrap();
}

// plot wrapped, atan2
pub fn plot_wrapped(wrapped: Vec<f64>, sf: usize, path: &str) {
    let path = &format!("examples/{}/wrapped.svg", path);
    let root_area = SVGBackend::new(path, (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("wrapped, atan2", ("sans-serif", 40))
        .build_cartesian_2d(0..sf, -PI..PI)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new((0..sf).map(|x| (x, wrapped[x])), &RED))
        .unwrap();
}

// plot unwrapped
pub fn plot_unwrapped(unwrapped: Vec<f64>, wrapped: Vec<f64>, sf: usize, path: &str) {
    let path = &format!("examples/{}/unwrapped.svg", path);
    let root_area = SVGBackend::new(path, (600, 400)).into_drawing_area();
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

// plot all
pub fn plot_all(
    left: Vec<f64>,
    right: Vec<f64>,
    wrapped: Vec<f64>,
    unwrapped: Vec<f64>,
    sf: usize,
    path: &str,
) {
    plot_left_right(left, right, sf, path);
    plot_wrapped(wrapped.clone(), sf, path);
    plot_unwrapped(unwrapped, wrapped, sf, path);
}
