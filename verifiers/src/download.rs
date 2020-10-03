use std::io::{self, Write};
use std::path::PathBuf;

pub async fn download_testcase(
    oj: &str,
    id: &str,
) -> Result<Vec<(PathBuf, PathBuf)>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let dir = PathBuf::from(format!("testcases/{}/{}", oj, id));
    std::fs::create_dir_all(dir.as_path())?;
    let url_case_in = |id, casenum| match oj {
        "aoj" => format!(
            "https://judgedat.u-aizu.ac.jp/testcases/{}/{}/in",
            id,
            casenum + 1
        ),
        _ => unimplemented!(),
    };
    let url_case_out = |id, casenum| match oj {
        "aoj" => format!(
            "https://judgedat.u-aizu.ac.jp/testcases/{}/{}/out",
            id,
            casenum + 1,
        ),
        _ => unimplemented!(),
    };

    let mut cases = vec![];
    for casenum in 0.. {
        let url_case_in = url_case_in(id, casenum);
        let file_case_in = dir.as_path().join(format!("{}.in", casenum));

        let url_case_out = url_case_out(id, casenum);
        let file_case_out = dir.as_path().join(format!("{}.out", casenum));

        let in_ = fetch(&client, &url_case_in, file_case_in).await;
        let out = fetch(&client, &url_case_out, file_case_out).await;

        match (in_, out) {
            (Ok(i), Ok(o)) => cases.push((i, o)),
            _ => break,
        }
    }

    Ok(cases)
}

async fn fetch(
    client: &reqwest::Client,
    url: &str,
    path_buf: PathBuf,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if path_buf.as_path().exists() {
        return Ok(path_buf);
    }

    let content = client.get(url).send().await?.text().await?;
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
