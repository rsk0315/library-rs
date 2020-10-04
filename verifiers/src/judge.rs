use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use mpsc::RecvTimeoutError;

pub trait Solver: Fn(String) -> String + Send + Sync {}
impl<F> Solver for F where F: Fn(String) -> String + Send + Sync {}

pub trait Judge:
    Fn(String, String, String) -> Result<(), Option<String>> + Send + Sync
{
}
impl<F> Judge for F where
    F: Fn(String, String, String) -> Result<(), Option<String>> + Send + Sync
{
}

pub struct Verifier {
    testcases: Option<PathBuf>,
    tl: Duration,
    solver: Option<&'static dyn Solver>,
    judge: &'static dyn Judge,
}

fn default_judge(
    _: String,
    output: String,
    jury_output: String,
) -> Result<(), Option<String>> {
    if output == jury_output {
        Ok(())
    } else {
        Err(None)
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
            Err(Wa(e)) => panic!("{}", e),
            Err(Re(e)) => panic!("{}", e),
            Err(Tle(e)) => panic!("{}", e),
            Ok(n) if n > 0 => {
                eprintln!("passed {} test{}", n, if n > 1 { "s" } else { "" });
            }
            Ok(_) => panic!("no testcases found"),
        }
    }
    fn do_run(&self) -> Result<usize, Rejected> {
        use RecvTimeoutError::{Disconnected, Timeout};

        let path = self.testcases.clone().unwrap();

        for i in 0.. {
            let fin = path.join(format!("{}.in", i));
            let fout = path.join(format!("{}.out", i));
            if !(fin.exists() && fout.exists()) {
                return Ok(i);
            }

            let input = String::from_utf8_lossy(&std::fs::read(fin).unwrap())
                .to_string();
            let output =
                self.run_solver(input.clone()).map_err(|e| match e {
                    Timeout => Tle(format!("TLE on test #{}", i)),
                    Disconnected => Re(format!("RE on test #{}", i)),
                })?;
            let jury_output =
                String::from_utf8_lossy(&std::fs::read(fout).unwrap())
                    .to_string();

            (self.judge)(input, output, jury_output).map_err(|e| match e {
                Some(msg) => Wa(format!("WA on test #{}; {}", i, msg)),
                None => Wa(format!("WA on test #{}", i)),
            })?;
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

enum Rejected {
    Wa(String),
    Re(String),
    Tle(String),
}

use Rejected::*;
