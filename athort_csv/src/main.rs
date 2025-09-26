use std::{env, fs::File, sync::Arc};

use arrow::{csv, datatypes::{DataType, Field, Schema}};
use athort_common::schema::{AthortSchema, Athorts};
use athort_csv::athorts;
use parquet::arrow::ArrowWriter;


pub fn main() {
    // スキーマを作成
    let athorts = Athorts::from_vec(vec![
        AthortSchema::new(String::from("id"), DataType::UInt64, false),
        AthortSchema::new(String::from("title"), DataType::Utf8, false),
        AthortSchema::new(String::from("content"), DataType::Utf8, false),
        AthortSchema::new(String::from("page"), DataType::UInt64, false),
    ]);

    // csvファイルを読み込む
    let file_path = env::var_os("HOME").expect("HOME environment variable not set").to_str().unwrap().to_string() + "/programs/rust/athort/data";
    athorts::CsvAthorts::csv_to_parquet(
        &(file_path.clone() + "/sample.csv"),
        &(file_path + "/sample.parquet"),
        &athorts,
    );
}

fn easy_main() {
    // Athortのスキーマを作成
    let athort_schemas = vec![
        AthortSchema::new(String::from("id"), DataType::UInt64, false),
        AthortSchema::new(String::from("title"), DataType::Utf8, false),
        AthortSchema::new(String::from("content"), DataType::Utf8, false),
        AthortSchema::new(String::from("page"), DataType::UInt64, false),
    ];

    // スキーマからAthortsを作成
    let athorts = Athorts::from_vec(athort_schemas);

    // csvファイル名を指定
    let file_path = env::var_os("HOME").expect("HOME environment variable not set").to_str().unwrap().to_string() + "/programs/rust/athort/data";
    let file = File::open(format!("{}/sample.csv", file_path)).expect("Failed to open CSV file");


}