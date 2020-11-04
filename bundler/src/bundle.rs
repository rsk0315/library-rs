use std::error::Error;

use syn::{parse_file, parse_quote};

pub fn extract_uses_file(
    src: &str,
) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let ast = parse_file(&src)?;
    let parsed: syn::File = parse_quote! { #ast };
    let uses: Vec<_> = parsed
        .items
        .into_iter()
        .filter_map(|i| {
            // トップレベルの use 以外検出されませんね
            if let syn::Item::Use(u) = i {
                Some(u)
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
    res.retain(|v| v[0] == "nekolib");

    Ok(res)
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
