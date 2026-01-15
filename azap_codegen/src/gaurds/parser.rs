use anyhow::{bail, Result};
use std::{
    collections::HashMap,
    ffi::OsStr,
    fmt::{self, Display},
    fs,
    path::{Component, Path, PathBuf},
    str::FromStr,
};
use syn::{
    parse::{Parse, ParseStream},
    Attribute, LitStr, Token,
};
use walkdir::WalkDir;

use crate::{debug_log, gaurds, GUARD_BASE_DIR};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GuardType {
    FromFn,
    FromFnWithState,
    Layer,
}

impl GuardType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FromFn => "fn",
            Self::FromFnWithState => "fn_with_state",
            Self::Layer => "layer",
        }
    }

    pub fn parse(s: &str) -> syn::Result<Self> {
        s.parse().map_err(|_| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "Unknown guard type: '{}'. Expected 'fn', 'fn_with_state', or 'layer'",
                    s
                ),
            )
        })
    }

    pub fn parse_from_attrs(attrs: &[Attribute]) -> syn::Result<Self> {
        for attr in attrs {
            dbg!(attr);
            if attr.path().is_ident("register_guard") {
                return attr.parse_args::<GuardType>();
            }
        }

        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Missing #[register_guard(guard_type = \"...\")] attribute",
        ))
    }
}

impl Parse for GuardType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // guard_type
        let key: syn::Ident = input.parse()?;
        if key != "guard_type" {
            return Err(syn::Error::new_spanned(key, "Expected 'guard_type'"));
        }

        // =
        input.parse::<Token![=]>()?;

        // "fn" | "fn_with_state" | "layer"
        let value: LitStr = input.parse()?;

        GuardType::from_str(&value.value()).map_err(|_| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "Unknown guard type: '{}'. Expected 'fn', 'fn_with_state', or 'layer'",
                    value.value()
                ),
            )
        })
    }
}

impl FromStr for GuardType {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "fn" => Ok(Self::FromFn),
            "fn_with_state" => Ok(Self::FromFnWithState),
            "layer" => Ok(Self::Layer),
            _ => Err(()),
        }
    }
}

impl fmt::Display for GuardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_str())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModulePath(String);

impl ModulePath {
    pub fn construct(path: &Path, module: &str) -> Result<Self, syn::Error> {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").map_err(|_| {
            syn::Error::new(proc_macro2::Span::call_site(), "CARGO_MANIFEST_DIR not set")
        })?;

        let guards_dir = PathBuf::from(manifest_dir).join("src").join(GUARD_BASE_DIR);

        debug_log!("{}, {}", path.display(), &guards_dir.display());

        let relative = path.strip_prefix(&guards_dir).map_err(|_| {
            syn::Error::new_spanned(
                path.display().to_string(),
                "path must be under guards directory",
            )
        })?;

        let mut modules = Vec::new();

        modules.push(GUARD_BASE_DIR.to_string());

        for comp in relative.components() {
            match comp {
                Component::Normal(name) => {
                    let name = name.to_str().ok_or_else(|| {
                        syn::Error::new(proc_macro2::Span::call_site(), "non-UTF8 path component")
                    })?;

                    // Remove `.rs` only from the *last* component
                    let name = name.strip_suffix(".rs").unwrap_or(name);

                    modules.push(name.to_owned());
                }
                _ => {}
            }
        }

        modules.push(module.to_owned());

        Ok(Self(modules.join("::")))
    }
}

impl fmt::Display for ModulePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Guard {
    pub name: syn::Ident,
    pub module_path: ModulePath,
    pub guard_type: GuardType,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GuardStore(HashMap<String, Guard>);

impl GuardStore {
    pub fn new(path: &PathBuf) -> Result<Self> {
        if !path.exists() {
            bail!("{} dir doesn't exist — skipping guards", path.display());
        }

        let mut guards: HashMap<String, Guard> = HashMap::new();

        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| {
                let path = entry.path();
                path.is_file()
                    && path.extension() == Some(OsStr::new("rs"))
                    && path.file_name() != Some(OsStr::new("mod.rs"))
            })
        {
            let content = fs::read_to_string(entry.path())?;
            let syn_tree = syn::parse_file(&content)?;

            for item in syn_tree.items {
                if let syn::Item::Fn(func) = item {
                    let guard_type = GuardType::parse_from_attrs(&func.attrs)?;

                    let fn_name = func.sig.ident.clone();
                    let module_path = ModulePath::construct(entry.path(), &fn_name.to_string())?;

                    let guard = Guard {
                        name: fn_name.clone(),
                        module_path,
                        guard_type,
                    };

                    guards.insert(fn_name.to_string(), guard);
                }
            }
        }

        Ok(GuardStore(guards))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::GUARD_BASE_DIR;

    #[test]
    fn test_guard_store() -> Result<()> {
        let gaurd_dir = PathBuf::from("/home/gauravlohar/personal/azap/examples/basic/src")
            .join(GUARD_BASE_DIR);
        let guards = GuardStore::new(&gaurd_dir)?;
        dbg!(guards);
        assert_eq!(1, 1);
        Ok(())
    }
}
