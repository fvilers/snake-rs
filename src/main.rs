use snake_rs::run;
use std::{io, process};

fn main() {
    let mut stdout = io::stdout();

    if let Err(e) = run(&mut stdout) {
        eprintln!("{e}");
        process::exit(-1);
    }
}
