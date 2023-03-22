//! Various [`HashMap`] and [`BTreeMap`] extensions

use paste::paste;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
//use std::mem::ManuallyDrop;

macro_rules! tuple_borrowing {
    ($($ty:ident) +) => {
        paste! {
            trait [< HashTupleBorrow $($ty)+ >]<$($ty,)+> {
                fn tuple(&self) -> ($(&$ty,)+);
            }
            impl<$($ty,)+> [< HashTupleBorrow $($ty)+ >]<$($ty,)+> for ($($ty,)+) {
                #[allow(non_snake_case)]
                fn tuple(&self) -> ($(&$ty,)+) {
                    let ($([< entry $ty >],)+) = self;
                    ($([< entry $ty >],)+)
                }
            }
            impl<$($ty,)+> [< HashTupleBorrow $($ty)+ >]<$($ty,)+> for ($(&'_ $ty,)+) {
                #[allow(non_snake_case)]
                fn tuple(&self) -> ($(&$ty,)+) {
                    let ($([< entry $ty >],)+) = self;
                    ($([< entry $ty >],)+)
                }
            }
            impl<'r, $($ty,)+> Hash for dyn 'r + [< HashTupleBorrow $($ty)+ >]<$($ty,)+>
            where
                $($ty: 'r + Hash,)+
            {
                fn hash<H: Hasher>(&self, h: &mut H) {
                    self.tuple().hash(h);
                }
            }
            impl<'r, $($ty,)+> PartialEq for dyn 'r + [< HashTupleBorrow $($ty)+ >]<$($ty,)+>
            where
                $($ty: 'r + Eq ,)+
            {
                fn eq(self: &Self, other: &Self) -> bool {
                    self.tuple() == other.tuple()
                }
            }
            impl<'r, $($ty,)+> Eq for dyn 'r + [< HashTupleBorrow $($ty)+ >]<$($ty,)+>
            where
                $($ty: 'r + Eq ,)+
            {
            }
            impl<'r, $($ty,)+> Borrow<dyn 'r + [< HashTupleBorrow $($ty)+ >]<$($ty,)+>> for ($($ty,)+)
            where
                $($ty: 'r + Eq + Hash,)+
            {
                fn borrow(&self) -> &(dyn 'r + [< HashTupleBorrow $($ty)+ >]<$($ty,)+>) {
                    self
                }
            }


            trait [< OrdTupleBorrow $($ty)+ >]<$($ty,)+> {
                fn tuple(&self) -> ($(&$ty,)+);
            }
            impl<$($ty,)+> [< OrdTupleBorrow $($ty)+ >]<$($ty,)+> for ($($ty,)+) {
                #[allow(non_snake_case)]
                fn tuple(&self) -> ($(&$ty,)+) {
                    let ($([< entry $ty >],)+) = self;
                    ($([< entry $ty >],)+)
                }
            }
            impl<$($ty,)+> [< OrdTupleBorrow $($ty)+ >]<$($ty,)+> for ($(&'_ $ty,)+) {
                #[allow(non_snake_case)]
                fn tuple(&self) -> ($(&$ty,)+) {
                    let ($([< entry $ty >],)+) = self;
                    ($([< entry $ty >],)+)
                }
            }
            impl<'r, $($ty,)+> PartialEq for dyn 'r + [< OrdTupleBorrow $($ty)+ >]<$($ty,)+>
            where
                $($ty: 'r + Ord ,)+
            {
                fn eq(self: &Self, other: &Self) -> bool {
                    self.tuple() == other.tuple()
                }
            }
            impl<'r, $($ty,)+> Eq for dyn 'r + [< OrdTupleBorrow $($ty)+ >]<$($ty,)+>
            where
                $($ty: 'r + Ord ,)+
            {
            }
            impl<'r, $($ty,)+> PartialOrd for dyn 'r + [< OrdTupleBorrow $($ty)+ >]<$($ty,)+>
            where
                $($ty: 'r + Ord,)+
            {
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    self.tuple().partial_cmp(&other.tuple())
                }
            }
            impl<'r, $($ty,)+> Ord for dyn 'r + [< OrdTupleBorrow $($ty)+ >]<$($ty,)+>
            where
                $($ty: 'r + Ord,)+
            {
                fn cmp(&self, other: &Self) -> Ordering {
                    self.tuple().cmp(&other.tuple())
                }
            }
            impl<'r, $($ty,)+> Borrow<dyn 'r + [< OrdTupleBorrow $($ty)+ >]<$($ty,)+>> for ($($ty,)+)
            where
                $($ty: 'r + Ord,)+
            {
                fn borrow(&self) -> &(dyn 'r + [< OrdTupleBorrow $($ty)+ >]<$($ty,)+>) {
                    self
                }
            }

            /// The method provided by this extension trait allows you to look up values
            /// in a map with a tuple key — i.e.
            #[doc = ::std::concat!("`", ::std::stringify!(($($ty),+)), "`")]
            /// — by utilizing a trick where you essentially have
            #[doc = ::std::concat!("`Borrow<", ::std::stringify!(($(&$ty),+)), "> for ", ::std::stringify!(($($ty),+)), "`")]
            /// letting us look up a tuple key with owned values using a tuple of the same
            /// types with borrowed values.
            pub trait [< MapTupleExtensions $($ty)+ >]<$($ty,)+ Val> {
                /// Look up a value in a map using a tuple of owned values as a key by using
                /// a tuple of borrowed values.
                ///
                /// # Examples
                ///
                /// Because the trait implementations are macro generated, the example will only
                /// use a specific tuple size, but it works the same for all tuples up to arity
                /// 10.
                ///
                /// ```
                /// use std::collections::HashMap;
                /// use ilyvion_util::map_extensions::MapTupleExtensionsTUW;
                ///
                /// let mut hash_map = HashMap::new();
                /// hash_map.insert((16_i32, 32_u8, String::from("Hello, world!")), "first");
                /// hash_map.insert((8_i32, 16_u8, String::from("Bye, world!")), "second");
                ///
                /// assert_eq!(Some(&"first"), hash_map.get_by_tuple((&16, &32, &String::from("Hello, world!"))));
                /// assert_eq!(Some(&"second"), hash_map.get_by_tuple((&8, &16, &String::from("Bye, world!"))));
                /// ```
                ///
                /// ```
                /// use std::collections::BTreeMap;
                /// use ilyvion_util::map_extensions::MapTupleExtensionsTUW;
                ///
                /// let mut tree_map = BTreeMap::new();
                /// tree_map.insert((16_i32, 32_u8, String::from("Hello, world!")), "first");
                /// tree_map.insert((8_i32, 16_u8, String::from("Bye, world!")), "second");
                ///
                /// assert_eq!(Some(&"first"), tree_map.get_by_tuple((&16, &32, &String::from("Hello, world!"))));
                /// assert_eq!(Some(&"second"), tree_map.get_by_tuple((&8, &16, &String::from("Bye, world!"))));
                /// ```
                fn get_by_tuple(&self, ($([< entry_ $ty:lower >],)+): ($(&$ty,)+)) -> Option<&Val> {
                    self.get_by_tuple_entries($([< entry_ $ty:lower >],)+)
                }

                /// Look up a value in a map using a tuple of owned values as a key by using
                /// borrowed values.
                ///
                /// # Examples
                ///
                /// Because the trait implementations are macro generated, the example will only
                /// use a specific tuple size, but it works the same for all tuples up to arity
                /// 10.
                ///
                /// ```
                /// use std::collections::HashMap;
                /// use ilyvion_util::map_extensions::MapTupleExtensionsTUW;
                ///
                /// let mut hash_map = HashMap::new();
                /// hash_map.insert((16_i32, 32_u8, String::from("Hello, world!")), "first");
                /// hash_map.insert((8_i32, 16_u8, String::from("Bye, world!")), "second");
                ///
                /// assert_eq!(Some(&"first"), hash_map.get_by_tuple_entries(&16, &32, &String::from("Hello, world!")));
                /// assert_eq!(Some(&"second"), hash_map.get_by_tuple_entries(&8, &16, &String::from("Bye, world!")));
                /// ```
                ///
                /// ```
                /// use std::collections::BTreeMap;
                /// use ilyvion_util::map_extensions::MapTupleExtensionsTUW;
                ///
                /// let mut tree_map = BTreeMap::new();
                /// tree_map.insert((16_i32, 32_u8, String::from("Hello, world!")), "first");
                /// tree_map.insert((8_i32, 16_u8, String::from("Bye, world!")), "second");
                ///
                /// assert_eq!(Some(&"first"), tree_map.get_by_tuple_entries(&16, &32, &String::from("Hello, world!")));
                /// assert_eq!(Some(&"second"), tree_map.get_by_tuple_entries(&8, &16, &String::from("Bye, world!")));
                /// ```
                #[allow(clippy::too_many_arguments)]
                fn get_by_tuple_entries(&self, $([< entry_ $ty:lower >]: &$ty,)+) -> Option<&Val>;
            }
            impl<$($ty,)+ Val> [< MapTupleExtensions $($ty)+ >]<$($ty,)+ Val> for HashMap<($($ty,)+), Val>
            where
                ($($ty,)+): Eq + Hash,
                $($ty: Eq + Hash,)+
            {
                #[allow(nonstandard_style)]
                #[inline]
                fn get_by_tuple_entries(&self, $([< entry $ty >]: &$ty,)+) -> Option<&Val> {
                    let k: &dyn [< HashTupleBorrow $($ty)+ >]<$($ty,)+> = &($([< entry $ty >],)+);
                    self.get(k)
                }
            }
            impl<$($ty,)+ Val> [< MapTupleExtensions $($ty)+ >]<$($ty,)+ Val> for BTreeMap<($($ty,)+), Val>
            where
                ($($ty,)+): Ord,
                $($ty: Ord,)+
            {
                #[allow(nonstandard_style)]
                #[inline]
                fn get_by_tuple_entries(&self, $([< entry $ty >]: &$ty,)+) -> Option<&Val> {
                    let k: &dyn [< OrdTupleBorrow $($ty)+ >]<$($ty,)+> = &($([< entry $ty >],)+);
                    self.get(k)
                }
            }
        }
    };
}

tuple_borrowing!(T U);

tuple_borrowing!(T U W);

tuple_borrowing!(T U W X);

tuple_borrowing!(T U W X Y);

tuple_borrowing!(T U W X Y Z);

tuple_borrowing!(T U W X Y Z A);

tuple_borrowing!(T U W X Y Z A B);

tuple_borrowing!(T U W X Y Z A B C);

tuple_borrowing!(T U W X Y Z A B C D);
