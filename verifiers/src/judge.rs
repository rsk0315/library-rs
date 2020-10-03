use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn run_solver(
    input: String,
    solver: &'static (dyn Fn(String) -> String + Send + Sync),
    tl_millis: u64,
) -> Result<String, mpsc::RecvTimeoutError> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || tx.send(solver(input)));
    rx.recv_timeout(Duration::from_millis(tl_millis))
}

enum Rejected {
    Wa(String),
    Tle(String),
    Re(String),
}

use Rejected::*;

fn do_solve(
    path: &Path,
    solver: &'static (dyn Fn(String) -> String + Send + Sync),
    tl_millis: u64,
    custom_judge: Option<&dyn Fn(String, String) -> Result<(), Option<String>>>,
) -> Result<usize, Rejected> {
    use mpsc::RecvTimeoutError::{Disconnected, Timeout};

    for i in 0.. {
        let fin = path.join(format!("{}.in", i));
        let fout = path.join(format!("{}.out", i));
        if !(fin.exists() && fout.exists()) {
            return Ok(i);
        }

        let input =
            String::from_utf8_lossy(&std::fs::read(fin).unwrap()).to_string();
        let output =
            run_solver(input, solver, tl_millis).map_err(|e| match e {
                Timeout => Tle(format!("TLE on test #{}", i)),
                Disconnected => Re(format!("RE on test #{}", i)),
            })?;

        let jury_output =
            String::from_utf8_lossy(&std::fs::read(fout).unwrap()).to_string();

        match custom_judge {
            Some(j) => j(output, jury_output).map_err(|e| match e {
                Some(msg) => Wa(format!("WA on test #{}; {}", i, msg)),
                None => Wa(format!("WA on test #{}", i)),
            }),
            None if output != jury_output => {
                Err(Wa(format!("WA on test #{}", i)))
            }
            _ => Ok(()),
        }?;
    }
    unreachable!();
}

pub fn solve(
    path: &Path,
    solver: &'static (dyn Fn(String) -> String + Send + Sync),
    tl_millis: u64,
    custom_judge: Option<&dyn Fn(String, String) -> Result<(), Option<String>>>,
) {
    match do_solve(path, solver, tl_millis, custom_judge) {
        Err(Wa(e)) => panic!("{}", e),
        Err(Tle(e)) => panic!("{}", e),
        Err(Re(e)) => panic!("{}", e),
        Ok(n) if n > 0 => {
            eprintln!("passed {} test{}", n, if n > 1 { "s" } else { "" });
        }
        Ok(_) => panic!("no tests found"),
    }
}
