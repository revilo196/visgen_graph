use std::fs::read;
use std::path::Path;

/// read a binary shader from a filepath
pub fn read_shader_file(path: &str) -> Vec<u8> {
    let path = Path::new(path);
    let display = path.display();

    match read(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(data) => data,
    }
}
