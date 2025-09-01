mod all;
pub use all::*;

#[cfg(feature = "deprecated-icons")]
mod deprecated;
#[cfg(feature = "deprecated-icons")]
pub use deprecated::*;
