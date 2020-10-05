use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub async fn download_testcase(
    oj: &str,
    id: &str,
    dir: &Path,
) -> Result<Vec<(PathBuf, PathBuf)>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let dir = PathBuf::from(format!("{}/{}/{}", dir.to_str().unwrap(), oj, id));
    std::fs::create_dir_all(dir.as_path())?;

    let token = match oj {
        "yukicoder" => Some(
            std::env::var("YUKICODER_TOKEN").expect("$YUKICODER_TOKEN not set"),
        ),
        _ => None,
    };

    let (casenames, caselen) = match oj {
        "yukicoder" => {
            let url =
                format!("https://yukicoder.me/api/v1/problems/{}/file/in", id);
            let tmp = client
                .get(&url)
                .bearer_auth(token.clone().unwrap())
                .send()
                .await?
                .text()
                .await?;
            let casenames: Vec<String> = serde_json::from_str(&tmp)?;
            let caselen = casenames.len();
            (Some(casenames), Some(caselen))
        }
        _ => (None, None),
    };

    let url_case_in = |id, casenum: usize| match oj {
        "aoj" => Some(format!(
            "https://judgedat.u-aizu.ac.jp/testcases/{}/{}/in",
            id,
            casenum + 1
        )),
        "yukicoder" if casenum < caselen.unwrap() => Some(format!(
            "https://yukicoder.me/api/v1/problems/{}/file/in/{}",
            id,
            casenames.as_ref().map_or("", |v| &v[casenum])
        )),
        "yukicoder" => None,
        _ => unimplemented!(),
    };
    let url_case_out = |id, casenum: usize| match oj {
        "aoj" => Some(format!(
            "https://judgedat.u-aizu.ac.jp/testcases/{}/{}/out",
            id,
            casenum + 1,
        )),
        "yukicoder" if casenum < caselen.unwrap() => Some(format!(
            "https://yukicoder.me/api/v1/problems/{}/file/out/{}",
            id,
            casenames.as_ref().map_or("", |v| &v[casenum])
        )),
        "yukicoder" => None,
        _ => unimplemented!(),
    };

    let mut cases = vec![];
    for casenum in 0.. {
        let url_case_in = url_case_in(id, casenum);
        let file_case_in = dir.as_path().join(format!("{}.in", casenum));

        let url_case_out = url_case_out(id, casenum);
        let file_case_out = dir.as_path().join(format!("{}.out", casenum));

        let (url_case_in, url_case_out) = match (url_case_in, url_case_out) {
            (Some(i), Some(o)) => (i, o),
            _ => break,
        };

        let in_ =
            fetch(&client, token.clone(), &url_case_in, file_case_in).await;
        let out =
            fetch(&client, token.clone(), &url_case_out, file_case_out).await;

        match (in_, out) {
            (Ok(i), Ok(o)) => cases.push((i, o)),
            _ => break,
        }
    }

    Ok(cases)
}

async fn fetch(
    client: &reqwest::Client,
    token: Option<String>,
    url: &str,
    path_buf: PathBuf,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if path_buf.as_path().exists() {
        return Ok(path_buf);
    }

    let client = match token {
        Some(token) => client.get(url).bearer_auth(token),
        None => client.get(url),
    };
    let content = client.send().await?.text().await?;

    if content == "/* This is a single file for multiple testcases. serial should be 1. */"
    {
        Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "no more cases")))
    }
    else if content.starts_with("/* Test case #") && content.ends_with(" is not available. */") {
        Err(Box::new(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "not available",
        )))
    } else {
        let mut file = std::fs::File::create(path_buf.as_path())?;
        file.write_all(content.as_bytes())?;
        eprintln!("save to {:#?}", file);
        Ok(path_buf)
    }
}

pub fn find_testcases_toml(filename: &str) -> Result<PathBuf, std::io::Error> {
    PathBuf::from(std::env::current_dir().unwrap())
        .ancestors()
        .find_map(|p| {
            let p = p.join(filename);
            if p.as_path().exists() {
                Some(p)
            } else {
                None
            }
        })
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{} not found", filename),
        ))
}
