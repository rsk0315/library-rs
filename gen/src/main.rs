use std::error::Error;

use gen::{decl, generate};

fn main() -> Result<(), Box<dyn Error>> {
    let cd = std::env::current_dir().unwrap();
    let lib_root = format!(
        // "{}/git/rsk0315/library-rs", // for local
        "{}/work/library-rs/library-rs/master", // for remote
        std::env::var("HOME")?
    );

    let src_toml_glob = format!("{}/crates/*/*/Cargo.toml", lib_root);
    let dst = cd.join("generated/nekolib").into();
    generate(&src_toml_glob, &dst)?;

    // XXX
    // let src_glob = format!("{}/verifiers/verify/Cargo.toml", lib_root);
    // let dst = cd.join("generated/nekolib-verify").into();
    // generate(src_glob, dst)?;

    let src_rs_glob = format!("{}/static/nekolib/src/*.rs", lib_root);

    decl(&src_toml_glob, &src_rs_glob, &dst.join("decl-index.toml"))?;

    Ok(())
}
