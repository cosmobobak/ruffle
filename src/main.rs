use std::io::{self, Write};

use rand::seq::SliceRandom;

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return Ok(());
    }
    let output = std::io::stdout();
    let file = std::fs::File::open(&args[1])?;
    let mmap = unsafe { memmap::Mmap::map(&file)? };
    let mut lines = mmap.split(|&c| c == b'\n').collect::<Vec<_>>();
    lines.shuffle(&mut rand::thread_rng());
    let mut output = std::io::BufWriter::new(output.lock());
    for line in lines {
        output.write_all(line)?;
        output.write_all(b"\n")?;
    }
    Ok(())
}
