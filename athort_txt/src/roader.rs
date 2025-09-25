use std::{env, fs::File, sync::Arc};

use arrow::{csv, datatypes::{DataType, Field, Schema}};
use parquet::arrow::ArrowWriter;

// Arrowのスキーマ
pub fn make_schema() -> Schema {
    Schema::new(vec![
        Field::new("id", DataType::UInt64, false),
        Field::new("title", DataType::Utf8, false),
        Field::new("content", DataType::Utf8, false),
        Field::new("page", DataType::UInt64, false),
    ])
}


pub fn main() {
    // ArrowのStringArrayを作成する
    let schema = Arc::new(make_schema());

    // csvファイルを読み込む
    let file_path = env::var_os("HOME").expect("HOME environment variable not set").to_str().unwrap().to_string() + "/programs/rust/athort/data";
    let file = File::open(format!("{}/sample.csv", file_path)).expect("Failed to open CSV file");
    let mut csv = csv::ReaderBuilder::new(Arc::clone(&schema)).with_header(true).build(file).expect("Failed to build CSV reader");

    let output = File::create(format!("{}/output.parquet", file_path)).expect("Failed to create Parquet file");
    let mut writer = ArrowWriter::try_new(output, Arc::clone(&schema), None).expect("Failed to create Parquet writer");
    while let Some(batch) = csv.next() {
        let batch = batch.expect("Failed to read CSV batch");
        writer.write(&batch).expect("Failed to write batch to Parquet");
    }

    writer.close().expect("Failed to close Parquet writer");
}