#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod algebra;
pub mod combinatorics;

pub mod data_storage;
pub mod analysis;
pub mod mclib;
pub mod observer;
pub mod plant;
pub mod signal;
pub mod state_space;
pub mod statistics;
pub mod system_identification;

//Re-export
pub use data_storage::*;
pub use algebra::*;