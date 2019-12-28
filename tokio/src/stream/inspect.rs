use crate::stream::Stream;

use core::fmt;
use core::pin::Pin;
use core::task::{Context, Poll};
use pin_project_lite::pin_project;

pin_project! {
    /// Stream returned by the [`inspect`](super::StreamExt::inspect) method.
    #[must_use = "streams do nothing unless polled"]
    pub struct Inspect<St, F> {
        #[pin]
        stream: St,
        f: F,
    }
}

impl<St, F> fmt::Debug for Inspect<St, F>
where
    St: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Inspect")
            .field("stream", &self.stream)
            .finish()
    }
}

impl<St, F> Inspect<St, F> {
    pub(super) fn new(stream: St, f: F) -> Self {
        Self { stream, f }
    }
}

impl<St, F> Stream for Inspect<St, F>
where
    St: Stream,
    F: FnMut(&St::Item),
{
    type Item = St::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<St::Item>> {
        self.as_mut().project().stream.poll_next(cx).map(|ready| {
            if let Some(e) = ready.as_ref() {
                (self.as_mut().project().f)(e);
            }
            ready
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}
