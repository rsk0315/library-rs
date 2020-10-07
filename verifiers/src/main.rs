use verifiers::{jury, test_set};

macro_rules! downloads {
    ( $( $t:ty, )* ) => {
        $( test_set::download::<$t>().await?; )*
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use jury::*;

    downloads! {
        Aoj0000,
        Aoj0002,
        Yuki3287,
    }

    Ok(())
}
