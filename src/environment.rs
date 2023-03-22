//! Provides a macro to help with using environment variables more ergonomically.

use std::env::VarError;
use std::error::Error;
use thiserror::Error;

// These re-exports are macro implementation details and not part of the
// public API
#[doc(hidden)]
pub use dotenvy as __dotenvy;
#[doc(hidden)]
pub use once_cell as __once_cell;
#[doc(hidden)]
pub use paste as __paste;

// Note: since `rustdoc` is currently unable to handle a `#[doc(hidden)]` not
// infecting a `#[doc(inline)] pub use ...`, we get our hands dirty and hide
// the stuff manually.
//
// "Stolen" from
// <https://github.com/danielhenrymantilla/rust-uninit/blob/43a115e5d6a8f8f45c726365407c2dc987450abe/src/read/mod.rs#L169>

macro_rules! well_located_public_macro {(
    $(
        #[doc = $doc:expr]
    )*
    pub
    macro_rules! $macro_name:ident {
        $(
            $input:tt => $output:tt
        );+ $(;)?
    }
) => (
    #[cfg(not(feature = "better-docs"))]
    #[doc(hidden)]
    #[macro_export]
    macro_rules! $macro_name {
        $(
            $input => $output;
        )+
    }

    #[cfg(not(feature = "better-docs"))]
    pub use $macro_name;

    $(
        #[doc = $doc]
    )*
    #[cfg(feature = "better-docs")]
    #[rustc_macro_transparency = "semitransparent"]
    pub
    macro $macro_name {
        $(
            $input => $output,
        )+
    }
)}

well_located_public_macro! {
    /// This macro declares a module that contains utility functions for reading the given environment
    /// variables. For a given environment variable, two functions are generated, one that is named
    /// the same as the declared environment variable that panics if the environment variable isn't
    /// present and one prefixed with `try_` that returns a `Result` (see the return value of
    /// [`std::env::var`] for details on the error.)
    ///
    /// The syntax of the macro is:
    /// ```rust
    /// # /*
    /// define_environment! {
    ///     mod $environment_module_name {
    ///         $environment_variable_name1();
    ///         $environment_variable_name2();
    ///     }
    /// }
    /// # */
    /// ```
    ///
    /// Both the module name and the variable names accept visibility modifiers like `pub` and
    /// `pub(crate)`. The variable name should be given in lower case, and will become the name of
    /// the panicking utility function discussed above. This name will also be the name of the
    /// environment variable that is read, but converted to uppercase. (I.e. a variable name of `path`
    /// will attempt to read an environment variable named `PATH`.)
    ///
    /// # Examples
    /// ```rust
    /// use ilyvion_util::environment::define_environment;
    ///
    /// define_environment! {
    ///     pub(crate) mod environment {
    ///         pub path();
    ///     }
    /// }
    ///
    /// println!("{}", environment::path());
    /// ```
    ///
    pub macro_rules! define_environment {
        (
            $($vis:vis $environment_variable:ident()$(: $ty:ty)?);+ $(;)?
        ) => {
            /// Convenience method to load environment variables from a `.env` file.
            /// This function calls [`dotenvy::dotenv()`](dotenvy::dotenv)`.ok()`
            pub fn init_dotenv() {
                $crate::environment::__dotenvy::dotenv().ok();
            }
            $(
                $crate::environment::__paste::paste! {
                    #[allow(dead_code)]
                    static [<$environment_variable:upper>]: $crate::environment::__once_cell::sync::Lazy<
                        ::std::result::Result<::std::string::String, ::std::env::VarError>
                    > = $crate::environment::__once_cell::sync::Lazy::new(|| {
                        ::std::env::var(stringify!($environment_variable))
                    });

                    /// Returns the value of the environment variable `
                    #[doc = ::std::stringify!([<$environment_variable:upper>])]
                    /// `.
                    ///
                    /// # Panics
                    ///
                    /// This function will panic if the environment variable isn't set.
                    #[allow(dead_code)]
                    #[track_caller]
                    $vis fn $environment_variable() -> &'static str {
                        [<$environment_variable:upper>].as_deref().expect(concat!(
                            "environment variable '",
                            ::std::stringify!([<$environment_variable:upper>]),
                            "' to be present"))
                    }

                    /// Returns the value of the environment variable `
                    #[doc = ::std::stringify!([<$environment_variable:upper>])]
                    /// `. If the environment variable isn't set, this function will
                    /// return `None`.
                    #[allow(dead_code)]
                    #[track_caller]
                    $vis fn [< try_ $environment_variable >]() -> ::std::option::Option<&'static str> {
                        [<$environment_variable:upper>].as_deref().ok()
                    }

                    $(
                        #[track_caller]
                        $vis fn [< $environment_variable _ $ty>]() -> $ty {
                            [< try_ $environment_variable _ $ty>]().expect(concat!(
                                "environment variable '",
                                ::std::stringify!([<$environment_variable:upper>]),
                                "' to be present and be a valid value for type ",
                                ::std::stringify!($ty),
                            ))
                        }

                        #[track_caller]
                        $vis fn [< try_ $environment_variable _ $ty>]() -> ::std::result::Result<
                            $ty,
                            $crate::environment::EnvironmentConverterError<
                                <$ty as $crate::environment::EnvironmentConverter>::ConversionError
                            >
                        > {
                            let var = [<$environment_variable:upper>]
                                .as_deref()
                                .map_err(::std::clone::Clone::clone)?;
                            match <$ty as $crate::environment::EnvironmentConverter>::
                                try_convert(stringify!($environment_variable), var) {
                                ::std::result::Result::Ok(var) => Ok(var),
                                ::std::result::Result::Err(err) => Err(
                                    $crate::environment::EnvironmentConverterError::
                                        <<$ty as $crate::environment::EnvironmentConverter>::ConversionError>::ConversionError(err)
                                ),
                            }

                        }
                    )?
                }
            )+
        };
    }
}

/// The error type used in [`EnvironmentConverter`] implementations
#[derive(Error, Debug)]
pub enum EnvironmentConverterError<E> {
    /// The error variant for operations interacting with environment variables
    VarError(#[from] VarError),
    /// The error variant for conversions
    ConversionError(E),
}

#[doc(hidden)]
pub trait EnvironmentConverter: Sized {
    type ConversionError: Error;

    fn try_convert(name: &'static str, value: &'static str) -> Result<Self, Self::ConversionError>;
}

impl EnvironmentConverter for bool {
    type ConversionError = ConvertBoolError;

    fn try_convert(name: &'static str, value: &'static str) -> Result<Self, Self::ConversionError> {
        Ok(match value {
            "on" | "true" | "yes" | "1" => true,
            "off" | "false" | "no" | "0" => false,
            _ => {
                return Err(ConvertBoolError {
                    name,
                    unexpected_value: value,
                })
            }
        })
    }
}

/// The error type for converting to `bool`.
#[derive(Clone, Copy, Debug, Error)]
#[error(
    "Invalid bool value for environment variable '{name}': {unexpected_value}. \
    Valid values are 'on', 'true', 'yes' and '1' for true, \
    'off', 'false', 'no' and '0' for false"
)]
pub struct ConvertBoolError {
    name: &'static str,
    unexpected_value: &'static str,
}