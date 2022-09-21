use fluvio_smartmodule::{smartmodule, Record, RecordData, Result};
use serde_json::Value;

#[smartmodule(map, params)]
pub fn map(record: &Record, _params: &SmartModuleOpt) -> Result<(Option<RecordData>, RecordData)> {

    let key = RecordData::from(String::from("JSON")).into();
    // the key for the fluvio key value pairs -- setting it to report that it is a JSON key

    let input: Value = serde_json::from_slice(record.value.as_ref())?;
    // pulling the input value from the record
    let fact = &input["fact"];
    // getting just the fact portion of the JSON object

    let output = serde_json::to_string(fact)?;
    // string-ifying the JSON content
    Ok((key, output.into()))
    //returning the new key-value pair
}

#[derive(fluvio_smartmodule::SmartOpt, Default)]
pub struct SmartModuleOpt;
