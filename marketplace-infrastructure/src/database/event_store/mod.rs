#[allow(clippy::module_inception)]
mod event_store;
pub use event_store::*;

#[cfg(test)]
mod test;