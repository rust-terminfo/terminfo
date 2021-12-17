use std::io;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
	/// IO error.
	Io(io::Error),

	/// Database not found.
	NotFound,

	/// Parsing error.
	Parse,

	/// Expansion error.
	Expand(Expand),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Expand {
	/// The expansion string is invalid.
	Invalid,

	/// There was a type mismatch while expanding.
	TypeMismatch,

	/// The stack underflowed while expanding.
	StackUnderflow,
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<io::Error> for Error {
	fn from(value: io::Error) -> Self {
		Error::Io(value)
	}
}

impl From<Expand> for Error {
	fn from(value: Expand) -> Self {
		Error::Expand(value)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
		f.write_str(error::Error::description(self))
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Error::Io(ref err) =>
				err.description(),

			Error::NotFound =>
				"Capability database not found.",

			Error::Parse =>
				"Failed to parse capability database.",

			Error::Expand(ref err) =>
				match *err {
					Expand::Invalid =>
						"The expansion string is invalid.",

					Expand::StackUnderflow =>
						"Not enough elements on the stack.",

					Expand::TypeMismatch =>
						"Type mismatch.",
				},
		}
	}
}
