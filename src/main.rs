use std::io::{self, Write, Read};

use rand::seq::SliceRandom;

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let ref_args = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    match ref_args[..] {
        [_] | [_, "-"] => {
            let mut buf = Vec::new();
            // read from stdin
            let _bytes_read = io::stdin().lock().read_to_end(&mut buf)?;
            driver(&buf)
        }
        [_, filename] => {
            let file = std::fs::File::open(filename)?;
            let mmap = unsafe { memmap::Mmap::map(&file)? };
            driver(&mmap)
        }
        _ => {
            eprintln!("ruffle: extra operand '{}'", ref_args[2]);
            std::process::exit(1);
        }
    }
}

fn driver(bytes: &[u8]) -> Result<(), io::Error> {
    let output = std::io::stdout();
    let bytes = trim_ascii_whitespace(bytes);
    let mut lines = bytes.split(|&c| c == b'\n').collect::<Vec<_>>();
    lines.shuffle(&mut rand::thread_rng());
    let mut output = std::io::BufWriter::new(output.lock());
    for line in lines {
        output.write_all(line)?;
        output.write_all(b"\n")?;
    }
    Ok(())
}

fn trim_ascii_whitespace(bytes: &[u8]) -> &[u8] {
    let mut bytes = bytes;
    while let [first, rest @ ..] = bytes {
        if first.is_ascii_whitespace() {
            bytes = rest;
        } else {
            break;
        }
    }
    while let [rest @ .., last] = bytes {
        if last.is_ascii_whitespace() {
            bytes = rest;
        } else {
            break;
        }
    }
    bytes
}
