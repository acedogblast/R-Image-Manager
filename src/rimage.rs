use std::path::Path;

pub fn load_collection(path: String) {
    
}

pub fn new_collection(path: String) {
    // Create image dir
    let r_path: &Path = Path::new(path + "/images");
    std::fs::create_dir_all(r_path);

    // Create meta-data.rimage

}

fn rimage_server(path: String) {




}