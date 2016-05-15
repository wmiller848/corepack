use std::fmt::Display;

use collections::String;

use alloc::boxed::Box;

use std::fmt;

use serde;

#[derive(Debug, Clone, Copy)]
pub enum Reason {
    TooBig,
    ExtraItems,
    Unsized,
    BadValue,
    EndOfStream,
    BadType,
    BadLength,
    BadVariant,
    BadField,
    NoField,
    DupField,
    UTF8Error,
    Other,
}

#[derive(Debug)]
pub struct Error {
    reason: Reason,
    detail: String,
    cause: Option<Box<serde::error::Error>>
}

impl Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match self.reason {
            Reason::TooBig => "Overflowing value",
            Reason::ExtraItems => "More items that expected",
            Reason::Unsized => "Unsized value",
            Reason::BadValue => "Invalid value",
            Reason::EndOfStream => "End of stream",
            Reason::BadType => "Invalid type",
            Reason::BadLength => "Invalid length",
            Reason::BadVariant => "Unknown variant",
            Reason::BadField => "Unknown field",
            Reason::NoField => "Missing field",
            Reason::DupField => "Duplicate field",
            Reason::UTF8Error => "UTF-8 encoding error",
            Reason::Other => "Other error"
        };

        if !self.detail.is_empty() {
            write!(fmt, "{}: {}", name, self.detail)
        } else {
            write!(fmt, "{}", name)
        }
    }
}

impl Error {
    pub const fn chain(reason: Reason, detail: String, cause: Option<Box<serde::error::Error>>) -> Error {
        Error {
            reason: reason,
            detail: detail,
            cause: cause
        }
    }

    pub const fn new(reason: Reason, detail: String) -> Error {
        Error::chain(reason, detail, None)
    }

    pub fn simple(reason: Reason) -> Error {
        Error::new(reason, String::new())
    }
}

impl serde::error::Error for Error {
    fn description(&self) -> &str {
        "Corepack error"
    }

    fn cause(&self) -> Option<&serde::error::Error> {
        if let Some(ref e) = self.cause {
            Some(e.as_ref())
        } else {
            None
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Error {
        Error::new(Reason::Other, msg.into())
    }

    fn invalid_value(msg: &str) -> Self {
        Error::new(Reason::BadValue, msg.into())
    }
}

impl serde::de::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Error {
        serde::ser::Error::custom(msg)
    }

    fn end_of_stream() -> Error {
        Error::simple(Reason::EndOfStream)
    }

    fn invalid_type(ty: serde::de::Type) -> Error {
        Error::new(Reason::BadType, format!("Expected {:?}", ty))
    }

    fn invalid_value(msg: &str) -> Error {
        Error::new(Reason::BadValue, msg.into())
    }

    fn invalid_length(len: usize) -> Error {
        Error::new(Reason::BadLength, format!("{}", len))
    }

    fn unknown_variant(field: &str) -> Error {
        Error::new(Reason::BadVariant, field.into())
    }

    fn unknown_field(field: &str) -> Error {
        Error::new(Reason::BadField, field.into())
    }

    fn missing_field(field: &'static str) -> Error {
        Error::new(Reason::NoField, field.into())
    }

    fn duplicate_field(field: &'static str) -> Error {
        Error::new(Reason::DupField, field.into())
    }
}