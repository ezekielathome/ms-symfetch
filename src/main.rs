use std::error::Error;
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

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Options::parse();

    // construct file id from timestamp and virtual_size
    let id = format!("{:08X}{:x}", opts.timestamp, opts.virtual_size);

    // request the binary from the symbol endpoint
    let url = format!("{SYMBOL_ENDPOINT}/{0}/{1}/{0}", opts.name, id);
    let res = ureq::get(&url).call()?;

    // determine the output path
    let name = match &opts.output {
        Some(buf) => {
            // create path if it doesn't exist
            fs::create_dir_all(buf).ok();
            buf.join(&opts.name)
        }
        None => PathBuf::from(&opts.name),
    };

    // create output file
    let mut output = File::create(name)?;

    // copy response stream into output file
    io::copy(&mut res.into_reader(), &mut output)?;

    Ok(())
}
