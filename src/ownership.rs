//! Own-borrow types, that lets you use both borrowed and owned values interchangeably.
//! Differs from `Cow` mainly in that it borrows mutably, and doesn't convert the borrowed type
//! into the owned type on write.

mod ob;
mod obox;

pub use ob::*;
pub use obox::*;
