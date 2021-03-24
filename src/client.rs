use std::cell::RefCell;

use tokio::io::*;
use tokio::net::{TcpStream, ToSocketAddrs};

use crate::Type;

pub struct Client {
    addr: std::net::SocketAddr,
    conn: RefCell<BufStream<TcpStream>>,
}

const V1: &str = "V1\n";

impl Client {
    async fn write_handshake(&self) -> std::io::Result<()> {
        let mut conn = self.conn.borrow_mut();
        conn.write_all(V1.as_bytes()).await?;
        conn.flush().await?;
        Ok(())
    }

    async fn read_handshake(&self) -> std::io::Result<bool> {
        let mut conn = self.conn.borrow_mut();
        let mut line = String::new();
        conn.read_line(&mut line).await?;
        Ok(line == V1)
    }

    async fn do_handshake(&self) -> std::io::Result<()> {
        self.write_handshake().await?;
        let ok = self.read_handshake().await?;
        if !ok {
            return Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                "Invalid handshake",
            ));
        }
        Ok(())
    }

    pub async fn new(s: impl ToSocketAddrs) -> std::io::Result<Client> {
        let conn = TcpStream::connect(s).await?;
        let addr = conn.peer_addr()?;
        let conn = RefCell::new(BufStream::new(conn));
        let client = Client { conn, addr };
        client.do_handshake().await?;
        Ok(client)
    }

    pub async fn reconnect(&mut self) -> std::io::Result<()> {
        let conn = TcpStream::connect(&self.addr).await?;
        let conn = RefCell::new(BufStream::new(conn));
        self.conn = conn;
        self.do_handshake().await?;
        Ok(())
    }

    pub async fn close(self) -> std::io::Result<()> {
        self.conn.into_inner().shutdown().await?;
        Ok(())
    }

    async fn write_message(
        &self,
        conn: &mut BufStream<TcpStream>,
        msg: impl Type,
    ) -> std::io::Result<()> {
        let mut data = Vec::new();
        msg.encode_bin(&mut data)?;
        let len = data.len() as u64;
        conn.write_all(&len.to_be_bytes()).await?;
        conn.write_all(data.as_slice()).await?;
        conn.flush().await?;

        Ok(())
    }

    async fn read_message<T: Type>(&self, conn: &mut BufStream<TcpStream>) -> std::io::Result<T> {
        let mut len_buf = [0u8; 8];
        conn.read_exact(&mut len_buf).await?;
        let len = u64::from_be_bytes(len_buf);

        let mut data = vec![0u8; len as usize];
        conn.read_exact(data.as_mut_slice()).await?;

        T::decode_bin(data.as_slice())
    }

    async fn request(&self, command: impl AsRef<str>, msg: impl Type) -> std::io::Result<()> {
        let mut conn = self.conn.borrow_mut();
        conn.write_all(command.as_ref().as_bytes()).await?;
        conn.write_u8(b'\n').await?;
        self.write_message(&mut conn, msg).await?;

        Ok(())
    }

    async fn response<T: Type>(&self) -> std::io::Result<T> {
        let mut conn = self.conn.borrow_mut();

        let mut status_buf = [0];
        conn.read_exact(&mut status_buf).await?;
        if status_buf[0] > 0 {
            let s = self.read_message::<String>(&mut conn).await?;
            return Err(Error::new(ErrorKind::Other, s));
        } else {
            self.read_message::<T>(&mut conn).await
        }
    }

    pub async fn ping(&self) -> std::io::Result<()> {
        self.request("ping", ()).await?;
        self.response::<()>().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{client::*, *};

    #[tokio::test]
    async fn test_connect_handshake() -> std::io::Result<()> {
        let client = Client::new("127.0.0.1:8888").await?;
        client.ping().await?;
        client.close().await?;
        Ok(())
    }
}
