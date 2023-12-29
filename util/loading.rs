use arrow::{record_batch::RecordBatch, csv::ReaderBuilder};
use arrow_csv::infer_schema_from_files;
use std::{fs::File, sync::Arc, io::BufReader};
use regex::Regex;

pub fn read_data(file_path: String) -> Result<RecordBatch, String> {
    let schema = infer_schema_from_files(&[file_path.clone()], b',', Some(1000), true).unwrap();
    let file = File::open(file_path).unwrap();
    let file_reader = BufReader::new(file);
    let mut csv = ReaderBuilder::new(Arc::new(schema))
        .with_header(true)
        .with_null_regex(Regex::new(r"NA|^$").unwrap())
        .with_batch_size(2500)
        .build(file_reader)
        .unwrap();
    let batch = csv.next().unwrap();
    match batch {
        Ok(b) => Ok(b),
        Err(m) => Err(m.to_string()),
    }
}