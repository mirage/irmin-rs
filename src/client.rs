use std::cell::RefCell;

use tokio::io::*;
use tokio::net::{TcpStream, ToSocketAddrs, UnixStream};

use crate::{Commit, Info, Key, Tree, Type};

use blake2::Digest;

pub type Tcp = TcpStream;
pub type Unix = UnixStream;

pub struct Client<Socket, Contents: Type, H: Type> {
    conn: RefCell<BufStream<Socket>>,
    _t: std::marker::PhantomData<(Contents, H)>,
}

pub struct Store<'a, Socket, Contents: Type, H: Type> {
    client: &'a Client<Socket, Contents, H>,
}

impl<Socket: Unpin + AsyncRead + AsyncWrite, Contents: Type, H: Type> Client<Socket, Contents, H> {
    async fn write_handshake(&self, content_name: &str) -> std::io::Result<()> {
        let mut conn = self.conn.borrow_mut();
        let hash = format!("{:x}\n", blake2::Blake2b::digest(content_name.as_bytes()));
        conn.write_all(hash.as_bytes()).await?;
        conn.flush().await?;
        Ok(())
    }

    async fn read_handshake(&self, content_name: &str) -> std::io::Result<bool> {
        let mut conn = self.conn.borrow_mut();
        let mut line = String::new();
        conn.read_line(&mut line).await?;
        let hash = format!("{:x}\n", blake2::Blake2b::digest(content_name.as_bytes()));
        Ok(line == hash)
    }

    async fn do_handshake(&self, content_name: impl AsRef<str>) -> std::io::Result<()> {
        let content_name = content_name.as_ref();
        self.write_handshake(content_name).await?;
        let ok = self.read_handshake(content_name).await?;
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
        T::decode_bin(&mut data.as_slice())
    }

    async fn request(&self, command: impl AsRef<str>, msg: impl Type) -> std::io::Result<()> {
        let mut conn = self.conn.borrow_mut();
        conn.write_all(command.as_ref().as_bytes()).await?;
        conn.write_u8(b'\n').await?;
        self.write_message(&mut *conn, msg).await?;

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
            self.read_message::<T>(&mut *conn).await
        }
    }

    pub async fn ping(&self) -> std::io::Result<()> {
        self.request("ping", ()).await?;
        self.response::<()>().await?;
        Ok(())
    }

    pub fn store<'a>(&'a self) -> Store<'a, Socket, Contents, H> {
        Store { client: self }
    }
}

impl<C: Type, H: Type> Client<TcpStream, C, H> {
    pub async fn new(
        s: impl ToSocketAddrs,
        content_name: impl AsRef<str>,
    ) -> std::io::Result<Client<TcpStream, C, H>> {
        let conn = TcpStream::connect(s).await?;
        let conn = RefCell::new(BufStream::new(conn));
        let client = Client {
            conn,
            _t: std::marker::PhantomData,
        };
        client.do_handshake(content_name).await?;
        Ok(client)
    }
}

impl<C: Type, H: Type> Client<UnixStream, C, H> {
    pub async fn new(
        s: impl AsRef<std::path::Path>,
        content_name: impl AsRef<str>,
    ) -> std::io::Result<Client<UnixStream, C, H>> {
        let conn = UnixStream::connect(s).await?;
        let conn = RefCell::new(BufStream::new(conn));
        let client = Client {
            conn,
            _t: std::marker::PhantomData,
        };
        client.do_handshake(content_name).await?;
        Ok(client)
    }
}

impl<'a, Socket: Unpin + AsyncRead + AsyncWrite, Contents: Type, H: Type>
    Store<'a, Socket, Contents, H>
{
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

impl<H: Type + Clone> Commit<H> {
    pub async fn create<Socket: Unpin + AsyncRead + AsyncWrite, Contents: Type>(
        client: &Client<Socket, Contents, H>,
        node: &H,
        parents: impl Into<Vec<H>>,
        info: Info,
    ) -> std::io::Result<Commit<H>> {
        let parents = parents.into();
        client.request("commit.v", (info, parents, node)).await?;
        client.response::<Commit<H>>().await
    }
}

impl<T: Type, H: Type> Tree<T, H> {
    pub async fn add<Socket: Unpin + AsyncRead + AsyncWrite, Contents: Type>(
        &self,
        client: &Client<Socket, Contents, H>,
        key: &Key,
        value: &T,
    ) -> std::io::Result<Tree<T, H>> {
        client.request("tree.add", (self, key, value)).await?;
        client.response::<Tree<T, H>>().await
    }

    pub async fn remove<Socket: Unpin + AsyncRead + AsyncWrite, Contents: Type>(
        &self,
        client: &Client<Socket, Contents, H>,
        key: &Key,
    ) -> std::io::Result<Tree<T, H>> {
        client.request("tree.remove", (self, key)).await?;
        client.response::<Tree<T, H>>().await
    }

    pub async fn find<Socket: Unpin + AsyncRead + AsyncWrite, Contents: Type>(
        &self,
        client: &Client<Socket, Contents, H>,
        key: &Key,
    ) -> std::io::Result<Option<T>> {
        client.request("tree.find", (self, key)).await?;
        client.response::<Option<T>>().await
    }

    pub async fn find_tree<Socket: Unpin + AsyncRead + AsyncWrite, Contents: Type>(
        &self,
        client: &Client<Socket, Contents, H>,
        key: &Key,
    ) -> std::io::Result<Option<Tree<T, H>>> {
        client.request("tree.find_tree", (self, key)).await?;
        client.response::<Option<Tree<T, H>>>().await
    }
}

#[cfg(test)]
mod tests {
    use crate::Bytes;
    use crate::{client::*, *};

    fn skip() -> std::io::Result<()> {
        eprintln!("Skipping client test: client not connected, perhaps the server isn't running?");
        return Ok(());
    }

    #[tokio::test]
    async fn test_client() -> std::io::Result<()> {
        let client =
            match Client::<Tcp, Bytes, HashRef<Blake2b>>::new("127.0.0.1:9181", "string").await {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Server error: {:?}", e);
                    return skip();
                }
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

        let tree = Tree::<Bytes, HashRef<Blake2b>>::empty();
        println!("{:?}", tree);

        let mut s = Vec::new();
        tree.encode_bin(&mut s).unwrap();
        println!("{:?}", s);
        {
            let b = Bytes::from("testing123".as_bytes());
            let t = tree.add(&client, &key, &b).await?;

            let key1 = Key::new(["key1"]);
            let t = t.add(&client, &key1, &b).await?;

            let x = t.find(&client, &key1).await?;
            assert!(b.as_ref() == x.unwrap().as_ref());

            let key2 = Key::new(["key2"]);
            let y = t.find_tree(&client, &key2).await?;
            assert!(y.is_none());

            let t = t.remove(&client, &key1).await?;
            let x = t.find(&client, &key1).await?;
            assert!(x.is_none());
        }

        client.close().await?;
        Ok(())
    }
}
