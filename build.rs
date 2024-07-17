use std::{env, fs, path::PathBuf};


fn main() {
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let mut dict_dir = PathBuf::from(manifest);
    dict_dir.push("target");
    dict_dir.push(profile);
    dict_dir.push("cmudict-master");

    fs::create_dir_all(&dict_dir).expect("Unable to create dictionary data.");

    if let Ok(files) = fs::read_dir("cmudict-master") {
        for file in files.filter_map(Result::ok) {
            let filepath = file.path();
            let filename = filepath.file_name().unwrap();
            let destpath =  dict_dir.join(filename);
            fs::copy(filepath, destpath).expect("Failed to copy file.");
        }
    }
}