use convert_case::{Case, Casing};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

const COMMON: &str = include_str!("build/common.rs");
const LEPTOS_COMPONENT: &str = include_str!("build/leptos_component.rs");

#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrObject {
    String(String),
    Object(HashMap<String, String>),
}

fn main() {
    println!("cargo:rerun-if-changed=build/");
    println!("cargo:rerun-if-changed=package-lock.json");
    println!("cargo:rerun-if-changed=node_modules");

    Command::new(if cfg!(target_os = "windows") {
        "npm.cmd"
    } else {
        "npm"
    })
    .args(["ci", "--audit=false"])
    .status()
    .expect("Failed to get NPM's status code")
    .success()
    .then(|| ())
    .expect("Failed to install NPM's dependencies");

    let collection: HashMap<String, Vec<Vec<StringOrObject>>> = serde_json::from_reader(
        File::open(PathBuf::from(format!(
            "{}/node_modules/lucide-static/icon-nodes.json",
            env::current_dir().unwrap().to_str().unwrap()
        )))
        .unwrap(),
    )
    .unwrap();

    let lib_content = [
        COMMON.to_owned(),
        collection
            .iter()
            .map(|(name, markup)| {
                let mut tags = String::new();
                for markup in markup {
                    tags.push_str("<");
                    for markup in markup {
                        match markup {
                            StringOrObject::String(s) => tags.push_str(&s),
                            StringOrObject::Object(o) => {
                                for (k, v) in o {
                                    tags.push_str(&k);
                                    tags.push_str("=\"");
                                    tags.push_str(&v);
                                    tags.push_str("\" ");
                                }
                            }
                        }
                        tags.push(' ');
                    }
                    tags.push_str("/>");
                }

                let markup = LEPTOS_COMPONENT
                    .replace("__component_fn", &name.to_case(Case::Pascal))
                    .replace("__icon_paths", &tags)
                    .replace("__component_name", &name);

                format!("\n{markup}\n",)
            })
            .collect::<String>(),
    ]
    .concat();

    fs::create_dir_all("src").expect("Cannot create src");
    fs::write("src/lib.rs", lib_content.as_bytes()).expect("cannot write to lib.rs file");
}
