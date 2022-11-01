use std::io::{prelude::*, Error};
use std::{fs::File, io, path::Path};

fn file_as_str(path_string: &str) -> Result<String, Error> {
    let path = Path::new(path_string);
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_string_to_file(data: String, path_str: &str) -> Result<File, io::Error> {
    let mut file = File::create(path_str)?;
    file.write_all(data.as_bytes())?;
    Ok(file)
}

fn json_compact_printer_transcode(input: &String) {
    // A JSON deserializer. You can use any Serde Deserializer here.
    let mut deserializer = serde_json::Deserializer::from_str(input.as_str());

    // A compacted JSON serializer. You can use any Serde Serializer here.
    let mut serializer = serde_json::Serializer::new(io::stdout());

    // Prints `{"a boolean":true,"an array":[3,2,1]}` to stdout.
    // This line works with any self-describing Deserializer and any Serializer.
    serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();
}

fn json_to_toml(input: &String) -> Result<String, Error> {
    // A JSON deserializer. You can use any Serde Deserializer here.
    let mut deserializer = serde_json::Deserializer::from_str(input.as_str());

    // A compacted JSON serializer. You can use any Serde Serializer here.
    let mut output = String::new();
    let mut serializer = toml::Serializer::new(&mut output);

    // Prints `{"a boolean":true,"an array":[3,2,1]}` to stdout.
    // This line works with any self-describing Deserializer and any Serializer.
    serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();

    Ok(output)
}
// use serde::Deserialize;
// #[derive(Deserialize)]
// struct ModuleFive {
//     merchant_id: String,
//     store_number: String,
//     street: String,
//     city: String,
//     state: String,
//     zip: String,
// }

fn _toml_to_json(input: &String) -> Result<String, Error> {
    //let mut deserializer: ModuleFive = toml::from_str(input.as_str()).unwrap();
    let mut deserializer = toml::Deserializer::new(input.as_str());

    let mut output = String::new();
    let mut serializer = serde_json::Serializer::new(&mut output);

    serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();

    Ok(output)
}

#[allow(dead_code)]
fn toml_to_json() {}

fn main() -> Result<(), Error> {
    let json_str = file_as_str("json/application-settings-rendered.json")?;
    println!("JSON_STR: {:#?}", &json_str);

    let toml_str = file_as_str("toml/default.toml");
    println!("TOML_STR: {:#?}", toml_str);

    println!("-=> json_compact_printer_transcode");
    json_compact_printer_transcode(&json_str);

    println!("-=> json_to_toml");
    let toml_str = json_to_toml(&json_str)?;
    println!("{:#?}", toml_str);

    println!("-=> write_string_to_file(toml_str, toml_from_json.toml)");
    let f = write_string_to_file(toml_str, "toml_from_json.toml");
    println!("{:#?}", f);

    Ok(())
}
