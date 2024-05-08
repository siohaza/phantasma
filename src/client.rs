use std::fmt;
use std::io;
use std::ops::Deref;
use std::str;

use log::debug;
use thiserror::Error;

use crate::server_info::{Region, ServerInfo};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid packet data")]
    InvalidPacket,
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}

pub struct Filter<'a>(&'a [u8]);

impl fmt::Debug for Filter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.0))
    }
}

impl<'a> Deref for Filter<'a> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Debug)]
pub enum Packet<'a> {
    Challenge(Option<u32>),
    ServerAdd(Option<u32>, ServerInfo<&'a str>),
    ServerRemove,
    QueryServers(Region, Filter<'a>),
    ServerInfo,
}

impl<'a> Packet<'a> {
    pub fn decode(s: &'a [u8]) -> Result<Self, Error> {
        match s {
            [b'1', region, tail @ ..] => {
                let region = Region::try_from(*region).map_err(|_| Error::InvalidPacket)?;
                let (tail, _) = decode_cstr(tail)?;
                let (tail, filter) = decode_cstr(tail)?;
                if !tail.is_empty() {
                    return Err(Error::InvalidPacket);
                }
                Ok(Self::QueryServers(region, Filter(filter)))
            }
            [b'q', 0xff, b0, b1, b2, b3] => {
                let challenge = u32::from_le_bytes([*b0, *b1, *b2, *b3]);
                Ok(Self::Challenge(Some(challenge)))
            }
            [b'0', b'\n', tail @ ..] => {
                let (challenge, info, tail) =
                    ServerInfo::from_bytes(tail).map_err(|_| Error::InvalidPacket)?;
                if !tail.is_empty() {
                    debug!("unexpected data at end: {:?}", tail);
                }
                Ok(Self::ServerAdd(challenge, info))
            }
            [b'b', b'\n'] => Ok(Self::ServerRemove),
            [b'q'] => Ok(Self::Challenge(None)),
            [0xff, 0xff, 0xff, 0xff, b'S', b'o', b'u', b'r', b'c', b'e', b' ', b'E', b'n', b'g', b'i', b'n', b'e', b' ', b'Q', b'u', b'e', b'r', b'y', _, _] => {
                Ok(Self::ServerInfo)
            }
            _ => Err(Error::InvalidPacket),
        }
    }
}

fn decode_cstr(data: &[u8]) -> Result<(&[u8], &[u8]), Error> {
    data.iter()
        .position(|&c| c == 0)
        .ok_or(Error::InvalidPacket)
        .map(|offset| (&data[offset + 1..], &data[..offset]))
}
