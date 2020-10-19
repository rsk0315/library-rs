//! パーサ。

use std::io::{stdin, Error, Read};
use std::str::FromStr;

/// パーサ。
///
/// 文字列をパースし、所望の型に変換する。
///
/// # Examples
/// ```
/// use nekolib::utils::Parser;
///
/// use std::num::ParseIntError;
///
/// let mut p: Parser = "1 2  a\nb 3".to_string().into();
/// assert_eq!(p.next::<i32>(), Ok(1));
/// assert_eq!(p.next::<i32>(), Ok(2));
/// assert!(p.next::<i32>().is_err());
/// assert_eq!(p.next::<char>(), Ok('b'));
/// assert_eq!(p.next::<String>(), Ok("3".to_string()));
/// assert!(p.next::<i32>().is_err());
/// ```
pub struct Parser {
    buf: String,
    pos: usize,
}

impl From<String> for Parser {
    fn from(buf: String) -> Self {
        Self { buf, pos: 0 }
    }
}

impl Parser {
    pub fn from_stdin() -> Result<Self, Error> {
        let mut s = String::new();
        stdin().read_to_string(&mut s)?;
        Ok(Self::from(s))
    }
    pub fn next<T: FromStr>(&mut self) -> Result<T, <T as FromStr>::Err> {
        self.next_range(char::is_whitespace, char::is_whitespace)
    }
    pub fn next_range<T, P1, P2>(
        &mut self,
        skip: P1,
        take: P2,
    ) -> Result<T, <T as FromStr>::Err>
    where
        T: FromStr,
        P1: Fn(char) -> bool,

        P2: Fn(char) -> bool,
    {
        self.ignore_while(skip);
        self.get_while(take).parse()
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
