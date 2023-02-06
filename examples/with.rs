use std::{env, process};

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let (path_or_url, app) = match args.as_slice() {
        [path_or_url, app] => (path_or_url, app),
        _ => {
            eprintln!("usage: with <path-or-url> <app>");
            process::exit(1);
        }
    };

    match open::with(path_or_url, app) {
        Ok(()) => println!("Opened '{}' successfully.", path_or_url),
        Err(err) => {
            eprintln!("An error occurred when opening '{}': {}", path_or_url, err);
            process::exit(3);
        }
    }
}
