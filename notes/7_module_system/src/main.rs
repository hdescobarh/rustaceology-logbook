// module tree: package ⊇ crates ⊇ crate-tree := { root -> modules -> submodules -> paths}
fn main() {}
// modules/submodules compiler check order:
// (1) inline
// (2) file_module_name.rs
// (3) folder_module_name/mod.rs (old style)
pub mod big_module {

    // paths can be absolute (crate::item) or relative (self::item , super::item), the decision depends on the project, but as rule of thumb, use absolute
    fn path_types() {
        // absolute path
        crate::big_module::do_something_one();
        // relative path
        super::big_module::do_something_one();
        // another relative path, this function is at the same level inside the module tree than submod_children_privacity
        submod_children_privacity::public_function();
    }

    // by default, items inside CHILDREN are private to parent, must use 'pub' keyword to make them accesible
    pub mod submod_children_privacity {
        // by default, fields inside a public struct are private
        pub struct Astruct {
            field1: bool,
            field2: bool,
        }
        // by default, variants of a public enum are public
        pub enum AnEnum {
            variantA,
            variantB,
        }

        pub fn public_function() {}
    }

    // items in child modules can use the items in their ancestor modules.
    fn do_something_one() {}

    mod submod_parent_privacity {
        fn use_function_in_parent() {
            // use make shortcuts to avaiable paths in the scope
            use super::do_something_one;
            do_something_one()
        }
    }

    mod use_and_idiomatic_paths {
        // to bring function paths: 'use path::item'
        use crate::big_module;
        fn use_shortcut() {
            // when calling it: 'item::function'
            big_module::do_something_one();
        }
        // to bring other items paths (e.g. structs, enums): 'use path::other'
        use crate::big_module::submod_children_privacity::Astruct;

        // An exception to the later is whith repeated names; in such cases use the same idiomatic than bringing functions or rename with the 'as' keyword
        use std::fmt::Result;
        use std::io::Result as IoResult;

        // the later 'use' examples bring PRIVATE names, to make them public you can add 'pub' keyword
        fn re_exporting_technique() {
            pub use crate::big_module as alias;
            fn use_re_exported() {
                alias::do_something_one();
            }
        }
    }
}

// re-exporting names: pub use path:item

pub fn glob_operator() {
    //glob operator brings every that is public
    use crate::big_module::submod_children_privacity::*;
    // AnEnum is in submod_children_privacity wich is in big_module
    let x = AnEnum::variantA;
}
