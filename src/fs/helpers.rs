extern crate time;
extern crate users;

use time::Timespec;

use fuse::{FileType, FileAttr};
use fs;

pub fn default_guid() -> u32 {
    users::get_current_gid()
}

pub fn default_uid() -> u32 {
    users::get_current_uid()
}

pub fn create_inode_directory_attributes(inode: u64) -> FileAttr {
    FileAttr {
        ino: inode,
        size: fs::constants::DEFAULT_SIZE,
        blocks: fs::constants::DEFAULT_BLOCKS_COUNT,
        atime: fs::constants::DEFAULT_CREATE_TIME,
        mtime: fs::constants::DEFAULT_CREATE_TIME,
        ctime: fs::constants::DEFAULT_CREATE_TIME,
        crtime: fs::constants::DEFAULT_CREATE_TIME,
        kind: FileType::Directory,
        perm: fs::constants::DEFAULT_DIRECTORY_PERMISSIONS,
        nlink: fs::constants::DEFAULT_NLINKE_COUNT,
        uid: fs::helpers::default_uid(),
        gid: fs::helpers::default_guid(),
        rdev: fs::constants::DEFAULT_RDEV,
        flags: fs::constants::DEFAULT_FLAGS,
    }
}

pub fn create_inode_file_attributes(inode: u64, size: u64, updated: Timespec) -> FileAttr {
    FileAttr {
        ino: inode,
        size: size,
        blocks: fs::constants::DEFAULT_BLOCKS_COUNT,
        atime: updated,
        mtime: updated,
        ctime: updated,
        crtime: updated,
        kind: FileType::RegularFile,
        perm: fs::constants::DEFAULT_FILE_PERMISSIONS,
        nlink: fs::constants::DEFAULT_NLINKE_COUNT,
        uid: fs::helpers::default_uid(),
        gid: fs::helpers::default_guid(),
        rdev: fs::constants::DEFAULT_RDEV,
        flags: fs::constants::DEFAULT_FLAGS,
    }
}
