use std::{fs::File, sync::Arc};

use arrow::csv;
use parquet::arrow::ArrowWriter;

use crate::schema::Athorts;


pub struct CsvAthorts {}

impl CsvAthorts {
    // CSVを読み込み、Parquetに変換する
    pub fn csv_to_parquet(csv_path: &str, parquet_path: &str, athorts: &Athorts) {
        // ArrowのStringArrayを作成する
        let schema = Arc::new(athorts.to_arrow_schema());

        // csvファイルを読み込む
        let file = File::open(csv_path).expect("Failed to open CSV file");
        let mut csv = csv::ReaderBuilder::new(Arc::clone(&schema))
            .with_header(true)
            .build(file)
            .expect("Failed to build CSV reader");

        let output = File::create(parquet_path).expect("Failed to create Parquet file");
        let mut writer = ArrowWriter::try_new(output, Arc::clone(&schema), None)
            .expect("Failed to create Parquet writer");
        while let Some(batch) = csv.next() {
            let batch = batch.expect("Failed to read CSV batch");
            writer
                .write(&batch)
                .expect("Failed to write batch to Parquet");
        }

        writer.close().expect("Failed to close Parquet writer");
    }
}
