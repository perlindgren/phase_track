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

#[cfg(test)]
mod test {
    use csv::Reader;
    use std::error::Error;

    #[test]
    fn example() -> Result<(), Box<dyn Error>> {
        let mut rdr = Reader::from_path("octave/data.csv")?;
        for result in rdr.records() {
            let record = result?;
            println!("{:?}", record);
        }
        Ok(())
    }

    #[test]
    fn example2() {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path("octave/data.csv")
            .unwrap();

        let mut v = vec![];
        for result in rdr.deserialize() {
            let record: Vec<f64> = result.unwrap();
            v = record
        }
        println!("v {:?}", v);
    }
}
