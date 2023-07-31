use std::fs::File;
use std::io::prelude::*;
use clap::Parser;


/// Simple program to remove OOB blocks from NAND dumps (or anything else that
/// uses a repeated pattern of [x-byte block][y-byte block], where the x-byte
/// blocks are the ones to be extracted. You must provide proper OOB and page
/// size values; we don't validate all that.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path to the target file
    #[arg(short, long)]
    file: String,

    /// OOB data size
    #[arg(short, long, default_value_t = 64)]
    oob_size: usize,

    /// Page size
    #[arg(short, long, default_value_t = 2048)]
    page_size: usize,
}


fn main() {
    let args = Args::parse();

    let outfile_name = format!("{}_deooby.out", args.file);
    let mut target_file = File::open(&args.file).unwrap();
    let mut orig_bytes = Vec::new();
    let mut new_bytes = Vec::new();

    target_file.read_to_end(&mut orig_bytes).unwrap();
    let total_size = orig_bytes.len();
    println!("[+] Original file: {} ({total_size} bytes)", &args.file);
    if total_size % ((args.oob_size + args.page_size)) != 0 {
        println!(">>> WARNING: file size not aligned for given page size and OOB size");
    }

    let mut cursor: usize = 0;
    while cursor < total_size {
        let page_end = cursor + args.page_size;
        if page_end > total_size {
            println!(">>> WARNING: reading final page would go beyond EOF, grabbing the remaining bytes");
            let page_slice = &orig_bytes[cursor..];
            new_bytes.append(&mut page_slice.to_owned());
            break
        }
        let page_slice = &orig_bytes[cursor..page_end];
        new_bytes.append(&mut page_slice.to_owned());
        cursor += args.page_size + args.oob_size;
    }
        if cursor > 0 {
            println!("Processed {} pages", ((cursor-args.oob_size) / args.page_size)-1);
        }

    println!("[+] Output file: {} ({} bytes)", outfile_name, new_bytes.len());
    let mut output_file = File::create(outfile_name).unwrap();
    output_file.write_all(&new_bytes.to_owned()).unwrap();
}
