use std::fs::{self, File};
use std::io;
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Options {
    /// Name of the binary to fetch
    name: String,

    /// Timestamp of the binary to fetch
    timestamp: isize,

    /// Virtual size of the binary to fetch
    virtual_size: isize,

    /// Directory to output binary to.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

const SYMBOL_ENDPOINT: &str = "https://msdl.microsoft.com/download/symbols";

fn main() -> Result<(), ureq::Error> {
    let opts = Options::parse();
    let mut id = String::new();

    let part = format!("0000000{:X}", opts.timestamp);
    id.push_str(&part[part.len() - 8..]);
    id.push_str(&format!("{:x}", opts.virtual_size));

    let url = format!("{SYMBOL_ENDPOINT}/{0}/{1}/{0}", opts.name, id);
    let res = ureq::get(&url).call()?;

    let name = match opts.output {
        Some(mut buf) => {
            // ensure path exists
            let _ = fs::create_dir_all(&buf);

            buf.push(opts.name);
            buf.display().to_string()
        }
        None => opts.name,
    };

    // create output file
    let mut output = File::create(name).expect("failed to create file!");
    io::copy(&mut res.into_reader(), &mut output).expect("failed to copy stream to file!");

    Ok(())
}
