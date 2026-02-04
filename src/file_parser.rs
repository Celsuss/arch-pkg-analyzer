use regex::Regex;
use std::fs;
use std::path::Path;

/// Parse `desc` file
///
/// Return a map with description
pub fn print_pkg_names(path: &Path) {
    let filename = String::from("desc");

    if path.join(&filename).exists() {
        let contents = fs::read_to_string(path.join(&filename))
            .expect("Should have been able to read the file");

        let regex = Regex::new(r"%NAME%\s*(.*)\s*%VERSION%\s*(.*)").unwrap();
        if let Some(captures) = regex.captures(&contents) {
            let pkg_name = captures.get(1).map_or("", |m| m.as_str());
            let pkg_version = captures.get(2).map_or("", |m| m.as_str()).trim();
            let pkg = String::from(pkg_name) + ": " + pkg_version;
            println!("{}", pkg);
        } else {
            println!("Package name and version not found in the file.");
        }
    } else {
        let path_str = path.to_str().unwrap_or("Invalid path");
        println!("Can't open file: {}", path_str);
    }
}
