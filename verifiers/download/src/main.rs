macro_rules! downloads {
    ( $( $t:ty, )* ) => {
        $( download::download::<$t>().await?; )*
    }
}

#[allow(clippy::wildcard_imports)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use verify::jury::*;

    downloads! {
        Aoj0000,
        Aoj0002,
        Aoj0270,
        Aoj0425,
        Aoj0564,
        Aoj1180,
        AojDsl1A,
        AojDsl2B,
        Yuki3287,
    }

    Ok(())
}
