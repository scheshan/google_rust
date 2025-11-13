/***
Let’s design a simple logging utility, using a trait Logger with a log method. Code which might log its progress can then take an &impl Logger. In testing, this might put messages in the test logfile, while in a production build it would send messages to a log server.

However, the StderrLogger given below logs all messages, regardless of verbosity. Your task is to write a VerbosityFilter type that will ignore messages above a maximum verbosity.

This is a common pattern: a struct wrapping a trait implementation and implementing that same trait, adding behavior in the process. What other kinds of wrappers might be useful in a logging utility?
 */

use std::fmt::Display;

pub trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: impl Display);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: impl Display) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

fn do_things(logger: &impl Logger) {
    logger.log(5, "FYI");
    logger.log(2, "Uhoh");
}

struct VerbosityFilter {
    max_verbosity: u8,
    inner: StderrLogger,
}

impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: impl Display) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message)
        }
    }
}

fn main() {
    let l = VerbosityFilter {
        max_verbosity: 3,
        inner: StderrLogger,
    };
    do_things(&l);
}
