use std::fs;

fn main() {
    let mut found_css = false;
    let mut found_favicon = false;
    let mut found_script = false;
    match fs::read_dir("ui/target/public") {
        Ok(dir) => {
            for entry in dir {
                let path = entry.unwrap().path();
                let extension = path.extension().unwrap();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if extension == "css" {
                    println!("cargo:rerun-if-changed={}", path.display());
                    println!("cargo:rustc-env=STYLESHEET={}", file_name);
                    found_css = true;
                }
                if extension == "ico" {
                    println!("cargo:rerun-if-changed={}", path.display());
                    println!("cargo:rustc-env=FAVICON={}", file_name);
                    found_favicon = true;
                }
                if extension == "js" {
                    println!("cargo:rerun-if-changed={}", path.display());
                    println!("cargo:rustc-env=SCRIPT={}", file_name);
                    found_script = true;
                }
            }
        }
        Err(_) => panic!("Parcel output directory not found. Please execute `npm run build`"),
    }

    let mut panics: Vec<&str> = vec![];
    if !found_css {
        panics.push("Did not find compiled CSS!! Please execute `npm run build`");
    }
    if !found_favicon {
        panics.push("Did not find favicon!! Please execute `npm run build`");
    }
    if !found_script {
        panics.push("Did not find compiled JavaScript!! Please execute `npm run build`");
    }
    if !panics.is_empty() {
        panic!("{}", panics.join("\n"));
    }
}
