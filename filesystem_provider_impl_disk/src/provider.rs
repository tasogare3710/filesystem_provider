use ::std::path::PathBuf;
use filesystem_provider_api::provider::make as api_make;

#[derive(Debug)]
pub struct Provider;

/// ```
/// use ::{
///     filesystem_provider_api::provider::make::Make as _,
///     filesystem_provider_impl_disk::provider::Provider,
/// };
///
/// let root = std::path::PathBuf::from(".");
/// let mut filesystem = Provider::make(root);
/// ```
impl api_make::Make for Provider {
    type FS = crate::fs::FileSystem;

    fn make(root: PathBuf) -> Self::FS {
        let root = root.into_boxed_path();
        crate::fs::FileSystem { root }
    }
}

/// `crate::provider::Provider`は以下のcapabilitiesを備えたファイルシステムを作ることができる。
///
/// - `Readable`
/// - `Writable`
/// - `Appendable`
/// - `Truncatable`
/// - `Removable`
///
#[cfg(test)]
mod test_capabilities {
    use ::filesystem_provider_api::fs;

    fn inspect<F: fs::Introspect>(_: &F) {}
    fn filesystem<F: fs::FileSystem<MetadataE = std::io::Error>>(_: &F) {}

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        use filesystem_provider_api::provider::make::Make;

        let filesystem = crate::provider::Provider::make(std::path::PathBuf::from("."));

        inspect(&filesystem);

        assert!(fs::Introspect::is_readable(&filesystem));
        assert!(fs::Introspect::is_writable(&filesystem));
        assert!(fs::Introspect::is_appendable(&filesystem));
        assert!(fs::Introspect::is_truncatable(&filesystem));
        assert!(fs::Introspect::is_removable(&filesystem));

        self::filesystem(&filesystem);

        Ok(())
    }
}

/// `crate::provider::Provider`は以下の操作を備えたファイルシステムを作ることができる。
///
/// - open::{File, Dir}
/// - create::{File, Dir}
/// - remove::{File, Dir}
///
#[cfg(test)]
mod test_operations {
    use ::filesystem_provider_api::fs;

    fn open_file<F: fs::ops::OpenFile<E = crate::fs::OpenEntityError, File = crate::fs::File>>(_: &F) {}
    fn open_dir<F: fs::ops::OpenDir<E = crate::fs::OpenEntityError, Dir = crate::fs::Dir>>(_: &F) {}

    fn create_file<F: fs::ops::CreateFile<E = crate::fs::CreateEntityError, File = crate::fs::File>>(_: &F) {}
    fn create_dir<F: fs::ops::CreateDir<E = crate::fs::CreateEntityError, Dir = crate::fs::Dir>>(_: &F) {}

    fn remove_file<'a, F: fs::ops::RemoveFile<E = crate::fs::RemoveEntityError>>(_: &F) {}
    fn remove_dir<'a, F: fs::ops::RemoveDir<E = crate::fs::RemoveEntityError>>(_: &F) {}

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        use filesystem_provider_api::provider::make::Make;

        let filesystem = crate::provider::Provider::make(std::path::PathBuf::from("."));

        open_file(&filesystem);
        open_dir(&filesystem);

        create_file(&filesystem);
        create_dir(&filesystem);

        remove_file(&filesystem);
        remove_dir(&filesystem);

        Ok(())
    }
}
