use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _fut = MyFuture {}.await;
}

struct MyFuture {}

impl Future for MyFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("MyFuture::poll");
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
