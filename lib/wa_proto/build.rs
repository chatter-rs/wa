use std::{env, fs, path::Path};

use glob::glob;

extern crate prost_build;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let proto_files_basedir = Path::new("whatsmeow").join("proto");

    let mut proto_filepaths = Vec::new();

    // for entry in glob(&format!("{proto_files_basedir}/*/*.proto"))
    for entry in glob(
        proto_files_basedir
            .join("*")
            .join("*.proto")
            .to_str()
            .unwrap(),
    )
    .expect("failed to get proto files from submodule")
    {
        match entry {
            Ok(path) => proto_filepaths.push(path),
            Err(err) => panic!("error while evaluating proto paths glob: {:?}", err),
        }
    }

    prost_build::compile_protos(&proto_filepaths, &[proto_files_basedir]).unwrap();

    let mut mod_content = String::new();

    for entry in glob(out_dir.join("*.rs").to_str().unwrap())
        .expect("failed to get the compiled proto files")
    {
        match entry {
            Ok(path) => {
                let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                if file_name == "mod_items.rs" {
                    continue;
                }

                let mod_name = file_name.trim_end_matches(".rs");

                mod_content += &format!("pub mod {mod_name} {{ include!(concat!(env!(\"OUT_DIR\"), \"/{file_name}\")); }}\n")
            }
            Err(err) => panic!("error while evaluating compiled proto paths: {:?}", err),
        }
    }

    fs::write(out_dir.join("mod_items.rs"), mod_content)
        .expect("failed to write the items mod content");
}
