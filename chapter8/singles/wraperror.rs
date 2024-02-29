use std::fs::File;
use std::net::Ipv6Addr;
use std::error;
use std::io;
use std::net;
use std::fmt;

#[derive(Debug)]
enum UpstreamError {
    IO(io::Error),
    Parsing(net::AddrParseError),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for UpstreamError {

}

fn main() -> Result<(), UpstreamError> {
    let _f = File::open("invisible.txt").map_err(UpstreamError::IO)?;

    let _localhost = "::1".parse::<Ipv6Addr>().map_err(UpstreamError::Parsing)?;

    Ok(())
}