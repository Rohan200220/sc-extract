//! Library to extract graphics and decode csv files from Supercell's game files.
//!
//! The library exposes three high-level functions, [`process_sc`],
//! [`process_tex`] and [`process_csv`], to process extracted `sc`, `_tex.sc`
//! and `.csv` files respectively.
//!
//! This library is simply intended to get high quality graphics and data from
//! the files. It is in no way an attempt to:
//!
//! - modify the game in any way
//! - create a clone or any other game based on any of Supercell's games
//! - make profit
//!
//! [`process_sc`]: ./fn.process_sc.html
//! [`process_tex`]: ./fn.process_tex.html
//! [`process_csv`]: ./fn.process_csv.html

mod error;
mod extractors;
mod utils;

#[doc(inline)]
pub use error::Error;
#[doc(inline)]
pub use extractors::{csv::process_csv, sc::process_sc, tex::process_tex};
