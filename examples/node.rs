use std::env;
use std::path::PathBuf;
use rantools::{create_pdf_numerologie, read_template};

fn main() {
    let mut file_path = PathBuf::new();
    file_path.push(env::current_dir().unwrap().as_path());
    file_path.push("examples");
    file_path.push("template.an");

    match read_template(file_path.to_str().unwrap().to_string()) {
        Ok(ok) => {
            create_pdf_numerologie(ok.as_slice());
        },
        Err(err) => {
            eprintln!("{:?}", err)
        }
    };
}




