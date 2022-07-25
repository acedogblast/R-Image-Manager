use text_io::read;
use std::path::Path;

mod rimage;
fn main() {
    const VERSION: &str = "v0.0.1";

    println!("Starting R-image Manager. Version: {}", VERSION);

    // Promt to create new collection or open exisiting collection
    println!("Enter path of existing or new collection:");
    let path: String = read!();

    println!("Opening path: {}", path);

    if Path::new(&path).is_dir() { // supplied path exists on system
        
        // Check for meta-data file
        let path_meta: String = path.clone() + "/meta-data.rimage";
        let path_images: String = path.clone() + "/images";

        if Path::new(&path_meta).is_file() && Path::new(&path_images).is_dir() {
            // open existing collection
            rimage::load_collection(path);
        } else {
            // prompt to ask to make new collection
            println!("The path entered does not seem to be a R-image collection. Make a new collection at this path? (y/n)");
            let response: String = read!();
            if response == "y" {
                // Create new collection
                rimage::new_collection(path);
            } else {
                // Exit program
                std::process::exit(0);
            }
        }


    } else {
        // does not exist ask to make new collection
        println!("The path entered does not exist. Make a new collection at this path? (y/n)");
        let response: String = read!();

        if response == "y" {
            // Create new collection
            rimage::new_collection(path);
        } else {
            // Exit program
            std::process::exit(0);
        }
    }


}
