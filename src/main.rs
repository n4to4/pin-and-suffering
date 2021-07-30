use futures::Future;
use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncReadExt, ReadBuf},
    time::{Instant, Sleep},
};

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {
    let mut buf = vec![0u8; 128 * 1024];
    let f = File::open("/dev/urandom").await?;
    let mut f = SlowRead::new(f);

    {
        let mut f = unsafe { Pin::new_unchecked(&mut f) };

        let before = Instant::now();
        f.read_exact(&mut buf).await?;
        println!("Read {} bytes in {:?}", buf.len(), before.elapsed());
    }

    let mut f = f.into_inner();

    let before = Instant::now();
    f.read_exact(&mut buf).await?;
    println!("Read {} bytes in {:?}", buf.len(), before.elapsed());

    Ok(())
}

struct SlowRead<R> {
    reader: R,
    sleep: Sleep,
}

impl<R> SlowRead<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            sleep: tokio::time::sleep(Default::default()),
        }
    }
}

impl<R> SlowRead<R>
where
    R: Unpin,
{
    fn into_inner(self) -> R {
        self.reader
    }
}

impl<R> AsyncRead for SlowRead<R>
where
    R: AsyncRead + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        // pin-project both fields
        let (mut sleep, reader) = unsafe {
            let this = self.get_unchecked_mut();
            (
                Pin::new_unchecked(&mut this.sleep),
                Pin::new_unchecked(&mut this.reader),
            )
        };

        match sleep.as_mut().poll(cx) {
            Poll::Ready(_) => {
                sleep.reset(Instant::now() + Duration::from_millis(25));
                reader.poll_read(cx, buf)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
