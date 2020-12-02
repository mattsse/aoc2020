use std::path::Path;

pub fn read_to_string(path: impl AsRef<Path>) -> String {
    std::fs::read_to_string(path.as_ref()).unwrap()
}
