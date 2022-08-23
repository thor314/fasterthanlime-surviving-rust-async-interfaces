use std::{
  pin::Pin,
  task::{Context, Poll},
};

use async_std::{
  channel,
  io::{self, ReadExt},
  prelude::*,
};
use async_trait::async_trait;
use futures_lite::prelude::*;

use super::pin_project;

#[async_trait]
pub trait SimpleAsyncRead {
  /// Attempt to read from the `AsyncRead` into `buf`.
  ///
  /// On success, returns `Poll::Ready(Ok(num_bytes_read))`.
  ///
  /// If no data is available for reading, the method returns
  /// `Poll::Pending` and arranges for the current task (via
  /// `cx.waker().wake_by_ref()`) to receive a notification when the object becomes
  /// readable or is closed.
  ///
  /// # Implementation
  ///
  /// This function may not return errors of kind `WouldBlock` or
  /// `Interrupted`.  Implementations must convert `WouldBlock` into
  /// `Poll::Pending` and either internally retry or convert
  /// `Interrupted` into another error kind.
  async fn poll_read(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &mut [u8],
    // ) -> io::Result<usize>;
  ) -> Poll<io::Result<usize>>;
}

// #[async_trait]
// impl<R> SimpleAsyncRead for pin_project::TracingReader<R>
// // where R: AsyncRead  // not send means
// // poll might move across threads?
// where R: AsyncRead + Send
// {
//   async fn poll_read(
//     self: Pin<&mut Self>,
//     cx: &mut Context<'_>,
//     buf: &mut [u8],
//     // ) -> io::Result<usize> {
//   ) -> Poll<io::Result<usize>> {
//     // issue: Something ain't send, and it's R. Easy fix, but...
//     self.project().inner.poll_read(cx, buf)
//     // Why did R have to be Send?
//     // 1. inner.poll_read is async, meaning inner could be moved across threads. Therefore inner has
//     // to be send, or we have to ban multi-thread runtimes.
//     // hot tip: check out my threads like this:
//     let address = &self as &const _; println!("addr:{:?} => {:?}", address, std::thread::current().id());
//   }
// }

