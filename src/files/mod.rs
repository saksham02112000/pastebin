use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use serde_json::to_string;

mod random_hash;

pub async fn create_new_file(file_data: String, extension: String){
    let random_string: String= random_hash::generate_random_hash().await;
    let initial_path: &str = "./src/files_storage/";
    // let path = Path::new(initial_path+random_string+".".to_owned()+extension.to_owned());
    let path_string = format!("{}{}{}{}", initial_path, random_string, ".", extension);
    let path = Path::new(&path_string);
    let display = path.display();
    println!("{}", display);

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(file_data.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}