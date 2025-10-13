//! Constants associated with each target.

// Replaces the #[else] gate with #[cfg(not(any(…)))] of all the other gates.
// This ensures that they must be mutually exclusive and do not have precedence
// like cfg_select!.
macro cfg_unordered(
    $(#[cfg($cfg:meta)] $os:item)*
    #[else] $fallback:item
) {
    $(#[cfg($cfg)] $os)*
    #[cfg(not(any($($cfg),*)))] $fallback
}

cfg_unordered! {
    #[cfg(target_os = "dos")]
    pub mod os {
        pub const FAMILY: &str = "";
        pub const OS: &str = "dos";
        pub const DLL_PREFIX: &str = "";
        pub const DLL_SUFFIX: &str = "";
        pub const DLL_EXTENSION: &str = "";
        pub const EXE_SUFFIX: &str = ".com";
        pub const EXE_EXTENSION: &str = "com";
    }

    #[else]
    pub mod os {
        pub const FAMILY: &str = "";
        pub const OS: &str = "";
        pub const DLL_PREFIX: &str = "";
        pub const DLL_SUFFIX: &str = "";
        pub const DLL_EXTENSION: &str = "";
        pub const EXE_SUFFIX: &str = "";
        pub const EXE_EXTENSION: &str = "";
    }
}
