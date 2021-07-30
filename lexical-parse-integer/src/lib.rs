//! Fast lexical string-to-integer conversion routines.
//! TODO(ahuszagh) Add more documentation here...

#[macro_use]
mod error;

//pub mod algorithm;
//pub mod compact;
pub mod options;
pub mod parse;
pub mod sign;
mod bare;   // TODO(ahuszagh) Remove

mod api;

// Re-exports
//pub use self::api::{FromLexical, FromLexicalWithOptions};
pub use self::options::Options;
pub use self::bare::FromLexical;
