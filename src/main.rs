use std::env;
use std::io;
use std::io::BufRead;
use std::io::ErrorKind;
use std::process;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

fn help() {
    print!(
        "\
Descriptions:
  Read input from stdin and output only UTF-8 encoded lines.

Usage:
  {} </path/to/file

Options:
  -h, --help    Show this message and exit.
  --version     Show version info and exit.
",
        NAME
    )
}

fn version() {
    println!("{}", VERSION);
}

fn main() {
    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => {
                help();
                process::exit(0);
            }
            "--version" => {
                version();
                process::exit(0);
            }
            _ => {
                eprintln!("Invalid option: {}", arg);
                process::exit(1);
            }
        }
    }

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut buf = String::new();

    loop {
        match handle.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_n) => {
                print!("{}", buf);
                buf.clear();
            }
            Err(e) => {
                match e.kind() {
                    // Remove non UTF-8 line.
                    ErrorKind::InvalidData => continue,
                    _ => {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                }
            }
        }
    }
}
