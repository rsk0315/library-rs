use verifiers::download::download_testcase;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    download_testcase("aoj", "0000").await?;
    download_testcase("aoj", "0002").await?;

    Ok(())
}
