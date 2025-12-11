//! Filesystem category module

pub mod shared;
pub mod read_file;
pub mod write_file;
pub mod edit_block;
pub mod read_multiple_files;
pub mod list_directory;
pub mod create_directory;
pub mod delete_directory;
pub mod delete_file;
pub mod move_file;
pub mod get_file_info;
pub mod search;

// Re-export shared types
pub use shared::*;

// Re-export tool name constants from kodegen_config
pub use kodegen_config::{
    FS_CREATE_DIRECTORY,
    FS_DELETE_DIRECTORY,
    FS_DELETE_FILE,
    FS_EDIT_BLOCK,
    FS_GET_FILE_INFO,
    FS_LIST_DIRECTORY,
    FS_MOVE_FILE,
    FS_READ_FILE,
    FS_READ_MULTIPLE_FILES,
    FS_SEARCH,
    FS_WRITE_FILE,
};

// Re-export read_file tool types
pub use read_file::{
    FsReadFileArgs,
    FsReadFileOutput,
    FsReadFilePromptArgs,
    ReadFilePrompts,
};

// Re-export write_file tool types
pub use write_file::{
    FsWriteFileArgs,
    FsWriteFileOutput,
    FsWriteFilePromptArgs,
    WriteFilePrompts,
};

// Re-export edit_block tool types
pub use edit_block::{
    FsEditBlockArgs,
    FsEditBlockOutput,
    FsEditBlockPromptArgs,
    EditBlockPrompts,
};

// Re-export read_multiple_files tool types
pub use read_multiple_files::{
    FsReadMultipleFilesArgs,
    FsReadMultipleFilesOutput,
    FsReadMultipleFilesPromptArgs,
    ReadMultipleFilesPrompts,
    FileReadResult,
};

// Re-export list_directory tool types
pub use list_directory::{
    FsListDirectoryArgs,
    FsListDirectoryOutput,
    FsListDirectoryPromptArgs,
    ListDirectoryPrompts,
    DirectoryEntry,
};

// Re-export create_directory tool types
pub use create_directory::{
    FsCreateDirectoryArgs,
    FsCreateDirectoryOutput,
    FsCreateDirectoryPromptArgs,
    CreateDirectoryPrompts,
};

// Re-export delete_directory tool types
pub use delete_directory::{
    FsDeleteDirectoryArgs,
    FsDeleteDirectoryOutput,
    FsDeleteDirectoryPromptArgs,
    DeleteDirectoryPrompts,
};

// Re-export delete_file tool types
pub use delete_file::{
    FsDeleteFileArgs,
    FsDeleteFileOutput,
    FsDeleteFilePromptArgs,
    DeleteFilePrompts,
};

// Re-export move_file tool types
pub use move_file::{
    FsMoveFileArgs,
    FsMoveFileOutput,
    FsMoveFilePromptArgs,
    MoveFilePrompts,
};

// Re-export get_file_info tool types
pub use get_file_info::{
    FsGetFileInfoArgs,
    FsGetFileInfoOutput,
    FsGetFileInfoPromptArgs,
    GetFileInfoPrompts,
};

// Re-export search tool types
pub use search::{
    FsSearchArgs,
    FsSearchOutput,
    FsSearchPromptArgs,
    SearchPrompts,
    FsSearchResult,
    FsSearchSnapshot,
    SearchMatch,
    FileMatchCount,
};

// Note: FsPatternMode is already re-exported via `pub use shared::*;` above
