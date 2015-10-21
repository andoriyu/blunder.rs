#![doc(html_root_url = "https://andoriyu.github.io/blunder.rs/")]
#[macro_use] extern crate enum_primitive;
extern crate num;

extern crate errno;

use std::error::Error as StdError;
use std::fmt;
use std::result::Result;
use std::ops::Deref;


mod bsd;

pub use bsd::*;

/// Macro helper to propagate an error
#[macro_export]
macro_rules! fail {
    ($expr:expr) => (
        return Err(::std::error::FromError::from_error($expr));
        )
}

/// Macro helper to propagate an error if there is one. See BsdError::from_errno() for inpsiration.
#[macro_export]
macro_rules! maybe_fail {
    ($expr:expr) => ({
        if let Some(err) = $expr {
            fail!(err)
        }
    })
}

/// Generic af struct for errror handling
/// Designed to host anything that implements error::Error trait
/// Yet can host whatever (like errno from libc)
#[derive(Debug)]
pub struct Blunder<T> {
    /// How to identify the error
    kind: T,
    detail: Option<String>
}

/// Because we want easy switch/case on kind...
impl <T> Deref for Blunder<T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        &self.kind
    }
}
impl <T> Blunder<T> {
    /// Optional reasoning behind such behavior.
    /// Think "Client doesn't understand XXX cipher"
    pub fn detail(&self) -> Option<String> {
        self.detail.clone()
    }
}
impl <T: StdError> StdError for Blunder<T> {
    fn description(&self) -> &str {
        self.kind.description()
    }
    fn cause(&self) -> Option<&StdError> {
        self.kind.cause()
    }
}

impl <T: StdError> fmt::Display for Blunder<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.description())
    }
}

#[test]
fn it_works() {
    #[derive(Debug, PartialEq)]
    enum Wat {
        One,
    };
    impl fmt::Display for Wat {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f,"{}", "wat")
        }
    }
    impl StdError for Wat {
        fn description(&self) -> &str {
            "wat"
        }
        fn cause(&self) -> Option<&StdError> {
            None
        }
    }

    let error: Blunder<Wat> = Blunder { kind: Wat::One, detail: None };
    assert_eq!(error.cause().is_some(), false);
    assert_eq!(error.description(), "wat");
    assert_eq!(*error, Wat::One);
}
