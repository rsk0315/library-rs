## Custom Judge

入出力フォーマットは正しいことを仮定。

```ignore
use std::fs::File;
use std::io::Read;

use proconio::{input, source::auto::AutoSource};

// specify judge command instead of default diff judge. The given command
// (e.g. `./judge`) will be called as
// ```
// $ ./judge input.txt actual-output.txt expected-output.txt
// ```
// and should return the result with the exit code of its `main` function.

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = read(&args[1]);
    let actual = read(&args[2]);
    let expected = read(&args[3]);

    let mut input = AutoSource::from(input.as_ref());
    let mut actual = AutoSource::from(actual.as_ref());
    let mut expected = AutoSource::from(expected.as_ref());

    input! {
        from &mut input,
    }
    input! {
        from &mut actual,
    }
    input! {
        from &mut expected,
    }

    assert!(true);
}

fn read(path: &str) -> String {
    let mut res = "".to_owned();
    File::open(path).unwrap().read_to_string(&mut res).unwrap();
    res
}
```

## Generator

```toml
rand = "0.8.5"
rand_chacha = "0.3.1"
```

```ignore
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use nekolib::rand_gen;
use nekolib::utils::SpaceSep;
use nekolib::utils::rand_gen_macro::*;

fn main() {
    rand_gen! {
        rng: ChaCha20Rng;
    
        n in 1_usize..=10;
        a in [1_i64..=100; n];
    }
    
    println!("{n}");
    println!("{}", SpaceSep(&a));
}
```

## Reactive

<https://rsk0315.hatenablog.com/entry/2022/09/19/200454>

```sh
% sniff_reactive ./solution -- ./judge testcase.in 
```

### Reactive Runner

```python
import asyncio
import os
import sys

[CONTESTANT, JUDGE] = range(2)
COLUMNS = os.get_terminal_size().columns

CONTMARK = "\x1b[38;5;245m↵\x1b[m"
ELLIMARK = "\x1b[38;5;245m…\x1b[m"

CROSS_TOP = "┌┬┐"
CROSS_INNER = "├┼┤"
CROSS_BOTTOM = "└┴┘"


def output(left, right, *, preleft="", preright="", cross=CROSS_INNER):
    wl = COLUMNS // 2 - 4
    wr = (COLUMNS - 1) // 2 - 4

    if not left and not right:
        print(f' {cross[0]}─{"─" * wl}─{cross[1]}─{"─" * wr}─{cross[2]}')
        return

    if left and right:
        preleft = preright = "\x1b[1;37m"

    postleft = "\x1b[m" if preleft else ""
    postright = "\x1b[m" if preright else ""

    # print(f" |{preleft} {left:{wl}} {postleft}|{preright} {right:{wr}} {postright}|")

    if wl == 0 or wr == 0 or (len(left) <= wl and len(right) <= wr):
        print(
            f" │{preleft} {left:{wl}} {postleft}│{preright} {right:{wr}} {postright}│"
        )
        return

    offl = 0
    offr = 0
    chunkl = left[offl : offl + wl]
    chunkr = right[offr : offr + wr]
    elll = " "
    ellr = " "
    while chunkl or chunkr:
        contl = CONTMARK if left[offl + wl : offl + wl + 1] else " "
        contr = CONTMARK if right[offr + wr : offr + wr + 1] else " "
        print(
            f" │{preleft}{elll}{chunkl:{wl}}{contl}{postleft}│{preright}{ellr}{chunkr:{wr}}{contr}{postright}│"
        )
        offl += wl
        offr += wr
        chunkl = left[offl : offl + wl]
        chunkr = right[offr : offr + wr]
        elll = ELLIMARK if chunkl else " "
        ellr = ELLIMARK if chunkr else " "


async def listen(read, write, who, proc):
    while proc.returncode is None:
        content = await asyncio.to_thread(read.readline)
        if content:
            contents = ["", ""]
            contents[who] = content.rstrip("\n")
            output(*contents)
            print(content, end="", flush=True, file=write)
        else:
            # 待ち続けるのを防ぐ
            write.close()
            break


async def sniff(c_read, j_read, c_write, j_write, c_proc, j_proc):
    with open(c_read, "w") as w_c_read:
        with open(j_read, "w") as w_j_read:
            with open(c_write, "r") as r_c_write:
                with open(j_write, "r") as r_j_write:
                    try:
                        await asyncio.gather(
                            listen(r_c_write, w_j_read, CONTESTANT, c_proc),
                            listen(r_j_write, w_c_read, JUDGE, j_proc),
                        )
                    except Exception:
                        pass


async def main():
    paths = ["cr.pipe", "jr.pipe", "cw.pipe", "jw.pipe"]
    [c_read, j_read, c_write, j_write] = paths
    for path in paths:
        if os.path.exists(path):
            os.remove(path)
        os.mkfifo(path)

    sep = sys.argv.index("--")
    contestant_command = " ".join(sys.argv[1:sep]) + f" < {c_read} > {c_write}"
    judge_command = " ".join(sys.argv[sep + 1 :]) + f" < {j_read} > {j_write}"

    output("", "", cross=CROSS_TOP)
    output("contestant", "judge")
    output("", "")

    devnull = asyncio.subprocess.DEVNULL
    c_proc = await asyncio.create_subprocess_shell(
        contestant_command, shell=True, stderr=devnull
    )
    j_proc = await asyncio.create_subprocess_shell(
        judge_command, shell=True, stderr=devnull
    )

    await asyncio.gather(
        sniff(c_read, j_read, c_write, j_write, c_proc, j_proc),
        c_proc.communicate(),
        j_proc.communicate(),
    )

    status = "AC"
    if c_proc.returncode != 0:
        status = "WA" if j_proc.returncode != 0 else "RE"
    elif j_proc.returncode != 0:
        status = "WA"

    preright = "\x1b[1;92m" if status == "AC" else "\x1b[1;91m"
    output("", status, preright=preright)
    output("", "", cross=CROSS_BOTTOM)

    for path in paths:
        os.remove(path)


if __name__ == "__main__":
    asyncio.run(main())
```

### Reactive Judge

```ignore
use std::env;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Write};

use proconio::{
    input,
    marker::Usize1,
    source::{auto::AutoSource, line::LineSource},
};

macro_rules! println {
    ( $($t:tt)* ) => {
        std::println!($($t)*);
        stdout().flush().unwrap();
    }
}

fn main() {
    // --- テストケース用ファイルの読み込み ---

    let infile_source = {
        let name = env::args().nth(1).unwrap();
        let file = File::open(&name).unwrap();
        AutoSource::new(BufReader::new(file))
    };

    input! {
        from infile_source,
    }

    // --- ジャッジ側の前処理 ---

    let expected = {
    };

    // --- 提出プログラムとのやり取り ---

    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    println!("{}", n);

    let ql = 20;
    for i in 0..=ql {
        input! {
            from &mut source,
            ty: char,
        }

        assert!(['!', '?'].contains(&ty));

        if ty == '!' {
            input! {
                from &mut source,
            }
            assert_eq!((), expected);
            return;
        }

        if ty == '?' {
            assert!(i < ql);
            input! {
                from &mut source,
            }

            let res = {
            };

            println!("{}", res);
        }
    }
}
```
