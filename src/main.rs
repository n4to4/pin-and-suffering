use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _fut = MyFuture::new().await;
}

struct MyFuture {
    slept: bool,
}

impl MyFuture {
    fn new() -> Self {
        Self { slept: false }
    }
}

impl Future for MyFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("MyFuture::poll");

        match self.slept {
            false => {
                let waker = cx.waker().clone();
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_secs(1));
                    waker.wake();
                });
                self.slept = true;
                Poll::Pending
            }
            true => Poll::Ready(()),
        }
    }
}
