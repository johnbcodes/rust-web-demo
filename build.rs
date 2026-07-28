use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashSet},
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize)]
struct ManifestChunk {
    file: String,
    #[serde(default)]
    css: Vec<String>,
    #[serde(default)]
    imports: Vec<String>,
}

type Manifest = BTreeMap<String, ManifestChunk>;

fn main() {
    let public_dir = PathBuf::from("ui/target/public");
    let manifest_path = public_dir.join("manifest.json");
    println!("cargo:rerun-if-changed={}", manifest_path.display());

    let manifest_contents = fs::read_to_string(&manifest_path).unwrap_or_else(|error| {
        panic!(
            "Vite manifest not found at {} ({error}). Please execute `vp build` or `npm run build`",
            manifest_path.display()
        )
    });
    let manifest: Manifest = serde_json::from_str(&manifest_contents).unwrap_or_else(|error| {
        panic!(
            "Unable to parse Vite manifest at {} ({error}). Please execute `vp build` or `npm run build`",
            manifest_path.display()
        )
    });

    let entry_key = find_key(&manifest, "base.html").unwrap_or_else(|| {
        panic!(
            "Vite manifest does not contain the base.html entry. Please execute `vp build` or `npm run build`"
        )
    });
    let favicon_key = find_key(&manifest, "favicon.ico").unwrap_or_else(|| {
        panic!(
            "Vite manifest does not contain the favicon.ico asset. Please execute `vp build` or `npm run build`"
        )
    });

    let entry = &manifest[entry_key];
    let mut stylesheets = Vec::new();
    let mut module_preloads = Vec::new();
    let mut visited = HashSet::new();

    append_unique(&mut stylesheets, &entry.css);
    for import in &entry.imports {
        collect_imported_assets(
            import,
            &manifest,
            &mut visited,
            &mut stylesheets,
            &mut module_preloads,
        );
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set by Cargo"));
    let generated = out_dir.join("vite_assets.rs");
    fs::write(
        &generated,
        format!(
            "pub const STYLESHEETS: &[&str] = {};\n\
             pub const MODULE_PRELOADS: &[&str] = {};\n\
             pub const FAVICON: &str = {};\n\
             pub const SCRIPT: &str = {};\n",
            rust_slice(&stylesheets),
            rust_slice(&module_preloads),
            rust_string(&manifest[favicon_key].file),
            rust_string(&entry.file),
        ),
    )
    .unwrap_or_else(|error| panic!("Unable to write {} ({error})", generated.display()));

    emit_asset_rebuild_paths(&public_dir);
}

fn find_key<'a>(manifest: &'a Manifest, source_name: &str) -> Option<&'a String> {
    manifest
        .get_key_value(source_name)
        .map(|(key, _)| key)
        .or_else(|| {
            manifest.iter().find_map(|(key, chunk)| {
                (key.ends_with(source_name) || chunk.file.ends_with(source_name)).then_some(key)
            })
        })
}

fn collect_imported_assets(
    key: &str,
    manifest: &Manifest,
    visited: &mut HashSet<String>,
    stylesheets: &mut Vec<String>,
    module_preloads: &mut Vec<String>,
) {
    if !visited.insert(key.to_owned()) {
        return;
    }

    let chunk = manifest
        .get(key)
        .unwrap_or_else(|| panic!("Vite manifest entry `{key}` is referenced but missing"));
    for import in &chunk.imports {
        collect_imported_assets(import, manifest, visited, stylesheets, module_preloads);
    }
    append_unique(stylesheets, &chunk.css);
    if chunk.file.ends_with(".js") {
        module_preloads.push(chunk.file.clone());
    }
}

fn append_unique(values: &mut Vec<String>, additions: &[String]) {
    for addition in additions {
        if !values.contains(addition) {
            values.push(addition.clone());
        }
    }
}

fn emit_asset_rebuild_paths(public_dir: &Path) {
    let mut pending = vec![public_dir.to_path_buf()];
    while let Some(path) = pending.pop() {
        let entries = fs::read_dir(&path).unwrap_or_else(|error| {
            panic!(
                "Unable to read Vite output directory {} ({error})",
                path.display()
            )
        });
        for entry in entries {
            let entry = entry.expect("Vite output directory entry is readable");
            let entry_path = entry.path();
            if entry_path.is_dir() {
                pending.push(entry_path);
            } else {
                println!("cargo:rerun-if-changed={}", entry_path.display());
            }
        }
    }
}

fn rust_slice(values: &[String]) -> String {
    let values = values
        .iter()
        .map(|value| rust_string(value))
        .collect::<Vec<_>>();
    format!("&[{}]", values.join(", "))
}

fn rust_string(value: &str) -> String {
    format!("{value:?}")
}
