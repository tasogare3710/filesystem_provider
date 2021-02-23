//! ```
//! use ::{
//!     filesystem_provider_api::{
//!         fs::{
//!             ops,
//!             FileSystem as _,
//!             entity::File as _,
//!             entity::Type,
//!         },
//!         provider::make::Make as _
//!     },
//!     filesystem_provider_impl_disk::provider::Provider,
//! };
//!
//! fn foo() -> Result<(), Box<dyn std::error::Error>> {
//!     let root = std::path::PathBuf::from(".");
//!     let sub = root.join("test.txt");
//!     let mut filesystem = Provider::make(root);
//!
//!     let file = ops::OpenFile::open(&mut filesystem, sub)?;
//!     assert!(file.is_file());
//!     Ok(())
//! }
//! ```

pub mod fs;
pub mod provider;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
