mod unpack;
mod pack;

use rfd::FileDialog;
use std::env;

fn main() {
    let mut is_packing : bool = false;
    for arg in env::args_os() {
        if arg == "--pack" || arg=="-p" {
            is_packing = true;
        }
    }
    if is_packing {
        let index_game_archives = FileDialog::new()
            .add_filter("Crossout index files", &["index"])
            .set_directory("/")
            .pick_files();

        let output_folder = FileDialog::new()
            .pick_folder();
    }
    else{

    }

}