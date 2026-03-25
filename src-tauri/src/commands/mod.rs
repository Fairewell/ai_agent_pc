pub mod ai;
pub mod document;
pub mod indexing;
pub mod search;

pub use ai::{
    add_provider, agent_execute, create_chat_session, delete_chat_session, delete_provider,
    get_chat_messages, get_providers, list_chat_sessions, send_chat_message, test_provider,
    transform_document, update_provider,
};
pub use document::{get_file_detail, open_file};
pub use indexing::{get_index_status, start_scan};
pub use search::search_files;
