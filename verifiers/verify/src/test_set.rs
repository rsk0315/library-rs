use std::fmt::{Debug, Display};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::Duration;

use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Verdict {
    Ac(usize),
    Wa(usize, String),
    Re(usize, String),
    Tle(usize, String),
}

impl Display for Verdict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ac(0) => write!(f, "no cases found"),
            Ac(1) => write!(f, "passed 1 case"),
            Ac(n) => write!(f, "passed {} cases", n),
            Wa(n, _) => write!(f, "WA on test #{}", n),
            Re(n, _) => write!(f, "RE on test #{}", n),
            Tle(n, _) => write!(f, "TLE on test #{}", n),
        }?;
        match self {
            Wa(_, e) | Re(_, e) | Tle(_, e) if e != "" => write!(f, "; {}", e),
            _ => write!(f, ""),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Oj {
    Aoj(&'static str),
    Yukicoder(&'static str),
}

pub use Oj::*;
pub use Verdict::*;

pub trait Solver {
    type Jury: Jury;
    fn solve(
        input: <Self::Jury as Jury>::Input,
    ) -> <Self::Jury as Jury>::Output;
}

pub trait Jury {
    type Input: Clone + DeserializeOwned + Serialize + Send + Sync;
    type Output: Debug + Eq + DeserializeOwned + Serialize + Send + Sync;
    const TL: Duration;
    const PROBLEM: Oj;
    fn parse_input(input: String) -> Self::Input;
    fn parse_output(input: &Self::Input, output: String) -> Self::Output;
    fn judge(
        _input: Self::Input,
        output: Self::Output,
        jury: Self::Output,
    ) -> Verdict {
        if output == jury {
            Ac(1)
        } else {
            Wa(
                0,
                format!("\noutput: {:#?};\nexpected: {:#?}", output, jury),
            )
        }
    }
}

pub fn judge_vec<T: Debug + Eq>(output: &Vec<T>, jury: &Vec<T>) -> Verdict {
    let i = match output
        .iter()
        .zip(jury.iter())
        .enumerate()
        .find_map(|(i, (o, j))| if o != j { Some(i) } else { None })
    {
        Some(i) => i,
        None if output.len() == jury.len() => return Ac(1),
        None => output.len().min(jury.len()),
    };

    let mut msg = "".to_string();
    if output.len() != jury.len() {
        msg.push_str(&format!(
            "len differs; got: {}, expected: {}. ",
            output.len(),
            jury.len()
        ));
    }

    msg.push_str(&format!("{}-th element differs:\n", i));

    let c = 5;
    let range_got =
        (if i > c { i - c } else { 0 })..output.len().min(i + c + 1);
    let range_expected =
        (if i > c { i - c } else { 0 })..jury.len().min(i + c + 1);

    msg.push_str(&format!(
        "got[{:?}]: {:#?},\nexpected[{:?}]: {:#?}",
        &range_got,
        &output[range_got.clone()],
        &range_expected,
        &jury[range_expected.clone()]
    ));
    Wa(0, msg)
}

#[must_use]
pub fn test<S: Solver>() -> Verdict
where
    <S::Jury as Jury>::Input: 'static,
    <S::Jury as Jury>::Output: 'static,
{
    let dir = find_cases_dir(<S::Jury as Jury>::PROBLEM).unwrap();
    for i in 0.. {
        let (input, jury) = {
            let input = read(dir.clone().join(format!("{}.in", i)));
            let jury = read(dir.clone().join(format!("{}.out", i)));
            match (input, jury) {
                (Ok(i), Ok(j)) => (i, j),
                _ => return Ac(i),
            }
        };

        let input = <S::Jury as Jury>::parse_input(input);
        let jury = <S::Jury as Jury>::parse_output(&input, jury);
        let output = match run_solver::<S>(input.clone()) {
            Ok(output) => output,
            Err(RecvTimeoutError::Timeout) => {
                let tl = <S::Jury as Jury>::TL.as_millis();
                return Tle(i, format!("{} ms", tl));
            }
            Err(RecvTimeoutError::Disconnected) => {
                return Re(i, "".to_string())
            }
        };
        match <S::Jury as Jury>::judge(input, output, jury) {
            Ac(_) => {}
            Wa(_, e) => return Wa(i, e),
            Re(_, e) => return Re(i, e),
            Tle(_, e) => return Tle(i, e),
        }
    }
    unreachable!();
}

pub fn verify<S: Solver>()
where
    <S::Jury as Jury>::Input: 'static,
    <S::Jury as Jury>::Output: 'static,
{
    let verdict = test::<S>();
    match verdict {
        Ac(n) if n > 0 => eprintln!("{}", verdict),
        _ => panic!("{}", verdict),
    };
}

pub fn find_cases_dir(oj: Oj) -> Option<PathBuf> {
    let cd = Path::new(&std::env::current_dir().unwrap()).to_path_buf();
    let d = cd.ancestors().find_map(|d| {
        let d = d.join("testcases");
        if d.exists() {
            Some(d)
        } else {
            None
        }
    });
    match (d, oj) {
        (Some(d), Aoj(id)) => Some(d.join("aoj").join(id)),
        (Some(d), Yukicoder(id)) => Some(d.join("yukicoder").join(id)),
        _ => None,
    }
}

fn read(p: PathBuf) -> Result<String, std::io::Error> {
    Ok(String::from_utf8_lossy(&std::fs::read(p)?).to_string())
}

fn run_solver<S: Solver>(
    input: <S::Jury as Jury>::Input,
) -> Result<<S::Jury as Jury>::Output, RecvTimeoutError>
where
    <S::Jury as Jury>::Input: 'static,
    <S::Jury as Jury>::Output: 'static,
{
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || tx.send(S::solve(input)));
    rx.recv_timeout(<S::Jury as Jury>::TL)
}
