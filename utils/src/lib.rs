use std::{env, fs, io};

pub fn read_input_file() -> Result<String, io::Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)?;
    let contents = contents.trim_end().to_string();
    Ok(contents)
}
