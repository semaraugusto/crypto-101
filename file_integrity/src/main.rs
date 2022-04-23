use sha2::{Digest, Sha256};
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use clap::Parser;

/// Program used to get the sha256 signature of a file
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[clap(short, long)]
    in_filename: String,
}
const KB: u64 = 1024;
#[derive(Debug)]
struct ReverseFileIter {
    file: File,
    filesize: u64,
    offset: i64,
}

impl ReverseFileIter {
    fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let metadata = file.metadata()?;
        let filesize = metadata.len();
        let offset = (filesize % KB) as i64;

        Ok(ReverseFileIter {
            file,
            filesize,
            offset,
        })
    }
}

impl Iterator for ReverseFileIter {
    type Item = (usize, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        // println!("offset {}!", self.offset);
        if self.offset <= self.filesize as i64 {
            self.file.seek(SeekFrom::End(-self.offset)).unwrap();

            let mut buf = vec![0; KB as usize];
            let len = self.file.read(&mut buf).unwrap();
            // println!("len {}!", len);

            self.offset += 1024;

            return Some((len, buf));
        }
        None
    }
}

fn sign(file: ReverseFileIter) -> Vec<u8> {
    file.into_iter()
        .fold(vec![0; KB as usize], |mut signature, (len, mut block)| {
            if signature == vec![0; KB as usize] {
                signature = Sha256::digest(&block[0..len]).to_vec();
                println!("init fold {:?}!", signature);
                return signature;
            }
            block.extend(signature);
            signature = Sha256::digest(&block[0..block.len()]).to_vec();
            signature
        })
}

fn main() {
    let args = Args::parse();
    println!("in {}!", args.in_filename);

    let file = ReverseFileIter::new(Path::new(&args.in_filename)).unwrap();
    println!("file {:?}!", file);
    let signature = sign(file);
    println!("signature {:?}!", hex::encode(signature));
}
