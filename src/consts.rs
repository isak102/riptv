use std::path::PathBuf;

use directories::{self, ProjectDirs};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DATA_DIRECTORY: PathBuf = {
        let proj_dirs = ProjectDirs::from("", "", "riptv").expect("Getting ProjectDirs failed");
        let data_dir = proj_dirs.data_dir();

        if !data_dir.exists() {
            std::fs::create_dir_all(&data_dir).expect(
                format!(
                    "Error creating data directory {}",
                    data_dir.to_string_lossy()
                )
                .as_str(),
            );
        }

        data_dir.to_owned()
    };
}
