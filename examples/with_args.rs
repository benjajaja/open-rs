use std::{env, process};

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let (path_or_url, app, args) = match args.as_slice() {
        [path_or_url, app] => (path_or_url, app, &[] as &[String]),
        [path_or_url, app, args @ ..] => (path_or_url, app, args),
        _ => {
            eprintln!("usage: with <path-or-url> <app> [args...]");
            process::exit(1);
        }
    };

    match open::with_args(path_or_url, app, args) {
        Ok(()) => println!("Opened '{}' successfully.", path_or_url),
        Err(err) => {
            eprintln!("An error occurred when opening '{}': {}", path_or_url, err);
            process::exit(3);
        }
    }
}
