//! 型に紐づく値。
//!
//! C++ で言うところの `static` 変数の役割のものが欲しい。
//! その値の型 `PhantomData` に入れるように実装する（仮）。

/// 値を返す関数を持つ。
///
/// # Example
/// ```
/// use nekolib::traits::AssocVal;
/// use nekolib::impl_mod_int;
/// use nekolib::math::ModInt;
///
/// impl_mod_int! { Mod119l23p1 => 998_244_353_i64 }
/// type Mi = ModInt<Mod119l23p1>;
///
/// assert_eq!(Mi::from(2) - Mi::from(3), Mi::from(998_244_352));
/// assert_eq!(Mi::from(1) / Mi::from(2), Mi::from(499_122_177));
/// ```
///
/// 法が実行時に決まる場合は、`static` とかを使うしかない気がする。
/// `lazy_static` に依存しないようにはしたい。
/// ```
/// use std::sync::{Arc, Mutex};
///
/// use lazy_static::lazy_static;
///
/// use nekolib::traits::AssocVal;
/// use nekolib::impl_mod_int;
/// use nekolib::math::ModInt;
///
/// lazy_static! {
///     static ref MOD: Arc<Mutex<i64>> = Arc::new(Mutex::new(0));
/// }
/// let b = 24;
/// *MOD.lock().unwrap() = b;
///
/// impl_mod_int! { ModRunTime => *MOD.lock().unwrap() }
/// type Mi = ModInt<ModRunTime>;
///
/// assert_eq!(Mi::from(20) + Mi::from(4), Mi::from(0));
/// assert_eq!(Mi::from(1) / Mi::from(7), Mi::from(7));
/// ```
pub trait AssocVal<T> {
    fn get() -> T;
}

#[macro_export]
macro_rules! impl_assoc_val {
    ( $i:ident<$t:ty> => $e:expr ) => {
        #[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
        struct $i { _t: std::marker::PhantomData<$t> }
        impl AssocVal<$t> for $i {
            fn get() -> $t { $e }
        }
    };
    ( $( $i:ident<$t:ty> => $m:expr, )* ) => { $( impl_assoc_val!($i<$t> => $m); )* };
    ( $( $i:ident<$t:ty> => $m:expr ),* ) => { $( impl_assoc_val!($i<$t> => $m); )* };
}
