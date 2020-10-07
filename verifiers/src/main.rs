use test_set::{Jury, Solver};
use verifiers::{jury, solver, test_set};

use vec_segtree::*;

macro_rules! downloads {
    ( $( $t:ty, )* ) => {
        $( test_set::download::<$t>().await?; )*
    }
}

macro_rules! tests {
    ( $( $t:ty, )* ) => {
        $( {
            let v = test_set::verify::<$t>();
            eprintln!("{:?}: {:?}", <$t as Solver>::Jury::PROBLEM, v);
        }; )*
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    {
        use jury::*;
        downloads! {
            Aoj0000,
            Yuki3287,
        }
    }

    {
        use solver::*;
        tests! {
            Aoj0000,
            Yuki3287<VecSegtree<_>, VecSegtree<_>>,
        }
    }

    Ok(())
}
