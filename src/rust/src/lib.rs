use std::io::BufWriter;
use std::{fs::File, str::from_utf8};

use extendr_api::prelude::*;
use geojson::{Feature, FeatureCollection, Value};

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}
/// Convert `points.csv` to points_rust.geojson.
/// @export
#[extendr]
pub fn csv_to_geojson_rust() {
    let mut points = Vec::new();
    let mut reader = csv::Reader::from_path("points.csv").unwrap();
    let mut record = csv::ByteRecord::new();
    while reader.read_byte_record(&mut record).unwrap() {
        points.push(Feature::from(Value::Point(vec![
            from_utf8(record.get(0).unwrap())
                .unwrap()
                .parse::<f64>()
                .unwrap(),
            from_utf8(record.get(1).unwrap())
                .unwrap()
                .parse::<f64>()
                .unwrap(),
        ])));
    }
    let fc = FeatureCollection {
        bbox: None,
        features: points,
        foreign_members: None,
    };
    let f = File::create("points_rust.geojson").unwrap();
    let f = BufWriter::new(f);
    serde_json::to_writer_pretty(f, &fc).unwrap();
}
// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod georustr;
    fn hello_world;
    fn csv_to_geojson_rust;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_to_geojson_works() {
        csv_to_geojson_rust();
    }
}
