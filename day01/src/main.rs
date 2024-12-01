use utils::read_input_file;

fn main() {
    let input = read_input_file();
    match input {
        Ok(content) => {
            println!("{}", content);
        }
        Err(error) => {
            println!("Error reading file: {}", error);
        }
    }
}
