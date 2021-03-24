use std::cell::RefCell;

use tokio::io::*;
use tokio::net::{TcpStream, ToSocketAddrs};

use crate::{irmin, Type};

pub struct Client {
    addr: std::net::SocketAddr,
    conn: RefCell<BufStream<TcpStream>>,
}

const V1: &str = "V1\n";

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Key(Vec<String>);

impl Type for Key {
    fn encode_bin<W: std::io::Write>(&self, dest: W) -> std::io::Result<usize> {
        self.0.encode_bin(dest)
    }

    fn decode_bin<R: std::io::Read>(src: R) -> std::io::Result<Self> {
        let x = Vec::<String>::decode_bin(src)?;
        Ok(Key(x))
    }
}

impl Key {
    pub fn new<'a>(a: impl AsRef<[&'a str]>) -> Key {
        Key(a
            .as_ref()
            .iter()
            .filter_map(|x| {
                if x.is_empty() {
                    None
                } else {
                    Some(x.to_string())
                }
            })
            .collect())
    }

    pub fn push(&mut self, p: impl Into<String>) {
        let p = p.into();
        if !p.is_empty() {
            self.0.push(p.into())
        }
    }

    pub fn pop(&mut self) -> Option<String> {
        self.0.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Type)]
pub struct Info {
    date: i64,
    author: String,
    message: String,
}

impl Info {
    pub fn new() -> Info {
        let now = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH);
        let date = match now {
            Ok(x) => x.as_secs(),
            Err(_) => 0,
        };

        Info {
            date: date as i64,
            author: String::from("irmin-rs"),
            message: String::new(),
        }
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = author.into();
        self
    }
}

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
        let len = data.len() as i64;
        conn.write_all(&len.to_be_bytes()).await?;
        conn.write_all(data.as_slice()).await?;
        conn.flush().await?;

        Ok(())
    }

    async fn read_message<T: Type>(&self, conn: &mut BufStream<TcpStream>) -> std::io::Result<T> {
        let mut len_buf = [0u8; 8];
        conn.read_exact(&mut len_buf).await?;
        let len = i64::from_be_bytes(len_buf);
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

    pub async fn set<T: Type>(&self, key: &Key, value: T, info: &Info) -> std::io::Result<()> {
        self.request("store.set", (key, info, value)).await?;
        self.response::<()>().await
    }

    pub async fn find<T: Type>(&self, key: &Key) -> std::io::Result<Option<T>> {
        self.request("store.find", key).await?;
        self.response::<Option<T>>().await
    }
}

#[cfg(test)]
mod tests {
    use crate::client::*;

    fn skip() -> std::io::Result<()> {
        eprintln!("Skipping client test: client not connected, perhaps the server isn't running?");
        return Ok(());
    }

    #[tokio::test]
    async fn test_client() -> std::io::Result<()> {
        let client = match Client::new("127.0.0.1:8888").await {
            Ok(c) => c,
            Err(_) => return skip(),
        };
        client.ping().await?;
        let key = Key::new(&["a", "b", "c", "d"]);
        let info = Info::new();
        client.set(&key, "testing", &info).await?;
        let s: Option<String> = client.find(&key).await?;
        assert_eq!(s, Some("testing".to_string()));
        client.close().await?;
        Ok(())
    }
}
