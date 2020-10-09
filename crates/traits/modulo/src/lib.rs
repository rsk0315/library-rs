//! modint 用のトレイト。
//!
//! C++ で言うところの `static` 変数の役割のものが欲しいので、
//! `mod` を返すクラスを `PhantomData` に入れるように実装する（仮）。

use std::fmt::Debug;

/// 法を返す関数を持つ。
///
/// # Example
/// ```
/// use nekolib::traits::Mod;
/// use nekolib::impl_mod_int;
/// use nekolib::utils::ModInt;
///
/// impl_mod_int! { Mod119l23p1 => 998_244_353_i64 }
/// type Mi = ModInt<Mod119l23p1>;
///
/// assert_eq!(Mi::new(2) - Mi::new(3), Mi::new(998_244_352));
/// assert_eq!(Mi::new(1) / Mi::new(2), Mi::new(499_122_177));
/// ```
///
/// 法が実行時に決まる場合は、`static` とかを使うしかない気がする。
/// `lazy_static` に依存しないようにはしたい。
/// ```
/// use std::sync::{Arc, Mutex};
///
/// use lazy_static::lazy_static;
///
/// use nekolib::traits::Mod;
/// use nekolib::impl_mod_int;
/// use nekolib::utils::ModInt;
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
/// assert_eq!(Mi::new(20) + Mi::new(4), Mi::new(0));
/// assert_eq!(Mi::new(1) / Mi::new(7), Mi::new(7));
/// ```
pub trait Mod: Clone + Copy + Debug + Eq + PartialEq {
    fn get() -> i64;
}

#[macro_export]
macro_rules! impl_mod_int {
    ( $i:ident => $m:expr ) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        struct $i {}
        impl Mod for $i {
            fn get() -> i64 { $m }
        }
    };
    ( $( $i:ident => $m:expr, )* ) => { $( impl_mod_int!($i => $m); )* };
    ( $( $i:ident => $m:expr ),* ) => { $( impl_mod_int!($i => $m); )* };
}
