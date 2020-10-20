//! パーサ。

use std::io::{stdin, Error, Read};

/// パーサ。
///
/// 文字列をパースし、所望の型に変換する。
///
/// # Examples
/// ```
/// use nekolib::utils::Scanner;
///
/// use std::num::ParseIntError;
///
/// let mut p: Scanner = "1 2  a\nb 3".to_string().into();
/// assert_eq!(p.next::<i32>(), Ok(1));
/// assert_eq!(p.next::<i32>(), Ok(2));
/// assert!(p.next::<i32>().is_err());
/// assert_eq!(p.next::<char>(), Ok('b'));
/// assert_eq!(p.next::<String>(), Ok("3".to_string()));
/// assert!(p.next::<i32>().is_err());
/// ```
pub struct Scanner {
    buf: String,
    pos: usize,
}

impl From<String> for Scanner {
    fn from(buf: String) -> Self {
        Self { buf, pos: 0 }
    }
}

impl Scanner {
    pub fn from_stdin() -> Result<Self, Error> {
        let mut s = String::new();
        stdin().read_to_string(&mut s)?;
        Ok(Self::from(s))
    }
    pub fn next<T: Scan>(&mut self) -> Result<T, <T as Scan>::Err> {
        let (x, endpos) = T::scan(&self.buf[self.pos..]);
        self.pos += endpos;
        x
    }
    pub fn next_m1<T>(&mut self) -> Result<T, <T as Scan>::Err>
    where
        T: Scan + std::ops::Sub<Output = T> + std::convert::From<u8>,
    {
        self.next::<T>().map(|x| x - 1_u8.into())
    }
    pub fn next_n<T>(&mut self, n: usize) -> Result<Vec<T>, <T as Scan>::Err>
    where
        T: Scan + Clone,
    {
        let mut res = vec![];
        for _ in 0..n {
            res.push(self.next::<T>()?);
        }
        Ok(res)
    }
    pub fn get_while<P>(&mut self, pat: P) -> &str
    where
        P: Fn(char) -> bool,
    {
        let s = &self.buf[self.pos..];
        let len = s.find(|c| pat(c)).unwrap_or(s.len());
        let s = &s[..len];
        self.pos += len;
        s
    }
    pub fn get_line(&mut self) -> &str {
        let s = &self.buf[self.pos..];
        let len = s.find('\n').map(|i| i + 1).unwrap_or(s.len());
        let s = &s[..len];
        self.pos += len;
        s
    }
    pub fn ignore(&mut self) {
        self.ignore_while(char::is_whitespace);
    }
    pub fn ignore_while<P>(&mut self, pat: P)
    where
        P: Fn(char) -> bool,
    {
        self.get_while(|c| !pat(c));
    }
}

pub trait Scan: Sized {
    type Err: std::error::Error;
    fn scan(buf: &str) -> (Result<Self, Self::Err>, usize);
}

macro_rules! impl_scan_int {
    ($t:ty) => {
        impl Scan for $t {
            type Err = std::num::ParseIntError;
            fn scan(buf: &str) -> (Result<Self, Self::Err>, usize) {
                let start = buf.find(|c| !char::is_whitespace(c)).unwrap_or(buf.len());
                let buf = &buf[start..];
                let len = buf.find(char::is_whitespace).unwrap_or(buf.len());
                let buf = &buf[..len];
                (buf.parse(), start + len)
            }
        }
    };
    ( $( $t:ty, )* ) => { $( impl_scan_int!($t); )* }
}

impl_scan_int! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

macro_rules! impl_scan_float {
    ($t:ty) => {
        impl Scan for $t {
            type Err = std::num::ParseFloatError;
            fn scan(buf: &str) -> (Result<Self, Self::Err>, usize) {
                let start = buf.find(|c| !char::is_whitespace(c)).unwrap_or(buf.len());
                let buf = &buf[start..];
                let len = buf.find(char::is_whitespace).unwrap_or(buf.len());
                let buf = &buf[..len];
                (buf.parse(), start + len)
            }
        }
    };
    ( $( $t:ty, )* ) => { $( impl_scan_float!($t); )* }
}

impl_scan_float! { f32, f64, }

impl Scan for String {
    type Err = std::convert::Infallible;
    fn scan(buf: &str) -> (Result<Self, Self::Err>, usize) {
        let start = buf.find(|c| !char::is_whitespace(c)).unwrap_or(buf.len());
        let buf = &buf[start..];
        let len = buf.find(char::is_whitespace).unwrap_or(buf.len());
        let buf = &buf[..len];
        (Ok(buf.to_string()), start + len)
    }
}

#[derive(std::fmt::Debug, Eq, PartialEq)]
pub struct ScanTupleError();

impl std::fmt::Display for ScanTupleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error: while parsing tuple")
    }
}

impl std::error::Error for ScanTupleError {}

macro_rules! impl_scan_tuple {
    (
        ( $( ($i:ident: $t:tt) ),+ )
    ) => {
        impl< $( $t ),+> Scan for ( $( $t ),+ )
        where
            $( $t: Scan, )+
        {
            type Err = ScanTupleError;
            fn scan(buf: &str) -> (Result<Self, ScanTupleError>, usize) {
                let mut len = 0;
                $(
                    let (x, i) = $t::scan(&buf[len..]);
                    len += i;
                    let $i = match x {
                        Ok(x) => x,
                        Err(_) => return (Err(ScanTupleError()), len),
                    }
                );* ;
                (Ok(( $( $i ),+ )), len)
            }
        }
    };
}

impl_scan_tuple! { ((a: A), (b: B)) }
impl_scan_tuple! { ((a: A), (b: B), (c: C)) }
impl_scan_tuple! { ((a: A), (b: B), (c: C), (d: D)) }
impl_scan_tuple! { ((a: A), (b: B), (c: C), (d: D), (e: E)) }
impl_scan_tuple! { ((a: A), (b: B), (c: C), (d: D), (e: E), (f: F)) }
impl_scan_tuple! { ((a: A), (b: B), (c: C), (d: D), (e: E), (f: F), (g: G))}
impl_scan_tuple! {(
    (a: A), (b: B), (c: C), (d: D), (e: E), (f: F), (g: G), (h: H)
)}
impl_scan_tuple! {(
    (a: A), (b: B), (c: C), (d: D), (e: E), (f: F), (g: G), (h: H), (i: I)
)}
impl_scan_tuple! {(
    (a: A), (b: B), (c: C), (d: D), (e: E), (f: F), (g: G), (h: H), (i: I),
    (j: J)
)}
impl_scan_tuple! {(
    (a: A), (b: B), (c: C), (d: D), (e: E), (f: F), (g: G), (h: H), (i: I),
    (j: J), (k: K)
)}
impl_scan_tuple! {(
    (a: A), (b: B), (c: C), (d: D), (e: E), (f: F), (g: G), (h: H), (i: I),
    (j: J), (k: K), (l: L)
)}

// 結局、input! が使いやすくない？ また考える
