use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Read, BufWriter, Write};


pub mod model_schema;
pub mod parse;

fn main() {
    let urls_file = File::open(env::args().nth(1).unwrap()).unwrap();
    let urls_reader = BufReader::new(urls_file);

    let model_file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .truncate(true)
        .create(true)
        .open("./models.json").unwrap();

    let mut models: HashMap<u64, model_schema::Model> = HashMap::new();

    if std::path::Path::new("./models.json").metadata().unwrap().len() > 0 {
        let mut model_buf_reader = BufReader::new(&model_file);
        let mut model_file_string: String = String::new();
        model_buf_reader.read_to_string(&mut model_file_string).unwrap();
        models = serde_json::from_str(model_file_string.as_str()).unwrap();
    }

    for (_, url) in urls_reader.lines().enumerate() {
        let mut exists: bool = false;
        let url = url.unwrap();
        for model in models.values() {
            exists = exists || model.check_url(&url);
        }
        if !exists {
            match parse::thangs::from_url(url) {
                Some(mut s) => {
                    if models.contains_key(&s.id()) {
                        eprintln!("id clash detected, regenerating id...");
                        s = s.replace_id();
                    }
                    models.insert(s.id(), s);
                },
                None => ()
            }
        }
    }

    let mut model_buf_writer = BufWriter::new(model_file);
    write!(model_buf_writer, "{}", serde_json::to_string_pretty(&models).unwrap()).unwrap();

}
