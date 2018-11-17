extern crate toml_edit;

use self::toml_edit::{value, Array, Document};

#[derive(Copy, Clone)]
pub enum CrateType {
    Static,
    Dynamic,
}

pub fn set_crate_type(manifest: &str, target: CrateType) -> String {
    let mut doc = manifest.parse::<Document>().expect("Cargo.toml malformed");
    let mut array = Array::default();
    match target {
        CrateType::Static => array.push("staticlib"),
        CrateType::Dynamic => array.push("cdylib"),
    };
    doc["lib"]["crate-type"] = value(array);
    doc.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_the_crate_type_specified() {
        let manifest = r#"
[package]
name = "cargo-crate-type"
version = "0.1.0"
authors = ["Example <example@example.com>"]

[dependencies]
toml_edit = "0.1.3"
"#;

        let expected = r#"lib = {crate-type = ["cdylib"]}

[package]
name = "cargo-crate-type"
version = "0.1.0"
authors = ["Example <example@example.com>"]

[dependencies]
toml_edit = "0.1.3"
"#;

        let new_manifest = set_crate_type(manifest, CrateType::Dynamic);
        assert_eq!(new_manifest, expected);
    }

    #[test]
    fn adds_the_another_crate_type_specified() {
        let manifest = r#"
[package]
name = "cargo-crate-type"
version = "0.1.0"
authors = ["Example <example@example.com>"]

[dependencies]
toml_edit = "0.1.3"
"#;

        let expected = r#"lib = {crate-type = ["staticlib"]}

[package]
name = "cargo-crate-type"
version = "0.1.0"
authors = ["Example <example@example.com>"]

[dependencies]
toml_edit = "0.1.3"
"#;

        let new_manifest = set_crate_type(manifest, CrateType::Static);
        assert_eq!(new_manifest, expected);
    }

    #[test]
    fn overrides_already_specified_crate_type() {
        let manifest = r#"
[package]
name = "cargo-crate-type"
version = "0.1.0"
authors = ["Example <example@example.com>"]

[dependencies]
toml_edit = "0.1.3"

[lib]
crate-type = ["lib"]
"#;

        let expected = r#"
[package]
name = "cargo-crate-type"
version = "0.1.0"
authors = ["Example <example@example.com>"]

[dependencies]
toml_edit = "0.1.3"

[lib]
crate-type = ["staticlib"]
"#;

        let new_manifest = set_crate_type(manifest, CrateType::Static);
        assert_eq!(new_manifest, expected);
    }
}
