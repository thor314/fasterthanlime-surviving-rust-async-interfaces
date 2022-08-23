use std::pin::Pin;

use async_std::{channel, prelude::*};
use futures_lite::prelude::*;
use pin_project::pin_project;

#[pin_project]
pub struct TracingReader<R>
where R: AsyncRead {
  #[pin] // pinning is "structural" for inner, meaning:
  // since we want to call `poll_read` on it, which requires `Pin<&mut Self>`, we must guarantee
  // that while this struct is pinned, the inner field is also pinned.
  //
  // Our R: Unpin implied
  pub inner: R,
}

impl<R> AsyncRead for TracingReader<R>
where R: AsyncRead
{
  fn poll_read(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
    buf: &mut [u8],
  ) -> std::task::Poll<std::io::Result<usize>> {
    // where before, we were required R: Unpin to write:
    // let mut boxed = Box::new(&mut self.get_mut().inner);
    // let inner: Pin<&mut Box<&mut R>> = Pin::new(&mut boxed);
    // inner.poll_read(cx, buf)

    // We now can write TF, without R: Unpin. `project` takes a Pin<&mut Self> to get a &mut T for
    // unpinned fields, and a Pin<&mut T> for pinned fields. The following inner will then be
    // pinned by project?
    let inner: Pin<&mut R> = self.project().inner; // 😎 nice
    inner.poll_read(cx, buf)
    // this has the additional benefit that we can't accidentally Unpin something that really
    // *should* be pinned while polling it.
  }
}

// now what if we want to make our own async trait interfaces? Gotta import tha async_trait crate,
// since GATs are still pretty recent. Go to mod my_async_trait.