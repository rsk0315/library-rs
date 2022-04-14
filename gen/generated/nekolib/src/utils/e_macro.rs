/// デバッグ用マクロ。
///
/// debug 時は `eprintln!(...)` に展開され、release 時は `()` に展開される。
///
/// # Examples
/// ```
/// use nekolib::e;
///
/// e!("{:?}", (0..100).collect::<Vec<_>>());
/// ```
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! e {
    ( $($arg:tt)* ) => { eprintln!($($arg)*) };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! e {
    ( $($arg:tt)* ) => {
        ()
    };
}
