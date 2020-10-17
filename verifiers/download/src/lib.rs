use std::io::{self, Write};
use std::path::PathBuf;

use verify::Oj::{self, Aoj, Yukicoder};
use verify::{find_cases_dir, Jury};

/// # Errors
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
        Aoj(_) => client.get(url),
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

fn yukitoken() -> String {
    std::env::var("YUKICODER_TOKEN").expect("$YUKICODER_TOKEN not set")
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
