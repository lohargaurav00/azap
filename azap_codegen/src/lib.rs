use std::{
    fs,
    path::{Component, Path, PathBuf},
};
use walkdir::WalkDir;

pub(crate) mod router;

const ROUTE_BASE_DIR: &'static str = "routes";

#[derive(Debug, Clone)]
pub(crate) struct DiscoveredRoute {
    pub method: String,
    pub path: String,
    pub handler: String,
    pub module_path: String,
}

#[macro_export]
macro_rules!  debug_log {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        println!(
            "cargo:warning=[DEBUG]: {}",
            format!($fmt $(, $arg)*)
        );
    };
}

pub fn generate() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let routes_dir = PathBuf::from(&manifest_dir).join("src/routes");

    debug_log!("Found routes dir at {}", routes_dir.display());

    if !routes_dir.exists() {
        println!("cargo:warning=No src/routes directory found - skipping route generation");
        return;
    }

    let routes = discover_routes(&routes_dir);

    debug_log!("Found routes : {}", &routes.len());

    let code = router::generate_router(&routes);

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = PathBuf::from(out_dir).join("generated_routes.rs");

    debug_log!("Generated Routes Destination : {}", dest_path.display());

    fs::write(&dest_path, code).expect("Failed to write generate routes");

    // Tell Cargo to rerun if routes change
    println!("cargo::rerun-if-changed=src/routes")
}

fn discover_routes(route: &PathBuf) -> Vec<DiscoveredRoute> {
    dbg!(&route);

    let mut routes: Vec<DiscoveredRoute> = Vec::new();

    for entry in WalkDir::new(route).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if !path.is_file() || path.extension().map_or(true, |e| e != "rs") {
            continue;
        }

        // Skip mod.rs files (they just re-export)
        // TODO: handle new module structure
        if path.file_name().map_or(false, |n| n == "mod.rs") {
            continue;
        }

        if let Ok(file_routes) = parse_route_file(path, route) {
            routes.extend(file_routes);
        }
    }

    routes
}

fn parse_route_file(
    file_path: &Path,
    route_base: &Path,
) -> Result<Vec<DiscoveredRoute>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let syn_tree = syn::parse_file(&content)?;

    let mut routes: Vec<DiscoveredRoute> = Vec::new();

    for item in syn_tree.items {
        if let syn::Item::Fn(func) = item {
            let module_path = calculate_module_path(file_path, route_base, Some(ROUTE_BASE_DIR));

            if let Some(route) = extract_route_from_func(&func, &module_path) {
                routes.push(route);
            }
        }
    }

    Ok(routes)
}

fn extract_route_from_func(func: &syn::ItemFn, module_path: &str) -> Option<DiscoveredRoute> {
    for attr in &func.attrs {
        let method = attr.path().get_ident()?.to_string();

        if !["get", "post", "put", "patch", "delete"].contains(&method.as_str()) {
            continue;
        }

        let path = extract_path_from_attr(attr)?;

        return Some(DiscoveredRoute {
            method: method,
            path: path,
            handler: func.sig.ident.to_string(),
            module_path: module_path.to_string(),
        });
    }

    None
}

/// Extracts the inner path/value from a `#[attribute("...")]`.
///
/// # Examples
///
/// ```no__run
/// use syn::parse_quote;
///
/// let attr = parse_quote!(#[get("/api-route")]);
///
/// let result = azap_codegen::extract_path_from_attr(&attr);
/// assert_eq!(result, Some("/api-route".to_string()));
/// ```
pub(crate) fn extract_path_from_attr(attr: &syn::Attribute) -> Option<String> {
    if let syn::Meta::List(meta_list) = &attr.meta {
        let tokens = meta_list.tokens.to_string();
        let path = tokens.trim_matches('"').to_string();
        Some(path)
    } else {
        None
    }
}

/// Calculates a Rust module path from a full file path and a route base directory.
///
/// The function:
/// - Strips `route_base` from `file_path`
/// - Converts the remaining relative path into a Rust module path
/// - Optionally prefixes the result with a root module name
///
/// # Panics
///
/// Panics if `file_path` is not under `route_base`.
///
/// # Examples
///
/// ```no__run
/// use std::path::Path;
///
/// let file_path = Path::new(
///     "/home/gauravlohar/personal/azap/examples/basic/src/routes/users/get.rs",
/// );
///
/// let route_base = Path::new(
///     "/home/gauravlohar/personal/azap/examples/basic/src/routes",
/// );
///
/// let module_path = azap_codegen::calculate_module_path(
///     file_path,
///     route_base,
///     Some("routes"),
/// );
///
/// assert_eq!("routes::users::get", module_path);
/// ```
pub fn calculate_module_path(
    file_path: &Path,
    route_base: &Path,
    module_name: Option<&str>,
) -> String {
    let relative = file_path
        .strip_prefix(route_base)
        .expect("file_path must be under route_base");

    let module_path = calculate_module_path_from_file_path(relative);

    if let Some(root_module) = module_name {
        format!("{}::{}", root_module, module_path)
    } else {
        module_path
    }
}

/// Converts the file path to rust module path.
///
/// #Examples
///
/// ```no__run
/// use std::path::Path;
///
/// let file_path: &Path = Path::new("routes/users/get.rs");
///
/// let module_path = azap_codegen::calculate_module_path_from_file_path(file_path);
///
/// assert_eq!("routes::users::get", module_path);
/// ```
fn calculate_module_path_from_file_path(file_path: &Path) -> String {
    let mut parts: Vec<String> = Vec::new();

    for component in file_path.components() {
        if let Component::Normal(os_str) = component {
            if let Some(comp_as_str) = os_str.to_str() {
                let stripped_comp = comp_as_str.strip_suffix(".rs").unwrap_or(comp_as_str);
                parts.push(stripped_comp.to_string());
            }
        }
    }

    parts.join("::")
}

#[cfg(test)]
mod tests {

    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_discover_routes() {
        let path = PathBuf::from("/home/gauravlohar/personal/azap/examples/basic/src/routes");
        let _ = discover_routes(&path);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_extract_path_from_attr() {
        let attr: syn::Attribute = parse_quote!(#[get("/api-route")]);

        let result: Option<String> = extract_path_from_attr(&attr);
        assert_eq!(result, Some("/api-route".to_string()));
    }

    #[test]
    fn test_calculate_module_path() {
        let file_path: &Path =
            Path::new("/home/gauravlohar/personal/azap/examples/basic/src/routes/users/get.rs");

        let route_base: &Path =
            Path::new("/home/gauravlohar/personal/azap/examples/basic/src/routes");

        let module_path = calculate_module_path(file_path, route_base, Some(ROUTE_BASE_DIR));

        assert_eq!("routes::users::get", module_path);
    }

    #[test]
    fn test_calculate_module_path_from_file_path() {
        let file_path: &Path = Path::new("routes/users/get.rs");
        let module_path = calculate_module_path_from_file_path(file_path);

        assert_eq!("routes::users::get", module_path);
    }
}
