use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use mpsc::RecvTimeoutError;

pub trait Solver: Fn(String) -> String + Send + Sync {}
impl<F> Solver for F where F: Fn(String) -> String + Send + Sync {}

pub trait Judge:
    Fn(String, String, String, usize) -> Verdict + Send + Sync
{
}
impl<F> Judge for F where
    F: Fn(String, String, String, usize) -> Verdict + Send + Sync
{
}

pub struct Verifier {
    testcases: Option<PathBuf>,
    tl: Duration,
    solver: Option<&'static dyn Solver>,
    judge: &'static dyn Judge,
}

fn default_judge(_: String, out: String, jury: String, id: usize) -> Verdict {
    if out == jury {
        Ac(id + 1)
    } else {
        Wa(id, "".to_string())
    }
}

impl Verifier {
    pub fn new() -> Self {
        Self {
            testcases: None,
            tl: Duration::from_millis(2000),
            solver: None,
            judge: &default_judge,
        }
    }
    pub fn testcases(&mut self, testcases: PathBuf) -> &mut Self {
        self.testcases = Some(testcases);
        self
    }
    pub fn tl(&mut self, tl: Duration) -> &mut Self {
        self.tl = tl;
        self
    }
    pub fn solver(&mut self, solver: &'static dyn Solver) -> &mut Self {
        self.solver = Some(solver);
        self
    }
    pub fn custom_judge(&mut self, judge: &'static dyn Judge) -> &mut Self {
        self.judge = judge;
        self
    }
    pub fn run(&self) {
        match self.do_run() {
            Ac(n) if n > 0 => eprintln!("{}", Ac(n)),
            v => panic!("{}", v),
        }
    }
    fn do_run(&self) -> Verdict {
        let path = self.testcases.clone().unwrap();

        for i in 0.. {
            let fin = path.join(format!("{}.in", i));
            let fout = path.join(format!("{}.out", i));
            if !(fin.exists() && fout.exists()) {
                return Ac(i);
            }

            let input = String::from_utf8_lossy(&std::fs::read(fin).unwrap())
                .to_string();
            let output = match self.run_solver(input.clone()) {
                Err(RecvTimeoutError::Timeout) => {
                    return Tle(i, format!("{} ms", self.tl.as_millis()))
                }
                Err(RecvTimeoutError::Disconnected) => {
                    return Re(i, "".to_string())
                }
                Ok(output) => output,
            };
            let jury_output =
                String::from_utf8_lossy(&std::fs::read(fout).unwrap())
                    .to_string();

            match (self.judge)(input, output, jury_output, i) {
                Ac(_) => (),
                v => return v,
            }
        }
        unreachable!();
    }
    fn run_solver(&self, input: String) -> Result<String, RecvTimeoutError> {
        let (tx, rx) = mpsc::channel();
        let solver = self.solver.unwrap();
        thread::spawn(move || tx.send(solver(input)));
        rx.recv_timeout(self.tl)
    }
}

pub enum Verdict {
    Ac(usize),
    Wa(usize, String),
    Re(usize, String),
    Tle(usize, String),
}

use Verdict::*;

impl std::fmt::Display for Verdict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ac(n) if n > &1 => write!(f, "passed {} cases", n),
            Ac(1) => write!(f, "passed 1 case"),
            Ac(_) => write!(f, "no cases found"),
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
