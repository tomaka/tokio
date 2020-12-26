use crate::io::PollEvented;
use crate::process::kill::Kill;
use crate::process::SpawnedChild;

use mio::event::Evented;
use mio::{Poll as MioPoll, PollOpt, Ready, Token};
use std::fmt;
use std::future::Future;
use std::io;
use std::pin::Pin;
use std::process::{Child as StdChild, Command as StdCommand, ExitStatus};
use std::task::Context;
use std::task::Poll;

#[must_use = "futures do nothing unless polled"]
pub(crate) struct Child {
}

impl fmt::Debug for Child {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Child")
            .finish()
    }
}

pub(crate) fn spawn_child(cmd: &mut StdCommand) -> io::Result<SpawnedChild> {
    unimplemented!()
}

impl Child {
    pub(crate) fn id(&self) -> u32 {
        unimplemented!()
    }
}

impl Kill for Child {
    fn kill(&mut self) -> io::Result<()> {
        unimplemented!()
    }
}

impl Future for Child {
    type Output = io::Result<ExitStatus>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unimplemented!()
    }
}

pub(crate) fn try_wait(child: &StdChild) -> io::Result<Option<ExitStatus>> {
    unimplemented!()
}

pub(crate) type ChildStdin = PollEvented<Dummy>;
pub(crate) type ChildStdout = PollEvented<Dummy>;
pub(crate) type ChildStderr = PollEvented<Dummy>;

#[derive(Debug)]
pub(crate) struct Dummy;

impl io::Read for Dummy {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        unimplemented!()
    }
}

impl io::Write for Dummy {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        unimplemented!()
    }

    fn flush(&mut self) -> io::Result<()> {
        unimplemented!()
    }
}

impl Evented for Dummy {
    fn register(
        &self,
        poll: &MioPoll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        unimplemented!()
    }

    fn reregister(
        &self,
        poll: &MioPoll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        unimplemented!()
    }

    fn deregister(&self, poll: &MioPoll) -> io::Result<()> {
        unimplemented!()
    }
}
