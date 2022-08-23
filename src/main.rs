//! A batteries-included binary template.

// remove these when ready
#![allow(unused_imports)]
use std::{borrow::BorrowMut, pin::Pin};

// #![allow(unused_variables)]
// #![allow(dead_code)]
use async_std::{channel, prelude::*};
use futures_lite::prelude::*;
use utils::MyError;

mod pin_project;
mod my_async_trait;

#[cfg(test)] mod tests;
mod utils;
#[async_std::main]
async fn main() -> Result<(), MyError> {
  let context = utils::setup()?;
  if std::env::var("DOTENV_OK").is_ok() {
    tracing::info!("Hello, {}!", context.args.name);
    tracing::debug!("and build info: {:#?}", context.s);
    let (tx, rx) = channel::unbounded::<i32>();
    let _ = async {
      (0..100).for_each(|v| {
        let _ = tx.send(v);
      })
    }
    .await;

    let mut values = vec![];
    while let Ok(v) = rx.try_recv() {
      values.push(v);
    }

    println!("Values={:?}", values);
  }
  Ok(())
}

struct TracingReader<R>
where R: AsyncRead {
  inner: R,
}

impl<R> AsyncRead for TracingReader<R>
where R: AsyncRead
{
  fn poll_read(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
    buf: &mut [u8],
  ) -> std::task::Poll<std::io::Result<usize>> {
    // it's useful to have self pinned in mem, so that it can be polled reliably.
    // nope: cannot borrow self as mutable, that guy's pinned.
    // let inner : &mut R = &mut self.inner;
    // let inner: &mut R = &mut self.get_mut().inner; // No: R cannot be unpinned.

    // Try to live dangerously: getting an unchecked reference means that we tell the compiler that
    // inner won't move. but we still can't poll it: inner isn't pinned.
    // let inner: &mut R = unsafe { &mut self.get_unchecked_mut().inner };
    // inner.poll_read(cx,buf);

    // // nope: inner not pinned, silly try boxpinning? Grr. No: R cannot be unpinned.
    // let inner: &mut R = &mut Box::pin(&mut self).get_mut().inner;
    // inner.poll_read(cx, buf);
    // try another impl where R: Unpin?

    // Amos again. "Safety is for chumps". Who needs Unpin.
    let inner: Pin<&mut R> = unsafe { self.map_unchecked_mut(|x| &mut x.inner)};
    inner.poll_read(cx, buf) // 👿 I work but there's a way to do it safely. Go to the mod pin_project.


    // better tho, use pin-projection.

    // todo!()
  }
}


// impl<R> AsyncRead for TracingReader<R>
// where R: AsyncRead + Unpin
// {
//   fn poll_read(
//     self: std::pin::Pin<&mut Self>,
//     cx: &mut std::task::Context<'_>,
//     buf: &mut [u8],
//   ) -> std::task::Poll<std::io::Result<usize>> {
//     // cannot move out of deref of type.  Pin<..> isn't copy.
//     // let inner: &mut R = &mut Box::pin(&mut self).get_mut().inner;

//     // okay, try  different type for inner, Pin<Box<&mut R>>:
//     // let inner: Pin<Box<&mut R>> = Box::pin(&mut self.get_mut().inner); // OHMYGOD
//     // inner.poll_read(cx,buf)
//     // can we poll_read from it? No. But it does suggest that I might want a &mut Box. Does that
//     // make sense?

//     // not sure how to do that ergonomically, so this will have to do:
//     // let inner: Pin<&mut Box<&mut R>> = Pin::new(&mut Box::new(&mut self.get_mut().inner));

//     // inner.poll_read(cx,buf) // and that almost passed typecheck. But Box gets dropped too
// early.     // let mut boxed = Box::new(&mut self.get_mut().inner);
//     // let inner: Pin<&mut Box<&mut R>> = Pin::new(&mut boxed);
//     // inner.poll_read(cx, buf)  // WOW WOW WOW WE DID IT
//     // but there's probably a better way. Copy that below for conciseness and back to you Amos.

//     // Amos: Well, we could do this without R: Unpin. Scroll uppppp

//     todo!()
//   }
// }

// impl<R> AsyncRead for TracingReader<R>
// where R: AsyncRead + Unpin
// {
//   fn poll_read(
//     self: std::pin::Pin<&mut Self>,
//     cx: &mut std::task::Context<'_>,
//     buf: &mut [u8],
//   ) -> std::task::Poll<std::io::Result<usize>> {
//     let mut boxed = Box::new(&mut self.get_mut().inner);
//     let inner: Pin<&mut Box<&mut R>> = Pin::new(&mut boxed);
//     inner.poll_read(cx, buf) // 😎
//   }
// }