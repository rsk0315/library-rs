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
        Aoj0575,
        Aoj1180,
        AojAldsOne14B,
        AojAldsOne14D,
        AojDsl1A,
        AojDsl2B,
        AojGrl1A,
        AojGrl3C,
        Yuki3287,
    }

    Ok(())
}
