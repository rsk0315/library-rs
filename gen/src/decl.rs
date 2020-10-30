use std::collections::BTreeMap;
use std::error::Error;
use std::io::{Read, Write};
use std::path::PathBuf;

use glob::glob;
use syn::{parse_file, parse_quote};

pub fn decl(src_toml: &str, dst_toml: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut res = BTreeMap::new();

    for src_toml in glob(&src_toml).unwrap() {
        let toml_path = match src_toml {
            Ok(t) => t,
            _ => continue,
        };

        let mod_path = toml_path.parent().unwrap();
        let crate_path = mod_path.parent().unwrap();

        let mod_name = mod_path.file_stem().unwrap().to_str().unwrap();
        let crate_name = crate_path.file_stem().unwrap().to_str().unwrap();

        let mod_name_ = mod_name.replace("-", "_");

        eprintln!("parsing {}/{}", &crate_name, &mod_name_);

        for ident in parse(mod_path.join("src/lib.rs"))? {
            res.insert(ident, (crate_name.to_string(), mod_name_.clone()));
        }
    }

    let toml = toml::ser::to_string(&res)?;
    let mut outfile = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&dst_toml)?;

    outfile.write_all(toml.as_bytes())?;

    Ok(())
}

fn parse(src: PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
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
