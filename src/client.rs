use std::cell::RefCell;

use tokio::io::*;
use tokio::net::{TcpStream, ToSocketAddrs, UnixStream};

use crate::{Commit, Info, Key, Type};

pub type Tcp = TcpStream;
pub type Unix = UnixStream;

pub struct Client<Socket> {
    conn: RefCell<BufStream<Socket>>,
}

pub struct Store<'a, Socket> {
    client: &'a Client<Socket>,
}

mod handshake {
    pub const V1: &str = "V1\n";
}

impl<Socket: Unpin + AsyncRead + AsyncWrite> Client<Socket> {
    async fn write_handshake(&self) -> std::io::Result<()> {
        let mut conn = self.conn.borrow_mut();
        conn.write_all(handshake::V1.as_bytes()).await?;
        conn.flush().await?;
        Ok(())
    }

    async fn read_handshake(&self) -> std::io::Result<bool> {
        let mut conn = self.conn.borrow_mut();
        let mut line = String::new();
        conn.read_line(&mut line).await?;
        Ok(line == handshake::V1)
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

    pub async fn close(self) -> std::io::Result<()> {
        self.conn.into_inner().shutdown().await?;
        Ok(())
    }

    async fn write_message(
        &self,
        conn: &mut BufStream<Socket>,
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

    async fn read_message<T: Type>(&self, conn: &mut BufStream<Socket>) -> std::io::Result<T> {
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

    pub fn store<'a>(&'a self) -> Store<'a, Socket> {
        Store { client: self }
    }
}

impl Client<TcpStream> {
    pub async fn new(s: impl ToSocketAddrs) -> std::io::Result<Client<TcpStream>> {
        let conn = TcpStream::connect(s).await?;
        let conn = RefCell::new(BufStream::new(conn));
        let client = Client { conn };
        client.do_handshake().await?;
        Ok(client)
    }
}

impl Client<UnixStream> {
    pub async fn new(s: impl AsRef<std::path::Path>) -> std::io::Result<Client<UnixStream>> {
        let conn = UnixStream::connect(s).await?;
        let conn = RefCell::new(BufStream::new(conn));
        let client = Client { conn };
        client.do_handshake().await?;
        Ok(client)
    }
}

impl<'a, Socket: Unpin + AsyncRead + AsyncWrite> Store<'a, Socket> {
    pub async fn set<T: Type>(&self, key: &Key, value: T, info: Info) -> std::io::Result<()> {
        self.client.request("store.set", (key, info, value)).await?;
        self.client.response::<()>().await
    }

    pub async fn find<T: Type>(&self, key: &Key) -> std::io::Result<Option<T>> {
        self.client.request("store.find", key).await?;
        self.client.response::<Option<T>>().await
    }

    pub async fn remove(&self, key: &Key, info: Info) -> std::io::Result<()> {
        self.client.request("store.remove", (key, info)).await?;
        self.client.response::<()>().await
    }
}
/*val create :
t ->
info:Irmin.Info.f ->
parents:hash list ->
tree ->
commit Error.result Lwt.t*/
impl<Hash: Type + Clone> Commit<Hash> {
    pub async fn create<Socket: Unpin + AsyncRead + AsyncWrite>(
        client: Client<Socket>,
        node: Hash,
        parents: impl Into<Vec<Hash>>,
        info: Info,
    ) -> std::io::Result<Commit<Hash>> {
        let parents = parents.into();
        client.request("new_commit", (info, parents, node)).await?;
        client.response::<Commit<Hash>>().await
    }
}

#[cfg(test)]
mod tests {
    use crate::client::*;
    use crate::Bytes;

    fn skip() -> std::io::Result<()> {
        eprintln!("Skipping client test: client not connected, perhaps the server isn't running?");
        return Ok(());
    }

    #[tokio::test]
    async fn test_client() -> std::io::Result<()> {
        let client = match Client::<Tcp>::new("127.0.0.1:8888").await {
            Ok(c) => c,
            Err(_) => return skip(),
        };
        client.ping().await?;
        let key = Key::new(["a", "b", "c", "d"]);
        let store = client.store();
        store
            .set(&key, Bytes::from("testing".as_bytes()), Info::new())
            .await?;
        let s: Option<String> = store.find(&key).await?;
        assert_eq!(s, Some("testing".to_string()));
        store.remove(&key, Info::new()).await?;
        client.close().await?;
        Ok(())
    }
}
