
//! LumberJack is a crate is a lightweight alternative to Python's
//! "Pandas" package.
//! 
//! Its main intention is to have a Python wrapper, but feel free
//! to make use of it in other settings!
//! 
//! # Examples
//! 
//! ```
//! 
//! ```

extern crate ndarray;
extern crate num;
extern crate libc;
extern crate num_traits;

#[macro_use]
pub mod macros;

pub mod series;
pub mod dataframe;
pub mod prelude;
pub mod containers;
