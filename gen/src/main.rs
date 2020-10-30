use std::error::Error;

use gen::{decl, generate};

fn main() -> Result<(), Box<dyn Error>> {
    let cd = std::env::current_dir().unwrap();
    let src_glob = format!(
        // "{}/git/rsk0315/library-rs/crates/*/*/Cargo.toml", // for local
        "{}/work/library-rs/library-rs/master/crates/*/*/Cargo.toml", // for remote
        std::env::var("HOME").unwrap()
    );
    let dst = cd.join("generated/nekolib").into();
    generate(&src_glob, &dst)?;

    // XXX
    // let src_glob = format!(
    //     // "{}/git/rsk0315/library-rs/verifiers/verify/Cargo.toml", // for local
    //     "{}/work/library-rs/library-rs/master/verifiers/verify/Cargo.toml", // for remote
    //     std::env::var("HOME").unwrap()
    // );
    // let dst = cd.join("generated/nekolib-verify").into();
    // generate(src_glob, dst)?;

    decl(&src_glob, &dst.join("decl.toml"))?;

    Ok(())
}
