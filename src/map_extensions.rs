//! Various [`HashMap`] and [`BTreeMap`] extensions

use paste::paste;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::mem::ManuallyDrop;

macro_rules! map_tuple_extensions {
    ($($ty:ident) +) => {

        paste! {
            /// Because there is no generic safe way to make a reference to a tuple of values
            /// when what you have is individual references to values without having to
            /// to clone said values, a process that seems wasteful and unnecessary when all
            /// you are using them for is for looking up values in a map, we do some unsafe
            /// (but sound¹!) trickery to turn individual references into a reference to a tuple
            /// and then use that to look up a value.
            ///
            /// ¹ Both according to my own understanding and by Miri
            pub trait [< MapTupleExtensions $($ty)+ >] <$($ty,)+ V> {
                /// Look up a value in the map with the tuple key based on the individual
                /// `entry*` references
                #[allow(nonstandard_style)]
                #[allow(clippy::too_many_arguments)]
                fn get_by_tuple_entries(&self, $(paste!{ [<entry $ty>] }: &$ty),+) -> Option<&V>;
            }

            impl<$($ty,)+ V> [< MapTupleExtensions $($ty)+ >] <$($ty,)+ V> for HashMap<($($ty,)+), V>
            where
                ($($ty,)+): Eq + Hash,
            {
                #[allow(nonstandard_style)]
                #[inline]
                fn get_by_tuple_entries(&self,  $(paste!{ [<entry $ty>] }: &$ty),+) -> std::option::Option<&V> {
                    let k = [< make_tuple_key$($ty)+ >]($(paste!{ [<entry $ty>] }),+);
                    self.get(&k)
                }
            }

            impl<$($ty,)+ V> [< MapTupleExtensions $($ty)+ >] <$($ty,)+ V> for BTreeMap<($($ty,)+), V>
            where
                ($($ty,)+): Ord,
            {
                #[allow(nonstandard_style)]
                #[inline]
                fn get_by_tuple_entries(&self,  $(paste!{ [<entry $ty>] }: &$ty),+) -> std::option::Option<&V> {
                    let k = [< make_tuple_key$($ty)+ >]($(paste!{ [<entry $ty>] }),+);
                    self.get(&k)
                }
            }

            #[allow(unsafe_code)]
            #[allow(nonstandard_style)]
            #[allow(clippy::too_many_arguments)]
            #[inline]
            fn [< make_tuple_key$($ty)+ >] <$($ty,)+>($(paste!{ [<entry $ty>] }: &$ty),+) -> ManuallyDrop<($($ty,)+)> {
                let k = unsafe {
                    // Copy the type values of the `entry*`s so that they are adjacent.
                    // This makes a tuple backed by the same data as the `entry*`s.
                    ($(std::ptr::read(paste!{ [<entry $ty>] }),)+)
                };

                // Make sure not to drop the values, even if we panic. The caller
                // or whoever actually owns the `entry*`s will be responsible
                // for dropping them.
                ManuallyDrop::new(k)
            }
        }
    }
}

map_tuple_extensions!(T U);

map_tuple_extensions!(T U W);

map_tuple_extensions!(T U W X);

map_tuple_extensions!(T U W X Y);

map_tuple_extensions!(T U W X Y Z);

map_tuple_extensions!(T U W X Y Z A);

map_tuple_extensions!(T U W X Y Z A B);

map_tuple_extensions!(T U W X Y Z A B C);

map_tuple_extensions!(T U W X Y Z A B C D);
