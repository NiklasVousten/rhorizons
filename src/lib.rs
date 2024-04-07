//#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod client;
mod ephemeris;
mod major_bodies;
mod utilities;

/// Query struct to allow access to the Horizons System API
/// 
/// Current implementation progress can be seen on [Github]()
/// At the moment only parts of the Vector and Element Ephermis Types are implemented, and some part of the Oberserver type.
pub mod command;

pub mod command_old;

pub use client::{ephemeris_orbital_elements, ephemeris_vector, major_bodies};
pub use ephemeris::{EphemerisOrbitalElementsItem, EphemerisVectorItem};
pub use major_bodies::MajorBody;
