mod calurator;
mod checker;
mod files;

pub use calurator::cn_length;
pub use checker::check_is_color;
pub use files::{list_files_in_dir, read_text, save_file};
