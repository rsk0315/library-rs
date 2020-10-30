use std::error::Error;
use std::io::Read;

use quote::quote;
use syn::{parse_file, parse_quote};
use syn::{Fields, ForeignItem, ImplItem, Item, TraitItem};

pub fn polish_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let mut file = std::fs::File::open(&filename)?;
    let mut src = String::new();
    file.read_to_string(&mut src)?;
    let ast = parse_file(&src)?;
    let mut parsed: syn::File = parse_quote! { #ast };
    remove_doc_attrs(&mut parsed.attrs);
    remove_doc_items(&mut parsed.items);
    let gen = quote! { #parsed };
    Ok(gen.to_string())
}

fn is_test(item: &syn::Item) -> bool {
    match item {
        Item::Const(item) => has_test(&item.attrs),
        Item::Enum(item) => has_test(&item.attrs),
        Item::ExternCrate(item) => has_test(&item.attrs),
        Item::Fn(item) => has_test(&item.attrs),
        Item::ForeignMod(item) => has_test(&item.attrs),
        Item::Impl(item) => has_test(&item.attrs),
        Item::Macro(item) => has_test(&item.attrs),
        Item::Macro2(item) => has_test(&item.attrs),
        Item::Mod(item) => has_test(&item.attrs),
        Item::Static(item) => has_test(&item.attrs),
        Item::Struct(item) => has_test(&item.attrs),
        Item::Trait(item) => has_test(&item.attrs),
        Item::TraitAlias(item) => has_test(&item.attrs),
        Item::Type(item) => has_test(&item.attrs),
        Item::Union(item) => has_test(&item.attrs),
        Item::Use(item) => has_test(&item.attrs),
        _ => false,
    }
}

fn has_test(attrs: &Vec<syn::Attribute>) -> bool {
    attrs.iter().any(|a| {
        (a.path.is_ident("cfg") && a.tokens.to_string() == "(test)")
            || a.path.is_ident("test")
    })
}

fn is_doc(attr: &syn::Attribute) -> bool {
    attr.path.is_ident("doc")
}

fn remove_doc_items(items: &mut Vec<syn::Item>) {
    items.retain(|i| !is_test(i));
    for item in items.iter_mut() {
        remove_doc_item(item);
    }
}

fn remove_doc_item(item: &mut syn::Item) {
    match item {
        Item::Const(ref mut item) => remove_doc_const(item),
        Item::Enum(ref mut item) => remove_doc_enum(item),
        Item::ExternCrate(ref mut item) => remove_doc_extern_crate(item),
        Item::Fn(ref mut item) => remove_doc_fn(item),
        Item::ForeignMod(ref mut item) => remove_doc_foreign_mod(item),
        Item::Impl(ref mut item) => remove_doc_impl(item),
        Item::Macro(ref mut item) => remove_doc_macro(item),
        Item::Macro2(ref mut item) => remove_doc_macro2(item),
        Item::Mod(ref mut item) => remove_doc_mod(item),
        Item::Static(ref mut item) => remove_doc_static(item),
        Item::Struct(ref mut item) => remove_doc_struct(item),
        Item::Trait(ref mut item) => remove_doc_trait(item),
        Item::TraitAlias(ref mut item) => remove_doc_trait_alias(item),
        Item::Type(ref mut item) => remove_doc_type(item),
        Item::Union(ref mut item) => remove_doc_union(item),
        Item::Use(ref mut item) => remove_doc_use(item),
        _ => {}
    };
}

fn remove_doc_attrs(attrs: &mut Vec<syn::Attribute>) {
    attrs.retain(|a| !is_doc(a));
}

fn remove_doc_const(item_const: &mut syn::ItemConst) {
    remove_doc_attrs(&mut item_const.attrs);
}
fn remove_doc_enum(item_enum: &mut syn::ItemEnum) {
    remove_doc_attrs(&mut item_enum.attrs);
}
fn remove_doc_extern_crate(item_extern_crate: &mut syn::ItemExternCrate) {
    remove_doc_attrs(&mut item_extern_crate.attrs);
}
fn remove_doc_fn(item_fn: &mut syn::ItemFn) {
    remove_doc_attrs(&mut item_fn.attrs);
}
fn remove_doc_foreign_mod(item_foreign_mod: &mut syn::ItemForeignMod) {
    remove_doc_attrs(&mut item_foreign_mod.attrs);
    for item in item_foreign_mod.items.iter_mut() {
        match item {
            ForeignItem::Fn(ref mut foreign_item_fn) => {
                remove_doc_attrs(&mut foreign_item_fn.attrs)
            }
            ForeignItem::Static(ref mut foreign_item_static) => {
                remove_doc_attrs(&mut foreign_item_static.attrs)
            }
            ForeignItem::Type(ref mut foreign_item_type) => {
                remove_doc_attrs(&mut foreign_item_type.attrs)
            }
            ForeignItem::Macro(ref mut foreign_item_macro) => {
                remove_doc_attrs(&mut foreign_item_macro.attrs)
            }
            _ => {}
        }
    }
}
fn remove_doc_impl(item_impl: &mut syn::ItemImpl) {
    remove_doc_attrs(&mut item_impl.attrs);
    for item in item_impl.items.iter_mut() {
        match item {
            ImplItem::Const(ref mut impl_item_const) => {
                remove_doc_attrs(&mut impl_item_const.attrs)
            }
            ImplItem::Method(ref mut impl_item_method) => {
                remove_doc_attrs(&mut impl_item_method.attrs)
            }
            ImplItem::Type(ref mut impl_item_type) => {
                remove_doc_attrs(&mut impl_item_type.attrs)
            }
            ImplItem::Macro(ref mut impl_item_macro) => {
                remove_doc_attrs(&mut impl_item_macro.attrs)
            }
            _ => {}
        };
    }
}
fn remove_doc_macro(item_macro: &mut syn::ItemMacro) {
    remove_doc_attrs(&mut item_macro.attrs);
}
fn remove_doc_macro2(item_macro2: &mut syn::ItemMacro2) {
    remove_doc_attrs(&mut item_macro2.attrs);
}
fn remove_doc_mod(item_mod: &mut syn::ItemMod) {
    remove_doc_attrs(&mut item_mod.attrs);
    if let Some((_, ref mut items)) = item_mod.content {
        remove_doc_items(items);
    }
}
fn remove_doc_static(item_static: &mut syn::ItemStatic) {
    remove_doc_attrs(&mut item_static.attrs);
}
fn remove_doc_struct(item_struct: &mut syn::ItemStruct) {
    remove_doc_attrs(&mut item_struct.attrs);
    match item_struct.fields {
        Fields::Named(ref mut fields_named) => {
            remove_doc_fields_named(fields_named);
        }
        Fields::Unnamed(ref mut fields_unnamed) => {
            remove_doc_fields_unnamed(fields_unnamed);
        }
        Fields::Unit => {}
    }
}
fn remove_doc_trait(item_trait: &mut syn::ItemTrait) {
    remove_doc_attrs(&mut item_trait.attrs);
    for item in item_trait.items.iter_mut() {
        match item {
            TraitItem::Const(ref mut trait_item_const) => {
                remove_doc_attrs(&mut trait_item_const.attrs)
            }
            TraitItem::Method(ref mut trait_item_method) => {
                remove_doc_attrs(&mut trait_item_method.attrs)
            }
            TraitItem::Type(ref mut trait_item_type) => {
                remove_doc_attrs(&mut trait_item_type.attrs)
            }
            TraitItem::Macro(ref mut trait_item_macro) => {
                remove_doc_attrs(&mut trait_item_macro.attrs)
            }
            _ => {}
        }
    }
}
fn remove_doc_trait_alias(item_trait_alias: &mut syn::ItemTraitAlias) {
    remove_doc_attrs(&mut item_trait_alias.attrs);
}
fn remove_doc_type(item_type: &mut syn::ItemType) {
    remove_doc_attrs(&mut item_type.attrs);
}
fn remove_doc_union(item_union: &mut syn::ItemUnion) {
    remove_doc_attrs(&mut item_union.attrs);
    remove_doc_fields_named(&mut item_union.fields);
}
fn remove_doc_use(item_use: &mut syn::ItemUse) {
    remove_doc_attrs(&mut item_use.attrs);
}

fn remove_doc_fields_named(fields_named: &mut syn::FieldsNamed) {
    for field in fields_named.named.iter_mut() {
        remove_doc_attrs(&mut field.attrs);
    }
}
fn remove_doc_fields_unnamed(fields_unnamed: &mut syn::FieldsUnnamed) {
    for field in fields_unnamed.unnamed.iter_mut() {
        remove_doc_attrs(&mut field.attrs);
    }
}
