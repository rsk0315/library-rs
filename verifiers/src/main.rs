use serde::Deserialize;

use verifiers::testcase::*;

#[derive(Deserialize)]
struct Testcases {
    aoj: Option<Vec<String>>,
    yukicoder: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fin = find_testcases_toml("verifiers/testcases.toml")?;
    let testcases =
        String::from_utf8_lossy(&std::fs::read(fin.clone()).unwrap())
            .to_string();
    let testcases: Testcases = toml::from_str(&testcases).unwrap();

    let dir = fin.parent().unwrap().join("testcases");

    if let Some(aoj) = testcases.aoj {
        for id in aoj {
            download_testcase("aoj", &id, &dir).await?;
        }
    }
    if let Some(yukicoder) = testcases.yukicoder {
        for id in yukicoder {
            download_testcase("yukicoder", &id, &dir).await?;
        }
    }

    Ok(())
}
