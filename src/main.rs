use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.is_empty() {
        read_stdin();
    } else {
        read_args(&args);
    }
}

fn read_stdin() {
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => (),
            Err(_) => break,
        }
        print!("{}", line);
    }
}

fn read_args(args: &Vec<String>) {
    let mut buf = vec![0u8; 1];
    for arg in args.iter() {
        let mut file = match File::open(arg) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("minicat: {}: no such file", arg);
                continue;
            }
        };

        let metadata = match file.metadata() {
            Ok(f) => f,
            Err(e) => {
                eprintln!("minicat: {:?}", e);
                continue;
            }
        };

        let size = metadata.len();
        for _n in 0..size {
            match file.read_exact(&mut buf) {
                Ok(()) => (),
                Err(e) => {
                    eprintln!("minicat: {:?}", e);
                    break;
                }
            };
            io::stdout().write(&buf).unwrap();
        }
    }
}
