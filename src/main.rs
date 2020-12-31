use faux;
use tokio::{io::BufReader, net::TcpStream, io::AsyncRead, net::TcpListener, io::AsyncReadExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

#[faux::create]
pub struct Connection
{
    reader: BufReader<OwnedReadHalf>,
    writer: OwnedWriteHalf,
}

#[faux::methods]
impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        let (r, writer) = stream.into_split();
        let reader = BufReader::new(r);
        Self {reader, writer}
    }
}

#[faux::methods(self_type="Pin")]
impl AsyncRead for Connection {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let me = unsafe { self.get_unchecked_mut() };
        let reader = unsafe {std::pin::Pin::new_unchecked(&mut me.reader) };
        AsyncRead::poll_read(reader, cx, buf)
    }
}

pub async fn connections() -> () {
    let listener = TcpListener::bind("localhost").await.unwrap();
    loop {
        match listener.accept().await {
            Err(_) => (),    /* log? */
            Ok((socket, _addr)) => {
                /* Tight coupling. That's okay. */
                let mut foo = Connection::new(socket);
                let mut num = [0; 10];
                foo.read(&mut num).await.unwrap();
            }
        }
    }
}

fn main() {}
