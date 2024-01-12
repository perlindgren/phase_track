// lib

use std::error::Error;

pub fn read_file(path: &str) -> Result<Vec<f64>, Box<dyn Error>> {
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
