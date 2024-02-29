use std::fs::File;
use std::net::Ipv6Addr;
use std::error;
use std::io;
use std::net;
use std::fmt;
use std::convert;

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

impl convert::From<io::Error> for UpstreamError {
    fn from(t: io::Error) -> Self {
        UpstreamError::IO(t)
    }
}

impl convert::From<net::AddrParseError> for UpstreamError {
    fn from(error: net::AddrParseError) -> Self {
        UpstreamError::Parsing(error)
    }
}

fn main() -> Result<(), UpstreamError> {
    let _f = File::open("p1")?;

    let _localhost = "::x".parse::<Ipv6Addr>()?;

    Ok(())
}