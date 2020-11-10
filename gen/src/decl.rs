use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::Debug;
use std::io::Write;
use std::path::{Component, PathBuf};

use glob::glob;
use serde::{Deserialize, Serialize};
use syn::{parse_file, parse_quote};

#[derive(Debug, Deserialize, Serialize)]
struct Manifest {
    dependencies: BTreeMap<String, toml::Value>,
}

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

pub fn decl(
    src_toml: &str,
    src_rs: &str,
    dst_toml: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    // src_toml から、依存関係を読む。
    let mut deps = BTreeMap::new();
    for src_toml in glob(&src_toml)? {
        let src_toml = match src_toml {
            Ok(s) => s,
            _ => continue,
        };
        let crate_mod = get_name(&src_toml);
        for (dep_crate, dep_mod) in parse_dep(&src_toml)? {
            deps.entry(crate_mod.clone())
                .or_insert(vec![])
                .push((dep_crate, dep_mod));
        }
    }

    // src_rs から、re-export された識別子を読む。
    let mut decls = BTreeMap::new();
    for src_rs in glob(&src_rs)? {
        let src_rs = match src_rs {
            Ok(s) => s,
            _ => continue,
        };
        // nekolib/src/{crate_name}.rs
        let file_name: PathBuf = src_rs.file_name().unwrap().into();
        let crate_name =
            file_name.file_stem().unwrap().to_str().unwrap().to_string();
        let content =
            String::from_utf8_lossy(&std::fs::read(&crate_name)?).to_string();
        for (mod_name, ident) in parse_pub_uses(&content)? {
            decls.insert(ident, (crate_name.clone(), mod_name.clone()));
        }
    }

    let whole = dep_star(deps.clone());

    let res = DeclIndex {
        declared: decls,
        depends: deps
            .into_iter()
            .map(|(k, mut v)| {
                let mut whole = whole[&k].clone();
                v.sort_unstable();
                whole.sort_unstable();
                eprintln!("name: {:?}", &k);
                eprintln!("direct: {:#?}", &v);
                eprintln!("whole: {:#?}", &whole);
                DependsMap {
                    name: k,
                    direct: v,
                    whole,
                }
            })
            .collect(),
    };

    eprintln!("{:#?}", res);

    let toml = toml::ser::to_string(&res)?;
    let mut outfile = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&dst_toml)?;

    outfile.write_all(toml.as_bytes())?;
    Ok(())
}

fn parse_dep(
    src_toml: &PathBuf,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let (crate_name, mod_name) = get_name(&src_toml);

    let content =
        String::from_utf8_lossy(&std::fs::read(&src_toml)?).to_string();
    let man: Manifest = toml::de::from_str(&content)?;

    Ok(man
        .dependencies
        .into_iter()
        .filter_map(|(_, dep)| {
            if let toml::Value::Table(table) = dep {
                let path = match table.get("path") {
                    Some(p) => PathBuf::from(p.as_str()?),
                    None => return None,
                };
                let mut res = vec![crate_name.clone(), mod_name.clone()];
                for c in path.components() {
                    match c {
                        Component::ParentDir => {
                            res.pop();
                        }
                        Component::Normal(s) => {
                            res.push(s.to_str().unwrap().to_string());
                        }
                        _ => todo!(),
                    };
                }
                let mod_name = res.pop().unwrap();
                let crate_name = res.pop().unwrap();
                Some((crate_name, mod_name))
            } else {
                None
            }
        })
        .collect())
}

fn get_name(toml_path: &PathBuf) -> (String, String) {
    let mod_path = toml_path.parent().unwrap();
    let crate_path = mod_path.parent().unwrap();

    let mod_name = mod_path.file_stem().unwrap().to_str().unwrap();
    let crate_name = crate_path.file_stem().unwrap().to_str().unwrap();

    let mod_name = mod_name.replace("-", "_");

    (crate_name.to_string(), mod_name.to_string())
}

fn dep_star(
    deps: BTreeMap<(String, String), Vec<(String, String)>>,
) -> BTreeMap<(String, String), Vec<(String, String)>> {
    let (enc, dec): (BTreeMap<_, _>, Vec<_>) = {
        let mut s = BTreeSet::new();
        for (k, v) in &deps {
            s.insert(k.clone());
            for vi in v {
                s.insert(vi.clone());
            }
        }
        let dec = s.iter().cloned().collect();
        let enc = s.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
        (enc, dec)
    };

    // use Floyd--Warshall algorithm, as #vertices is not large.

    let n = dec.len();
    let mut d = vec![vec![false; n]; n];
    for (k, v) in &deps {
        for vi in v {
            d[enc[k]][enc[vi]] = true;
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] |= d[i][k] && d[k][j];
            }
        }
    }

    d.into_iter()
        .enumerate()
        .filter_map(|(i, v)| {
            let v: Vec<_> = v
                .into_iter()
                .enumerate()
                .filter_map(
                    |(j, b)| {
                        if b {
                            Some(dec[j].clone())
                        } else {
                            None
                        }
                    },
                )
                .collect();
            if v.is_empty() {
                None
            } else {
                Some((dec[i].clone(), v))
            }
        })
        .collect()
}

pub fn parse_pub_uses(
    src: &str,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let ast = parse_file(&src)?;
    let parsed: syn::File = parse_quote! { #ast };
    let uses: Vec<_> = parsed
        .items
        .into_iter()
        .filter_map(|i| {
            if let syn::Item::Use(u) = i {
                if let syn::Visibility::Public(_) = u.vis {
                    Some(u)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let mut res = vec![];
    for use_ in uses.iter() {
        let mut cur = vec![];
        if use_.leading_colon.is_some() {
            cur.push("".to_string());
        }
        dfs_use_tree(&use_.tree, &mut cur, &mut res);
    }

    res.sort_unstable();
    res.dedup();
    let res = res.into_iter().map(|mut v| {
        let ident = v.pop().unwrap();
        let mod_name = v.pop().unwrap();
        (mod_name, ident)
    });

    Ok(res.collect())
}

fn dfs_use_tree(
    use_: &syn::UseTree,
    cur: &mut Vec<String>,
    res: &mut Vec<Vec<String>>,
) {
    use syn::UseTree::*;
    match use_ {
        Path(ref path) => {
            cur.push(path.ident.to_string());
            dfs_use_tree(&path.tree, cur, res);
            cur.pop();
        }
        Name(ref name) => {
            cur.push(name.ident.to_string());
            res.push(cur.clone());
            cur.pop();
        }
        Rename(ref rename) => {
            cur.push(rename.ident.to_string());
            res.push(cur.clone());
            cur.pop();
        }
        Glob(_) => {
            cur.push("*".to_string());
            res.push(cur.clone());
            cur.pop();
        }
        Group(ref group) => {
            for item in &group.items {
                dfs_use_tree(item, cur, res);
            }
        }
    }
}
