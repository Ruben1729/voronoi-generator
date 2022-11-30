use vcpkg;

use serde_derive::Deserialize;
use std::fs;
use toml;

use gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::env;
use std::fs::File;
use std::path::Path;

#[derive(Deserialize)]
struct Data {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    metadata: Metadata
}

#[derive(Deserialize)]
struct Metadata {
    vcpkg: Vcpkg
}

#[derive(Deserialize)]
struct Vcpkg {
    dependencies: [String; 3]
}

fn main() {
    let filename = "Cargo.toml";

    let contents = fs::read_to_string(filename).unwrap();
    let data: Data = toml::from_str(&contents).unwrap();

    // Unwrap all vcpkg dependencies
    for package_name in data.package.metadata.vcpkg.dependencies {
        let packages = vcpkg::find_package(&package_name).unwrap();

        for lib in packages.link_paths {
            println!("cargo:rustc-link-search={}", lib.to_string_lossy());
        }
    }

    // OpenGL Generator
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

    Registry::new(Api::Gl, (3, 3), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .expect("Unable to create the bindings for OpenGl");
}