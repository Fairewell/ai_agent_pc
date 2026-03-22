pub mod document;
pub mod indexing;
pub mod search;

pub use document::{get_file_detail, open_file};
pub use indexing::{get_index_status, start_scan};
pub use search::search_files;
