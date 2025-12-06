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

// Re-export read_file tool
pub use read_file::{
    FS_READ_FILE,
    FsReadFileArgs,
    FsReadFileOutput,
    FsReadFilePromptArgs,
    ReadFilePrompts,
};

// Re-export write_file tool
pub use write_file::{
    FS_WRITE_FILE,
    FsWriteFileArgs,
    FsWriteFileOutput,
    FsWriteFilePromptArgs,
    WriteFilePrompts,
};

// Re-export edit_block tool
pub use edit_block::{
    FS_EDIT_BLOCK,
    FsEditBlockArgs,
    FsEditBlockOutput,
    FsEditBlockPromptArgs,
    EditBlockPrompts,
};

// Re-export read_multiple_files tool
pub use read_multiple_files::{
    FS_READ_MULTIPLE_FILES,
    FsReadMultipleFilesArgs,
    FsReadMultipleFilesOutput,
    FsReadMultipleFilesPromptArgs,
    ReadMultipleFilesPrompts,
    FileReadResult,
};

// Re-export list_directory tool
pub use list_directory::{
    FS_LIST_DIRECTORY,
    FsListDirectoryArgs,
    FsListDirectoryOutput,
    FsListDirectoryPromptArgs,
    ListDirectoryPrompts,
    DirectoryEntry,
};

// Re-export create_directory tool
pub use create_directory::{
    FS_CREATE_DIRECTORY,
    FsCreateDirectoryArgs,
    FsCreateDirectoryOutput,
    FsCreateDirectoryPromptArgs,
    CreateDirectoryPrompts,
};

// Re-export delete_directory tool
pub use delete_directory::{
    FS_DELETE_DIRECTORY,
    FsDeleteDirectoryArgs,
    FsDeleteDirectoryOutput,
    FsDeleteDirectoryPromptArgs,
    DeleteDirectoryPrompts,
};

// Re-export delete_file tool
pub use delete_file::{
    FS_DELETE_FILE,
    FsDeleteFileArgs,
    FsDeleteFileOutput,
    FsDeleteFilePromptArgs,
    DeleteFilePrompts,
};

// Re-export move_file tool
pub use move_file::{
    FS_MOVE_FILE,
    FsMoveFileArgs,
    FsMoveFileOutput,
    FsMoveFilePromptArgs,
    MoveFilePrompts,
};

// Re-export get_file_info tool
pub use get_file_info::{
    FS_GET_FILE_INFO,
    FsGetFileInfoArgs,
    FsGetFileInfoOutput,
    FsGetFileInfoPromptArgs,
    GetFileInfoPrompts,
};

// Re-export search tool
pub use search::{
    FS_SEARCH,
    FsSearchArgs,
    FsSearchOutput,
    FsSearchPromptArgs,
    SearchPrompts,
    FsSearchResult,
    FsSearchSnapshot,
    SearchMatch,
    FileMatchCount,
};
