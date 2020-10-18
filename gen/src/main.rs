use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::io::{self, Write};
use std::path::{Component, PathBuf};

use glob::glob;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct LibPath {
    path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
struct Manifest {
    dependencies: BTreeMap<String, LibPath>,
}

fn main() -> Result<(), std::io::Error> {
    let cd = std::env::current_dir().unwrap();
    let lib_dir = format!(
        // "{}/git/rsk0315/library-rs/crates", // for local
        "{}/work/library-rs/library-rs/master/crates", // for remote
        std::env::var("HOME").unwrap()
    );
    let lib_dir = PathBuf::from(lib_dir);
    let dst = cd.join("generated/nekolib").into();
    generate(lib_dir, dst)?;

    let verify_dir = format!(
        // "{}/git/rsk0315/library-rs/verifiers", // for local
        "{}/work/library-rs/library-rs/master/verifiers", // for remote
        std::env::var("HOME").unwrap()
    );
    let verify_dir = PathBuf::from(verify_dir);
    let dst = cd.join("generated/nekolib-verify").into();
    generate(verify_dir, dst)?;

    Ok(())
}

fn generate(lib_dir: PathBuf, dst: PathBuf) -> Result<(), std::io::Error> {
    let lib_dir = format!(
        // "{}/git/rsk0315/library-rs/crates", // for local
        "{}/work/library-rs/library-rs/master", // for remote
        std::env::var("HOME").unwrap()
    );
    eprintln!("Move {:?} => {:?}", &lib_dir, &dst);

    let tomls = format!("{}/*/*/Cargo.toml", &lib_dir);

    // for local
    // if dst.exists() {
    //     std::fs::remove_dir_all(&dst)?;
    // }
    if !dst.exists() {
        std::fs::create_dir(&dst)?;
    }

    let mut crates = BTreeSet::<String>::new();

    for toml_path in glob(&tomls).unwrap() {
        let toml_path = match toml_path {
            Ok(toml_path) => toml_path,
            _ => continue,
        };

        eprintln!("cloning {:?}", &toml_path);

        match clone(&toml_path, &dst) {
            Ok(_) => {}
            Err(e) => eprintln!("{:?}: fail; {:?}", toml_path, e),
        }

        let mod_path = toml_path.parent().unwrap();
        let crate_path = mod_path.parent().unwrap();

        let crate_name = crate_path.file_stem().unwrap().to_str().unwrap();

        crates.insert(crate_name.to_string());
    }

    let lib_rs = dst.join("src/lib.rs");
    let mut outfile = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(lib_rs)?;

    for c in crates {
        // add `pub mod {crate};` to src/lib.rs
        let pubmod = format!("pub mod {};\n", c);
        outfile.write_all(pubmod.as_bytes())?;
    }

    Ok(())
}

fn clone(toml_path: &PathBuf, dst: &PathBuf) -> Result<(), io::Error> {
    let content = String::from_utf8_lossy(&std::fs::read(&toml_path).unwrap())
        .to_string();
    let man: Manifest = toml::de::from_str(&content).unwrap();

    let mod_path = toml_path.parent().unwrap();
    let crate_path = mod_path.parent().unwrap();

    let mod_name = mod_path.file_stem().unwrap().to_str().unwrap();
    let mod_name_ = mod_name.replace("-", "_");
    let crate_name = crate_path.file_stem().unwrap().to_str().unwrap();

    std::fs::create_dir_all(&dst.join(format!("src/{}", crate_name)))?;

    // {crate}/{mod}/src/lib.rs => src/{crate}/{mod}.rs
    let outfile_name = dst.join(format!("src/{}/{}.rs", crate_name, mod_name_));
    let mut outfile = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(outfile_name)?;

    let rs_path = mod_path.join("src/lib.rs");
    let lib_content = generate_lib_rs(man.dependencies, rs_path);
    outfile.write_all(lib_content.as_bytes())?;

    let outfile_name = dst.join(format!("src/{}.rs", crate_name));

    // append `pub mod {mod};` to src/{crate}.rs
    let mut outfile = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(outfile_name)?;

    let mod_content = vec![
        format!("pub mod {};", mod_name_),
        "#[doc(inline)]".to_string(),
        format!("pub use {}::*;\n", mod_name_),
    ]
    .join("\n");

    outfile.write_all(mod_content.as_bytes())?;

    Ok(())
}

fn generate_lib_rs(
    dependencies: BTreeMap<String, LibPath>,
    rs_path: PathBuf,
) -> String {
    let uses = dependencies.into_iter().filter_map(|(_, path)| {
        let s: Vec<_> = path
            .path
            .components()
            .map(|c| match c {
                Component::RootDir => "crate",
                Component::ParentDir => "super",
                Component::Normal(s) => s.to_str().unwrap(),
                _ => unreachable!(),
            })
            .collect();
        let s: String = s.join("::").replace("-", "_");

        match s.is_empty() {
            true => None,
            false => Some(format!("use {};", s)),
        }
    });
    let mut uses: Vec<_> = uses.collect();
    uses.sort_unstable();

    let mut output = "".to_string();

    let content =
        String::from_utf8_lossy(&std::fs::read(&rs_path).unwrap()).to_string();
    let mut content = content.lines();

    loop {
        match content.next() {
            None => break,
            Some(l) if l.starts_with("//!") => {
                output.push_str(&format!("{}\n", l))
            }
            Some(l) => {
                if !uses.is_empty() {
                    if l == "" {
                        output.push_str("\n");
                    }
                    for use_ in uses {
                        output.push_str(&format!("{}\n", use_));
                    }
                }
                output.push_str(&format!("{}\n", l));
                for l in content {
                    output.push_str(&format!("{}\n", l));
                }
                break;
            }
        }
    }

    output
}
