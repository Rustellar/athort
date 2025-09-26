use arrow::datatypes::{DataType};

// Athortが提供するスキーマ
pub struct AthortSchema {
    data_title: String,
    data_type: DataType,
    is_nullable: bool,
}

// AthortSchemaの集合
pub struct Athorts {
    schemas: Vec<AthortSchema>,
}

impl Athorts {
    pub fn new() -> Self {
        Self { schemas: Vec::new() }
    }

    pub fn add_schema(&mut self, schema: AthortSchema) {
        self.schemas.push(schema);
    }

    pub fn from_vec(schemas: Vec<AthortSchema>) -> Self {
        Self { schemas }
    }

    // ArrowのSchemaに変換するメソッド
    pub fn to_arrow_schema(&self) -> arrow::datatypes::Schema {
        let fields: Vec<arrow::datatypes::Field> = self.schemas.iter().map(|s| s.to_field()).collect();
        arrow::datatypes::Schema::new(fields)
    }
}

impl AthortSchema {
    pub fn new(data_title: String, data_type: DataType, is_nullable: bool) -> Self {
        Self {
            data_title,
            data_type,
            is_nullable,
        }
    }

    // ArrowのFieldに変換するメソッド
    pub fn to_field(&self) -> arrow::datatypes::Field {
        arrow::datatypes::Field::new(&self.data_title, self.data_type.clone(), self.is_nullable)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use arrow::datatypes::{Field, Schema};

    #[test]
    fn test_athort_schema_to_field() {
        let athort_id = AthortSchema::new(String::from("id"), DataType::UInt64, false);
        let athort_title = AthortSchema::new(String::from("title"), DataType::Utf8, false);
        let athort_content = AthortSchema::new(String::from("content"), DataType::Utf8, false);
        let athort_page = AthortSchema::new(String::from("page"), DataType::UInt64, false);

        let athort_to_arrow = Athorts::from_vec(vec![
            athort_id,
            athort_title,
            athort_content,
            athort_page,
        ]);

        let athort_to_arrow = athort_to_arrow.to_arrow_schema();

        let arrow_schema = Schema::new(vec![
        Field::new("id", DataType::UInt64, false),
        Field::new("title", DataType::Utf8, false),
        Field::new("content", DataType::Utf8, false),
        Field::new("page", DataType::UInt64, false),
        ]);

        assert_eq!(athort_to_arrow, arrow_schema);
    }
}