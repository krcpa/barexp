pub use barexp_macros::{export as barexp_export, export_fullpath as barexp_export_fullpath};

#[derive(Debug)]
pub struct ExportItem {
    pub name: &'static str,
    pub module_path: &'static str,
    pub full_path: &'static str,
    pub is_fullpath: bool,
}

inventory::collect!(ExportItem);

#[macro_export]
macro_rules! export {
    () => {
        pub use inventory::iter;
        #[allow(unused_imports)]
        pub use self::*;
    };
}

// barexp/src/lib.rs
#[cfg(test)]
mod tests {

    mod basic_test {
        use crate::{barexp_export, ExportItem};
        use super::*;

        #[test]
        fn test_basic_export() {
            #[barexp_export]
            struct TestStruct {}

            let exports: Vec<_> = inventory::iter::<ExportItem>().collect();
            assert!(!exports.is_empty());
            assert_eq!(exports[0].name, "TestStruct");
        }
    }

    mod multi_exports {
        use crate::{barexp_export, ExportItem};
        use super::*;

        #[test]
        fn test_multiple_exports() {
            #[barexp_export]
            struct Test1 {}

            #[barexp_export]
            struct Test2 {}

            let exports: Vec<_> = inventory::iter::<ExportItem>().collect();
            let names: Vec<_> = exports.iter().map(|e| e.name).collect();
            assert!(names.contains(&"Test1"));
            assert!(names.contains(&"Test2"));
        }
    }
}