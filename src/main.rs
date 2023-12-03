// #![allow(dead_code, unused_variables, unused_imports)]
#![feature(ip)]

use std::{
    fs::File,
    io::{self, Read, Write, BufReader},
    net::{Ipv4Addr, Ipv6Addr, TcpStream},
    path::PathBuf
};
use is_utf8::libcore::is_utf8;
use structopt::StructOpt;
use anyhow::{Context, Result};

#[derive(StructOpt)]
struct Args {
    #[structopt(parse(from_os_str))]
    file: PathBuf,

    #[structopt(short = "v", default_value = "4")]
    version: u8,
    address: String,
    port: u16
}

fn is_text<R: Read>(reader: &mut R) -> Result<bool> {
    const CHUNK_SIZE: usize = 1024;
    let mut buffer = [0; CHUNK_SIZE];
    let bytes_read = reader.read(&mut buffer)?;

    let is_plain_text: bool = is_utf8(&buffer[..bytes_read]);

    Ok(is_plain_text)
}

fn is_valid_address(address: &str, version: &u8) -> bool {
    match version {
        4 => {
            if let Ok(ipv4_addr) = address.parse::<Ipv4Addr>() {
                return ipv4_addr.is_global();
            }
        }
        6 => {
            if let Ok(ipv6_addr) = address.parse::<Ipv6Addr>() {
                return ipv6_addr.is_global();
            }
        }
        _ => { // the ip provided isn't v4 or v6.
            return false;
        }
    };
    false
}

fn main() -> Result<()> {
    let args = Args::from_args();

    if !is_valid_address(&args.address, &args.version) {
        eprintln!("ERROR: IP address {} of version {} isn't valid!", &args.address, &args.version);
        eprintln!("If you believe this is an error, please report it.");
    }

    let file = File::open(&args.file)?;
    let mut buf_reader = io::BufReader::new(file);
    let destination_ip = match args.version {
       4 => {
            format!("{}:{}", &args.address, &args.port)
       } 
       6 => {
            format!("[{}]:{}", &args.address, &args.port)
       }
       _ => {
            todo!()
       }
    };

    if let Err(e) = is_text(&mut buf_reader) {
        eprintln!("ERROR: {}", e);
    
    }

    let mut stream = TcpStream::connect(destination_ip)?;
    let mut buffer = Vec::new();
    buf_reader.read_to_end(&mut buffer)?;
    stream.write_all(&buffer)?;

    Ok(())
}

