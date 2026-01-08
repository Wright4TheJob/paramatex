use std::collections::HashMap;
use std::fs;
use calamine::{HeaderRow, open_workbook, Error,Ods, Xlsx, Reader, RangeDeserializerBuilder};
use serde::Deserialize;

fn main() {

    let path = format!("{}/example-parameters.ods", env!("CARGO_MANIFEST_DIR"));
    let mut excel: Ods<_> = open_workbook(path).unwrap();

    let mut text_string = fs::read_to_string("example-source.tex").expect("Unable to read file");
let sheet1 = excel
    .with_header_row(HeaderRow::Row(3))
    .worksheet_range("Sheet1")
    .unwrap();

let sheet1 = excel
    .with_header_row(HeaderRow::Row(1))
    .worksheet_range("Sheet1")
    .unwrap();
    let mut values: HashMap<String, String> = HashMap::new();
    for row in 0..sheet1.height() {
        let key = sheet1.get_value((row as u32, 1 as u32)); 
        let value = sheet1.get_value((row as u32, 2 as u32));
        if key.is_some() && value.is_some() {
            values.insert(
                key.unwrap().to_string(),
                value.unwrap().to_string(),
            );
        } 
    }
    for (key, value) in values.iter() {
        text_string = text_string.replace(key, value);
    }
    fs::write("example-output.tex", text_string).expect("Unable to write file");
//    #[serde(deserialize_with = "deserialize_as_f64_or_none")]
//    value: String,
//}

//fn example() -> Result<HashMap<String, String>, Error> {
//    let path = format!("{}/example-parameters.ods", env!("CARGO_MANIFEST_DIR"));
//    let mut workbook: Ods<_> = open_workbook(path)?;
//    let range = workbook.worksheet_range("Sheet1")?;
//
 //   let mut iter = RangeDeserializerBuilder::new().from_range(&range)?;
//
//    let iter_records = RangeDeserializerBuilder::with_headers(&["Key", "Value"]).from_range(&range)?;
//
 //   let mut key_values = HashMap::new();
//    key_values.insert("One".to_string(), "Two".to_string());

  //  for result in iter_records {
//        let record: Record = result?;
//        println!("{:?}", result);
        // println!("metric={:?}, value={:?}", record.metric, record.value);
        //key_values.push((record.Key, record.Value))
//    }
//    return Ok(key_values) 
//}

//fn main() {
 //   let output = example();
 //   println!("{:?}",output);
    
}
