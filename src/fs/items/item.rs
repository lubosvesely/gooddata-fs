use fuse::FileType;

// use fs::constants;

// Project Folder Item
pub struct ProjectItem {
    pub category: u8,
    pub reserved: u8,
    pub item_type: FileType,
    pub path: &'static str,
}
