//! # httpdt
//!
//! A datetime library for HTTP clients and servers.
//!
//! Generates timestamps for use in the HTTP Date header,
//! the only format required for implementation of HTTP.
//!
//! Calculates with a focus on clarity from `SystemTime`,
//! with no external dependencies, and provides for
//! updates to previously generated datetimes for speed.

mod datetime;
mod date;
mod time;

pub use datetime::Datetime;
