//! Utilities for interacting with Gubernator manifests.

pub mod common;
mod manifest;

pub use manifest::Manifest;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
