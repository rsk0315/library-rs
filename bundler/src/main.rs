use std::collections::BTreeMap;
use std::error::Error;
use std::io::Read;
use std::path::Path;

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
    let default_index = format!(
        "{}/git/rsk0315/library-rs/gen/generated/nekolib/decl-index.toml",
        std::env::var("HOME")?
    );
    let m = App::new("bundle")
        .arg(
            Arg::with_name("index")
                .short("i")
                .long("index")
                .value_name("INDEX_PATH")
                .help("Path to index file")
                .default_value(default_index.as_str()),
        )
        .arg(Arg::with_name("SOURCE").default_value("src/main.rs"))
        .get_matches();

    let source = m.value_of("SOURCE").unwrap();
    let index_path = m.value_of("index").unwrap();
    bundle(&source, Path::new(index_path))?;

    Ok(())
}

fn bundle(filename: &str, index_path: &Path) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::File::open(&filename)?;
    eprintln!("source: {:?}", file);
    let mut src = String::new();
    file.read_to_string(&mut src)?;

    let mut file = std::fs::File::open(&index_path)?;
    eprintln!("toml: {:?}", file);
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

    // まだ ::* をうまく扱えないと思う。扱わない方針になるかも

    let mut includes = vec![];
    for mut crate_mod in extract_uses_file(&src)? {
        let mod_name = crate_mod.pop().unwrap();
        // let decl_in = decl[&mod_name].clone();

        if let Some(decl_in) = decl.get(&mod_name).cloned() {
            if let Some(mut v) = deps.remove(&decl_in) {
                includes.append(&mut v);
            }
            includes.push(decl_in);
        } else {
            // 一旦 macro は foo_macro のファイル名に入れる運用にしてみる。
            // あと utils に入れるとする。本来は #[macro_export] を見るべき？
            let macro_name = format!("{}_macro", mod_name);
            let decl_in = ("utils".to_owned(), macro_name);
            if let Some(mut v) = deps.remove(&decl_in) {
                includes.append(&mut v);
            }
            includes.push(decl_in);
        }
    }

    includes.sort_unstable();
    includes.dedup();

    let includes = {
        let mut tmp = BTreeMap::new();
        for (crate_name, mod_name) in includes {
            tmp.entry(crate_name)
                .or_insert(vec![])
                .push(mod_name.replace("-", "_"));
        }
        for (_crate_name, v) in tmp.iter_mut() {
            v.sort_unstable();
            v.dedup();
        }
        tmp
    };

    print!("{}", src);

    if includes.is_empty() {
        return Ok(());
    }

    let pub_uses = {
        let mut tmp = BTreeMap::new();
        for (ident, crate_mod) in decl {
            tmp.entry(crate_mod).or_insert(vec![]).push(ident);
        }
        tmp
    };

    println!("");
    println!("/// This module is bundled automatically.");
    println!(
        "/// See <https://rsk0315.github.io/library-rs/nekolib/> for documentation."
    );
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
            let uses =
                pub_uses[&(crate_name.clone(), mod_name.clone())].join(", ");
            // 古いと self:: が必要？ GCJ でだめだった。要調査。
            println!("        pub use self::{}::{{{}}};", &mod_name, uses);
        }
        println!("    }}");
    }
    println!("}}");

    Ok(())
}
