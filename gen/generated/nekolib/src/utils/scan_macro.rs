#![cfg(ignore)]
// #![feature(once_cell)]

use std::any::type_name;
use std::fmt::Debug;
use std::io::{stdin, BufRead, BufReader, Stdin};
use std::marker::PhantomData;
use std::str::FromStr;
use std::sync::{LazyLock, Mutex};

pub static STDIN_SOURCE: LazyLock<Mutex<AutoSource<BufReader<Stdin>>>> =
    LazyLock::new(|| Mutex::new(AutoSource::new(BufReader::new(stdin()))));

pub trait Source<R: BufRead> {
    fn next_token(&mut self) -> Option<String>;
    fn next_token_unwrap(&mut self) -> String { self.next_token().unwrap() }
}

impl<R: BufRead, S: Source<R>> Source<R> for &'_ mut S {
    fn next_token(&mut self) -> Option<String> { (*self).next_token() }
}

pub type AutoSource<R> = OnceSource<R>;

pub struct OnceSource<R: BufRead> {
    tokens: std::vec::IntoIter<String>,
    _phantom: PhantomData<R>,
}

impl<R: BufRead> OnceSource<R> {
    pub fn new(mut source: R) -> Self {
        let mut context = "".to_owned();
        source.read_to_string(&mut context).unwrap();
        let tokens: Vec<_> =
            context.split_whitespace().map(|s| s.to_owned()).collect();
        Self {
            tokens: tokens.into_iter(),
            _phantom: PhantomData,
        }
    }
}

impl<R: BufRead> Source<R> for OnceSource<R> {
    fn next_token(&mut self) -> Option<String> { self.tokens.next() }
}

impl<'a> From<&'a str> for OnceSource<BufReader<&'a [u8]>> {
    fn from(s: &'a str) -> Self {
        OnceSource::new(BufReader::new(s.as_bytes()))
    }
}

pub trait Readable {
    type Output;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Self::Output;
}

impl<T: FromStr> Readable for T
where
    T::Err: Debug,
{
    type Output = T;
    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> T {
        let token = source.next_token_unwrap();
        match token.parse() {
            Ok(v) => v,
            Err(e) => panic!(
                "`{input}` `{ty}` `{err:?}`",
                input = token,
                ty = type_name::<T>(),
                err = e
            ),
        }
    }
}

#[macro_export]
macro_rules! scan {
    // terminator
    (@from [$source:expr] @rest) => {};

    // parse mutability
    (@from [$source:expr] @rest mut $($rest:tt)*) => {
        $crate::scan! {
            @from [$source]
            @mut [mut]
            @rest $($rest)*
        }
    };
    (@from [$source:expr] @rest $($rest:tt)*) => {
        $crate::scan! {
            @from [$source]
            @mut []
            @rest $($rest)*
        }
    };

    // parse variable pattern
    (@from [$source:expr] @mut [$($mut:tt)?] @rest $var:tt: $($rest:tt)*) => {
        $crate::scan! {
            @from [$source]
            @mut [$($mut)?]
            @var $var
            @kind []
            @rest $($rest)*
        }
    };

    // parse kind (type)
    (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @kind [$($kind:tt)*] @rest) => {
        let $($mut)? $var = $crate::read_value!(@source [$source] @kind [$($kind)*]);
    };
    (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @kind [$($kind:tt)*] @rest, $($rest:tt)*) => {
        $crate::scan!(@from [$source] @mut [$($mut)?] @var $var @kind [$($kind)*] @rest);
        $crate::scan!(@from [$source] @rest $($rest)*);
    };
    (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @kind [$($kind:tt)*] @rest $tt:tt $($rest:tt)*) => {
        $crate::scan!(@from [$source] @mut [$($mut)?] @var $var @kind [$($kind)* $tt] @rest $($rest)*);
    };

    (from $source:expr, $($rest:tt)*) => {
        #[allow(unused_variables, unused_mut)]
        let mut s = $source;
        $crate::scan! {
            @from [&mut s]
            @rest $($rest)*
        }
    };
    ($($rest:tt)*) => {
        let mut locked_stdin = $crate::STDIN_SOURCE.lock().unwrap();
        $crate::scan! {
            @from [&mut *locked_stdin]
            @rest $($rest)*
        }
        drop(locked_stdin);
    };
}

#[macro_export]
macro_rules! read_value {
    // array and variable-length array
    (@source [$source:expr] @kind [[$($kind:tt)*]]) => {
        $crate::read_value!(@vec @source [$source] @kind [] @rest $($kind)*)
    };
    (@vec @source [$source:expr] @kind [$($kind:tt)*] @rest) => {{
        let len = <usize as $crate::utils::scan_macro::Readable>::read($source);
        $crate::read_value!(@source [$source] @kind [[$($kind)*; len]])
    }};
    (@vec @source [$source:expr] @kind [$($kind:tt)*] @rest ; const $($rest:tt)*) => {
        $crate::read_value!(@array @source [$source] @kind [$($kind)*] @len [$($rest)*])
    };
    (@vec @source [$source:expr] @kind [$($kind:tt)*] @rest ; $($rest:tt)*) => {
        $crate::read_value!(@vec @source [$source] @kind [$($kind)*] @len [$($rest)*])
    };
    (@vec @source [$source:expr] @kind [$($kind:tt)*] @rest $tt:tt $($rest:tt)*) => {
        $crate::read_value!(@vec @source [$source] @kind [$($kind)* $tt] @rest $($rest)*)
    };
    (@vec @source [$source:expr] @kind [$($kind:tt)*] @len [$($len:tt)*]) => {{
        let len = $($len)*;
        (0..len)
            .map(|_| $crate::read_value!(@source [$source] @kind [$($kind)*]))
            .collect::<Vec<_>>()
    }};
    (@array @source [$source:expr] @kind [$($kind:tt)*] @len [$($len:tt)*]) => {{
        const LEN: usize = $($len)*;
        let mut tmp = [0; LEN];
        for i in 0..LEN {
            tmp[i] = $crate::read_value!(@source [$source] @kind [$($kind)*])
        }
        tmp
    }};

    // tuple
    (@source [$source:expr] @kind [($($kinds:tt)*)]) => {
        $crate::read_value!(@tuple @source [$source] @kinds [] @current [] @rest $($kinds)*)
    };
    (@tuple @source [$source:expr] @kinds [$([$($kind:tt)*])*] @current [] @rest) => {
        (
            $($crate::read_value!(@source [$source] @kind [$($kind)*]),)*
        )
    };
    (@tuple @source [$source:expr] @kinds [$($kinds:tt)*] @current [$($curr:tt)*] @rest) => {
        $crate::read_value!(@tuple @source [$source] @kinds [$($kinds)* [$($curr)*]] @current [] @rest)
    };
    (@tuple @source [$source:expr] @kinds [$($kinds:tt)*] @current [$($curr:tt)*] @rest, $($rest:tt)*) => {
        $crate::read_value!(@tuple @source [$source] @kinds [$($kinds)* [$($curr)*]] @current [] @rest $($rest)*)
    };
    (@tuple @source [$source:expr] @kinds [$($kinds:tt)*] @current [$($curr:tt)*] @rest $tt:tt $($rest:tt)*) => {
        $crate::read_value!(@tuple @source [$source] @kinds [$($kinds)* [$($curr)* $tt]] @current [] @rest $($rest)*)
    };

    // unreachable
    (@source [$source:expr] @kind []) => {
        compile_error!("Reached unreachable statement while parsing macro input.")
    };

    // normal other
    (@source [$source:expr] @kind [$kind:ty]) => {
        <$kind as $crate::utils::scan_macro::Readable>::read($source)
    };
}
