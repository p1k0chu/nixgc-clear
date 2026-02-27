use std::{borrow::Cow, fmt::Display, io, string::FromUtf8Error};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FailedToMakeString,
    ChildFailed(Option<i32>),
    Msg(Cow<'static, str>),
    /// Not actually an error. lol.
    Quit,
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Self {
        Self::FailedToMakeString
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(error) => error.fmt(f),
            Error::FailedToMakeString => write!(f, "Failed to make a string"),
            Error::ChildFailed(Some(x)) => write!(f, "Child exited abnormally with code {}", x),
            Error::ChildFailed(None) => write!(f, "Child exited abnormally"),
            Error::Msg(cow) => write!(f, "{}", *cow),
            Error::Quit => write!(f, "Quit"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(error) => Some(error),
            Error::FailedToMakeString => None,
            Error::ChildFailed(_) => None,
            Error::Msg(_) => None,
            Error::Quit => None,
        }
    }
}

impl From<Cow<'static, str>> for Error {
    fn from(value: Cow<'static, str>) -> Self {
        Self::Msg(value)
    }
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Cow::Borrowed(value).into()
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Cow::Owned::<str>(value).into()
    }
}
