use std::collections::BTreeMap;
use std::error::Error;
use std::io::Read;
use std::path::PathBuf;

use clap::{App, Arg};
use serde::{Deserialize, Serialize};

use bundler::{extract_uses_file, polish_file};

#[derive(Debug, Deserialize, Serialize)]
struct DependsMap {
    name: (String, String),
    direct: Vec<(String, String)>,
    whole: Vec<(String, String)>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DeclIndex {
    declared: BTreeMap<String, (String, String)>,
    depends: Vec<DependsMap>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let m = App::new("bundle")
        .arg(Arg::with_name("SOURCE").default_value("src/main.rs"))
        .get_matches();

    let source = m.value_of("SOURCE").unwrap();
    bundle(&source)?;

    Ok(())
}

fn bundle(filename: &str) -> Result<(), Box<dyn Error>> {
    eprintln!("bundle({:?})", filename);

    let mut file = std::fs::File::open(&filename)?;
    let mut src = String::new();
    file.read_to_string(&mut src)?;

    let index_path = format!(
        "{}/git/rsk0315/library-rs/gen/generated/nekolib/decl-index.toml",
        std::env::var("HOME")?
    );
    let index_path = PathBuf::from(index_path);
    let mut file = std::fs::File::open(&index_path)?;
    let mut toml = String::new();
    file.read_to_string(&mut toml)?;

    let decl_index: DeclIndex = toml::from_str(&toml)?;
    let (decl, mut deps) = {
        let mut deps = BTreeMap::new();
        for dep in &decl_index.depends {
            deps.insert(dep.name.clone(), dep.whole.clone());
        }
        (decl_index.declared, deps)
    };

    // まだ ::* をうまく扱えないと思う

    let mut includes = vec![];
    for mut crate_mod in extract_uses_file(&src)? {
        let mod_name = crate_mod.pop().unwrap();
        let decl_in = decl[&mod_name].clone();
        if let Some(mut v) = deps.remove(&decl_in) {
            includes.append(&mut v);
        }
        includes.push(decl_in);
    }

    includes.sort_unstable();
    includes.dedup();

    let includes = {
        let mut tmp = BTreeMap::new();
        for (crate_name, mod_name) in includes {
            tmp.entry(crate_name).or_insert(vec![]).push(mod_name);
        }
        tmp
    };

    print!("{}", src);

    if includes.is_empty() {
        return Ok(());
    }

    println!("");
    println!("// --- bundled automatically --- //");
    println!("");

    println!("pub mod nekolib {{");
    for (crate_name, v) in includes {
        println!("    pub mod {} {{", &crate_name);
        for mod_name in v {
            println!("        pub mod {} {{", &mod_name);
            let path = index_path
                .parent()
                .unwrap()
                .join(&format!("src/{}/{}.rs", &crate_name, &mod_name));
            println!("{}", polish_file(&path.to_str().unwrap()).unwrap());
            println!("        }}");
            println!("        pub use {}::*;", &mod_name);
        }
        println!("    }}");
    }
    println!("}}");

    Ok(())
}
