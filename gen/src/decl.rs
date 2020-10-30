use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Write};
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

pub fn decl(src_toml: &str, dst_toml: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut decls = BTreeMap::new();
    let mut deps = BTreeMap::new();

    for src_toml in glob(&src_toml).unwrap() {
        let toml_path = match src_toml {
            Ok(t) => t,
            _ => continue,
        };

        let mod_path = toml_path.parent().unwrap();
        let crate_path = mod_path.parent().unwrap();

        let mod_name = mod_path.file_stem().unwrap().to_str().unwrap();
        let crate_name = crate_path.file_stem().unwrap().to_str().unwrap();

        let mod_name = mod_name.replace("-", "_");

        eprintln!("parsing {}/{}", &crate_name, &mod_name);

        let crate_mod = (crate_name.to_string(), mod_name.to_string());

        for ident in parse(&mod_path.join("src/lib.rs"))? {
            decls.insert(ident, crate_mod.clone());
        }

        for (dep_crate, dep_mod) in parse_dep(&toml_path)? {
            deps.entry(crate_mod.clone())
                .or_insert(vec![])
                .push((dep_crate, dep_mod));
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
                eprintln!(
                    "name: {:?}\ndirect: {:#?}\nwhole: {:#?}",
                    &k, &v, &whole
                );
                DependsMap {
                    name: k,
                    direct: v,
                    whole,
                }
            })
            .collect(),
    };

    eprintln!("{:?}", res);

    let toml = toml::ser::to_string(&res)?;
    let mut outfile = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&dst_toml)?;

    outfile.write_all(toml.as_bytes())?;

    Ok(())
}

fn parse(src: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    use syn::Item::*;

    let mut file = std::fs::File::open(&src)?;
    let mut src = String::new();
    file.read_to_string(&mut src)?;
    let ast = parse_file(&src)?;
    let parsed: syn::File = parse_quote! { #ast };

    let res = parsed
        .items
        .into_iter()
        .filter_map(|item| match item {
            Const(item) => vis_ident(&item.vis, &item.ident),
            Enum(item) => vis_ident(&item.vis, &item.ident),
            ExternCrate(_) => None,
            Fn(item) => vis_ident(&item.vis, &item.sig.ident),
            ForeignMod(_) => None,
            Impl(_) => None,
            Macro(item) if item.ident.is_some() => {
                exported_ident(&item.attrs, &item.ident.unwrap())
            }
            Macro2(_) => None, // ?
            Mod(item) => vis_ident(&item.vis, &item.ident),
            Static(item) => vis_ident(&item.vis, &item.ident),
            Struct(item) => vis_ident(&item.vis, &item.ident),
            Trait(item) => vis_ident(&item.vis, &item.ident),
            TraitAlias(item) => vis_ident(&item.vis, &item.ident),
            Type(item) => vis_ident(&item.vis, &item.ident),
            Union(item) => vis_ident(&item.vis, &item.ident),
            _ => None,
        })
        .collect();

    Ok(res)
}

fn vis_ident(vis: &syn::Visibility, ident: &syn::Ident) -> Option<String> {
    if let syn::Visibility::Public(_) = vis {
        Some(ident.to_string())
    } else {
        None
    }
}

fn exported_ident(
    attrs: &Vec<syn::Attribute>,
    ident: &syn::Ident,
) -> Option<String> {
    if attrs.iter().any(|a| a.path.is_ident("macro_export")) {
        Some(ident.to_string())
    } else {
        None
    }
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
