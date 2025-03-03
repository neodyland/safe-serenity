//! A set of macros for easily working with internals.

#[cfg(any(feature = "model", feature = "utils"))]
macro_rules! cdn {
    ($e:expr) => {
        concat!("https://cdn.discordapp.com", $e)
    };
    ($e:expr, $($rest:tt)*) => {
        format!(cdn!($e), $($rest)*)
    };
}

#[cfg(feature = "http")]
macro_rules! api {
    ($e:expr) => {
        concat!("https://discord.com/api/v10", $e)
    };
    ($e:expr, $($rest:tt)*) => {
        format!(api!($e), $($rest)*)
    };
}

#[cfg(feature = "http")]
macro_rules! status {
    ($e:expr) => {
        concat!("https://status.discord.com/api/v2", $e)
    };
}

#[cfg(all(feature = "cache", feature = "client"))]
macro_rules! if_cache {
    ($e:expr) => {
        $e
    };
}

#[cfg(all(not(feature = "cache"), feature = "client"))]
macro_rules! if_cache {
    ($e:expr) => {
        None
    };
}
/// The `enum_number!` macro generates `From` implementations to convert between values and the
/// enum which can then be utilized by `serde` with `#[serde(from = "u8", into = "u8")]`.
///
/// When defining the enum like this:
/// ```ignore
/// enum_number! {
///     /// The `Foo` enum
///     #[derive(Clone, Copy, Deserialize, Serialize)]
///     #[serde(from = "u8", into = "u8")]
///     pub enum Foo {
///         /// First
///         Aah = 1,
///         /// Second
///         Bar = 2,
///         _ => Unknown(u8),
///     }
/// }
/// ```
///
/// Code like this will be generated:
///
/// ```
/// # use serde::{Deserialize, Serialize};
/// #
/// /// The `Foo` enum
/// #[derive(Clone, Copy, Deserialize, Serialize)]
/// #[serde(from = "u8", into = "u8")]
/// pub enum Foo {
///     /// First
///     Aah,
///     /// Second,
///     Bar,
///     /// Variant value is unknown.
///     Unknown(u8),
/// }
///
/// impl From<u8> for Foo {
///     fn from(value: u8) -> Self {
///         match value {
///             1 => Self::Aah,
///             2 => Self::Bar,
///             unknown => Self::Unknown(unknown),
///         }
///     }
/// }
///
/// impl From<Foo> for u8 {
///     fn from(value: Foo) -> Self {
///         match value {
///             Foo::Aah => 1,
///             Foo::Bar => 2,
///             Foo::Unknown(unknown) => unknown,
///         }
///     }
/// }
/// ```
macro_rules! enum_number {
    (
        $(#[$outer:meta])*
        $vis:vis enum $Enum:ident {
            $(
                $(#[doc = $doc:literal])*
                $(#[cfg $($cfg:tt)*])?
                $(#[default $($dummy:tt)?])?
                $Variant:ident = $value:literal,
            )*
            _ => Unknown($T:ty),
        }
    ) => {
        $(#[$outer])*
        $vis enum $Enum {
            $(
                $(#[doc = $doc])*
                $(#[cfg $($cfg)*])?
                $(#[default $($dummy:tt)?])?
                $Variant,
            )*
            /// Variant value is unknown.
            Unknown($T),
        }

        impl From<$T> for $Enum {
            fn from(value: $T) -> Self {
                match value {
                    $($(#[cfg $($cfg)*])? $value => Self::$Variant,)*
                    unknown => Self::Unknown(unknown),
                }
            }
        }

        impl From<$Enum> for $T {
            fn from(value: $Enum) -> Self {
                match value {
                    $($(#[cfg $($cfg)*])? $Enum::$Variant => $value,)*
                    $Enum::Unknown(unknown) => unknown,
                }
            }
        }
    };
}

/// The macro forwards the generation to the `bitflags::bitflags!` macro and implements the default
/// (de)serialization for Discord's bitmask values.
///
/// The flags are created with `T::from_bits_truncate` for the deserialized integer value.
///
/// Use the `bitflags::bitflags! macro directly if a different serde implementation is required.
macro_rules! bitflags {
    (
        $(#[$outer:meta])*
        $vis:vis struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:ident = $value:expr;
            )*
        }
    ) => {
        bitflags::bitflags! {
            #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
            $(#[$outer])*
            $vis struct $BitFlags: $T {
                $(
                    $(#[$inner $($args)*])*
                    const $Flag = $value;
                )*
            }
        }

        bitflags!(__impl_serde $BitFlags: $T);
    };
    (__impl_serde $BitFlags:ident: $T:tt) => {
        impl<'de> serde::de::Deserialize<'de> for $BitFlags {
            fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
                Ok(Self::from_bits_truncate(<$T>::deserialize(deserializer)?))
            }
        }

        impl serde::ser::Serialize for $BitFlags {
            fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
                self.bits().serialize(serializer)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::json::{assert_json, json};

    #[test]
    fn enum_number() {
        enum_number! {
            #[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
            #[serde(from = "u8", into = "u8")]
            pub enum T {
                /// AAA
                A = 1,
                /// BBB
                B = 2,
                /// CCC
                C = 3,
                _ => Unknown(u8),
            }
        }

        assert_json(&T::A, json!(1));
        assert_json(&T::B, json!(2));
        assert_json(&T::C, json!(3));
        assert_json(&T::Unknown(123), json!(123));
    }
}
