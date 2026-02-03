use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Parse `desc` file
///
/// Return a map with description
pub fn get_pkg_name(path: &Path) -> HashMap<String, String> {
    let filename = String::from("desc");
    let desc = HashMap::new();

    if path.join(&filename).exists() {
        let _contents = fs::read_to_string(path.join(&filename))
            .expect("Should have been able to read the file");
    }

    // Ok(desc)
    desc
}
