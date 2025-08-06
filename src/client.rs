use std::io;

use bytes::Bytes;
use tokio::net::{TcpStream, ToSocketAddrs};

use crate::conn::Connection;

pub(crate) struct Client {
    conn: Connection,
}

pub async fn connect<T: ToSocketAddrs>(addr: T) -> io::Result<Client> {
    let socket = TcpStream::connect(addr).await?;
    let conn = Connection::new(socket);

    Ok(Client { conn })
}

impl Client {
    pub async fn get(&self, _key: &str) -> io::Result<Option<Bytes>> {
        todo!()
    }

    pub async fn set(&mut self, _key: &str, _value: Bytes) {
        todo!()
    }
}
