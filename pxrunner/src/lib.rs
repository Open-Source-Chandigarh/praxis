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
pub mod cli;

#[path = "lib/github.rs"]
pub mod github_traits;

#[path = "lib/github.rs"]
pub mod github_implementations;

#[path = "lib/github.rs"]
pub mod app_logic;

// Re-export commonly used items for easier access
pub use indexer::indexer::{Exercise, Exercises, exercises};

