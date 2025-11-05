// Module declarations for files in the lib subdirectory
#[path = "lib/github.rs"]
pub mod github;

#[path = "lib/indexer.rs"]
pub mod indexer;

#[path = "lib/ratatui.rs"]
pub mod ratatui;

#[path = "lib/run_test.rs"]
pub mod run_test;

#[path = "lib/status.rs"]
pub mod status;

#[path = "lib/github.rs"]
pub mod gitcommands;

#[path = "lib/modindex.rs"]
pub mod indexerv2;

// Re-export commonly used items for easier access
pub use indexer::indexer::{Exercise, Exercises, exercises};
