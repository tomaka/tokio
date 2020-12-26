#![cfg(target_os = "wasi")]

use crate::signal::registry::{EventId, EventInfo, Init, Storage};
use crate::sync::mpsc::Receiver;

use std::io;
use std::task::{Context, Poll};

#[derive(Debug)]
pub(crate) struct OsStorage {
}

impl Init for OsStorage {
    fn init() -> Self {
        Self {
        }
    }
}

impl Storage for OsStorage {
    fn event_info(&self, id: EventId) -> Option<&EventInfo> {
        unimplemented!()
    }

    fn for_each<'a, F>(&'a self, mut f: F)
    where
        F: FnMut(&'a EventInfo),
    {
        unimplemented!()
    }
}

#[derive(Debug)]
pub(crate) struct OsExtraData {}

impl Init for OsExtraData {
    fn init() -> Self {
        Self {}
    }
}

/// Stream of events discovered via `SetConsoleCtrlHandler`.
///
/// This structure can be used to listen for events of the type `CTRL_C_EVENT`
/// and `CTRL_BREAK_EVENT`. The `Stream` trait is implemented for this struct
/// and will resolve for each notification received by the process. Note that
/// there are few limitations with this as well:
///
/// * A notification to this process notifies *all* `Event` streams for that
///   event type.
/// * Notifications to an `Event` stream **are coalesced** if they aren't
///   processed quickly enough. This means that if two notifications are
///   received back-to-back, then the stream may only receive one item about the
///   two notifications.
#[must_use = "streams do nothing unless polled"]
#[derive(Debug)]
pub(crate) struct Event {
    rx: Receiver<()>,
}

pub(crate) fn ctrl_c() -> io::Result<Event> {
    unimplemented!()
}

impl Event {
    pub(crate) async fn recv(&mut self) -> Option<()> {
        unimplemented!()
    }
}

/// Represents a stream which receives "ctrl-break" notifications sent to the process
/// via `SetConsoleCtrlHandler`.
///
/// A notification to this process notifies *all* streams listening for
/// this event. Moreover, the notifications **are coalesced** if they aren't processed
/// quickly enough. This means that if two notifications are received back-to-back,
/// then the stream may only receive one item about the two notifications.
#[must_use = "streams do nothing unless polled"]
#[derive(Debug)]
pub struct CtrlBreak {
}

impl CtrlBreak {
    /// Receives the next signal notification event.
    ///
    /// `None` is returned if no more events can be received by this stream.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use tokio::signal::windows::ctrl_break;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // An infinite stream of CTRL-BREAK events.
    ///     let mut stream = ctrl_break()?;
    ///
    ///     // Print whenever a CTRL-BREAK event is received
    ///     loop {
    ///         stream.recv().await;
    ///         println!("got signal CTRL-BREAK");
    ///     }
    /// }
    /// ```
    pub async fn recv(&mut self) -> Option<()> {
        unimplemented!()
    }

    /// Polls to receive the next signal notification event, outside of an
    /// `async` context.
    ///
    /// `None` is returned if no more events can be received by this stream.
    ///
    /// # Examples
    ///
    /// Polling from a manually implemented future
    ///
    /// ```rust,no_run
    /// use std::pin::Pin;
    /// use std::future::Future;
    /// use std::task::{Context, Poll};
    /// use tokio::signal::windows::CtrlBreak;
    ///
    /// struct MyFuture {
    ///     ctrl_break: CtrlBreak,
    /// }
    ///
    /// impl Future for MyFuture {
    ///     type Output = Option<()>;
    ///
    ///     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    ///         println!("polling MyFuture");
    ///         self.ctrl_break.poll_recv(cx)
    ///     }
    /// }
    /// ```
    pub fn poll_recv(&mut self, cx: &mut Context<'_>) -> Poll<Option<()>> {
        unimplemented!()
    }
}

cfg_stream! {
    impl crate::stream::Stream for CtrlBreak {
        type Item = ();

        fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<()>> {
            unimplemented!()
        }
    }
}

/// Creates a new stream which receives "ctrl-break" notifications sent to the
/// process.
///
/// # Examples
///
/// ```rust,no_run
/// use tokio::signal::windows::ctrl_break;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // An infinite stream of CTRL-BREAK events.
///     let mut stream = ctrl_break()?;
///
///     // Print whenever a CTRL-BREAK event is received
///     loop {
///         stream.recv().await;
///         println!("got signal CTRL-BREAK");
///     }
/// }
/// ```
pub fn ctrl_break() -> io::Result<CtrlBreak> {
    unimplemented!()
}
