use std::fmt::{Debug, Display};
use std::io::{self, Write};
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

pub(crate) use Oj::*;
pub(crate) use Verdict::*;

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
        match output == jury {
            true => Ac(1),
            false => {
                Wa(0, format!("output: {:?};\nexpected: {:?}", output, jury))
            }
        }
    }
}

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
            v => return v,
        }
    }
    unreachable!();
}

pub fn verify<S: Solver>() -> Verdict
where
    <S::Jury as Jury>::Input: 'static,
    <S::Jury as Jury>::Output: 'static,
{
    let verdict = test::<S>();
    match verdict {
        Ac(n) if n > 0 => eprintln!("{}", verdict),
        _ => panic!("{}", verdict),
    }
    verdict
}

fn find_cases_dir(oj: Oj) -> Option<PathBuf> {
    let cd = Path::new(&std::env::current_dir().unwrap()).to_path_buf();
    let d = cd.ancestors().find_map(|d| {
        let d = d.join("testcases");
        match d.exists() {
            true => Some(d),
            false => None,
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

async fn case_urls(
    problem: Oj,
) -> Result<
    Box<dyn Iterator<Item = (usize, (String, String))>>,
    Box<dyn std::error::Error>,
> {
    Ok(match problem {
        Aoj(id) => Box::new((0..).map(move |i| {
            let ui = format!(
                "https://judgedat.u-aizu.ac.jp/testcases/{}/{}/in",
                id,
                i + 1
            );
            let uo = format!(
                "https://judgedat.u-aizu.ac.jp/testcases/{}/{}/out",
                id,
                i + 1
            );
            (i, (ui, uo))
        })),
        Yukicoder(id) => {
            let url =
                format!("https://yukicoder.me/api/v1/problems/{}/file/in", id);
            let client = reqwest::Client::new();
            let tmp = client
                .get(&url)
                .bearer_auth(yukitoken())
                .send()
                .await?
                .text()
                .await?;
            let list: Vec<String> = serde_json::from_str(&tmp).unwrap();
            Box::new(list.into_iter().enumerate().map(move |(i, s)| {
                let ui = format!(
                    "https://yukicoder.me/api/v1/problems/{}/file/in/{}",
                    id, s
                );
                let uo = format!(
                    "https://yukicoder.me/api/v1/problems/{}/file/out/{}",
                    id, s
                );
                (i, (ui, uo))
            }))
        }
    })
}

pub async fn download<J: Jury>() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let dir = find_cases_dir(J::PROBLEM).unwrap();
    std::fs::create_dir_all(&dir)?;

    for (casenum, (url_in, url_out)) in case_urls(J::PROBLEM).await? {
        let file_in = dir.join(format!("{}.in", casenum));
        let file_out = dir.join(format!("{}.out", casenum));

        let input = fetch(&client, &url_in, file_in, J::PROBLEM).await;
        let output = fetch(&client, &url_out, file_out, J::PROBLEM).await;

        if input.is_err() | output.is_err() {
            break;
        }
    }

    Ok(())
}

fn yukitoken() -> String {
    std::env::var("YUKICODER_TOKEN").expect("$YUKICODER_TOKEN not set")
}

async fn fetch(
    client: &reqwest::Client,
    url: &str,
    path_buf: PathBuf,
    problem: Oj,
) -> Result<(), Box<dyn std::error::Error>> {
    use io::ErrorKind::{NotFound, PermissionDenied};

    if path_buf.exists() {
        return Ok(());
    }

    let client = match problem {
        Yukicoder(_) => client.get(url).bearer_auth(yukitoken()),
        _ => client.get(url),
    };
    let content = client.send().await?.text().await?;

    if content == "/* This is a single file for multiple testcases. serial should be 1. */"{
        Err(Box::new(io::Error::new(NotFound, "no more cases")))
    } else if content.starts_with("/* Test case #") && content.ends_with(" is not available. */") {
        Err(Box::new(io::Error::new(PermissionDenied, "not available")))
    } else {
        let mut file = std::fs::File::create(&path_buf)?;
        file.write_all(content.as_bytes())?;
        eprintln!("save to {:#?}", file);
        Ok(())
    }
}
