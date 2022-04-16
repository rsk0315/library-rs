//! 形式つき出力。

/// 形式つき出力（スペース区切り）。
///
/// # Examples
/// ```
/// use nekolib::utils::{SpaceSep, PerLine};
///
/// let a = vec![13, 5, 30, 27, 6];
/// let b = vec!['a', 'b', 'c'];
///
/// assert_eq!(format!("{}", SpaceSep(&a)), "13 5 30 27 6");
/// assert_eq!(format!("{:02}", SpaceSep(&a)), "13 05 30 27 06");
/// assert_eq!(format!("{:#04x}", SpaceSep(&a[..3])), "0x0d 0x05 0x1e");
/// assert_eq!(format!("{:?}", SpaceSep(&b)), "'a' 'b' 'c'");
/// assert_eq!(format!("{}", PerLine(&b)), "a\nb\nc");
/// ```
pub struct SpaceSep<'a, D: ?Sized>(pub &'a D);

/// 形式つき出力（改行区切り）。
///
/// # Examples
/// ```
/// use nekolib::utils::{SpaceSep, PerLine};
///
/// let a = vec![13, 5, 30, 27, 6];
/// let b = vec!['a', 'b', 'c'];
///
/// assert_eq!(format!("{}", SpaceSep(&a)), "13 5 30 27 6");
/// assert_eq!(format!("{:02}", SpaceSep(&a)), "13 05 30 27 06");
/// assert_eq!(format!("{:#04x}", SpaceSep(&a[..3])), "0x0d 0x05 0x1e");
/// assert_eq!(format!("{:?}", SpaceSep(&b)), "'a' 'b' 'c'");
/// assert_eq!(format!("{}", PerLine(&b)), "a\nb\nc");
/// ```
pub struct PerLine<'a, D: ?Sized>(pub &'a D);

use std::fmt;

macro_rules! impl_fmt {
    ( $( ($fmt:ident, $fn:ident), )* ) => { $(
        fn $fn<I>(it: I, sep: char, f: &mut fmt::Formatter) -> fmt::Result
        where
            I: IntoIterator,
            <I as IntoIterator>::Item: fmt::$fmt,
        {
            for (i, x) in it.into_iter().enumerate() {
                if i > 0 {
                    write!(f, "{}", sep)?;
                }
                fmt::$fmt::fmt(&x, f)?;
            }
            Ok(())
        }

        impl<'a, D: 'a> fmt::$fmt for SpaceSep<'a, D>
        where
            D: ?Sized,
            &'a D: IntoIterator,
            <&'a D as IntoIterator>::Item: fmt::$fmt,
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                $fn(self.0, ' ', f)
            }
        }

        impl<'a, D: 'a> fmt::$fmt for PerLine<'a, D>
        where
            D: ?Sized,
            &'a D: IntoIterator,
            <&'a D as IntoIterator>::Item: fmt::$fmt,
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                $fn(self.0, '\n', f)
            }
        }
    )* };
}

impl_fmt! {
    (Display, join_display),
    (Debug, join_debug),
    (Octal, join_octal),
    (LowerHex, join_lower_hex),
    (UpperHex, join_upper_hex),
    (Pointer, join_pointer),
    (Binary, join_binary),
    (LowerExp, join_lower_exp),
    (UpperExp, join_upper_exp),
}
