use fuse::{FileType, ReplyAttr, ReplyDirectory, Request};

use fs::constants;
use fs::GoodDataFS;

use super::item;

use super::super::inode;

pub const FEATURE_FLAGS_JSON: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Internal as u8,
    reserved: constants::ReservedFile::FeatureFlagsJson as u8,
    item_type: FileType::RegularFile,
    path: constants::FEATURE_FLAGS_JSON_FILENAME,
};

pub const PROJECT_JSON: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Internal as u8,
    reserved: constants::ReservedFile::ProjectJson as u8,
    item_type: FileType::RegularFile,
    path: constants::PROJECT_JSON_FILENAME,
};

pub const PERMISSIONS_JSON: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Internal as u8,
    reserved: constants::ReservedFile::PermissionsJson as u8,
    item_type: FileType::RegularFile,
    path: constants::USER_PERMISSIONS_JSON_FILENAME,
};

pub const USER_ROLES_JSON: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Ldm as u8,
    reserved: constants::ReservedFile::RolesJson as u8,
    item_type: FileType::RegularFile,
    path: constants::USER_ROLES_JSON_FILENAME,
};

pub const LDM_DIR: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Internal as u8,
    reserved: constants::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: constants::PROJECT_LDM_DIR,
};

pub const METADATA_DIR: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Metadata as u8,
    reserved: constants::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: constants::PROJECT_METADATA_DIR,
};

pub const PROJECT_ITEMS: [item::ProjectItem; 6] =
    [FEATURE_FLAGS_JSON, PROJECT_JSON, PERMISSIONS_JSON, USER_ROLES_JSON, LDM_DIR, METADATA_DIR];

pub fn readdir(fs: &mut GoodDataFS,
               _req: &Request,
               ino: u64,
               _fh: u64,
               offset: u64,
               mut reply: ReplyDirectory) {
    let mut offset = 0;

    let inode = inode::Inode::deserialize(ino);

    if inode.category == constants::Category::Ldm as u8 &&
       inode.reserved == constants::ReservedFile::KeepMe as u8 {
        reply.ok();
        return;
    }

    if inode.category == constants::Category::Metadata as u8 &&
       inode.reserved == constants::ReservedFile::KeepMe as u8 {
        reply.ok();
        return;
    }

    let projectid = inode.project;

    // Iterate over all project::ITEMS
    for item in PROJECT_ITEMS.into_iter() {
        let inode = inode::Inode {
            project: projectid,
            category: item.category,
            item: 0,
            reserved: item.reserved,
        };

        let fileinode: u64 = inode.into();
        println!("GoodDataFS::readdir() - Adding inode {} - {:?}, project {}, path {}",
                 fileinode,
                 &inode,
                 projectid - 1,
                 item.path);

        reply.add(fileinode, offset, item.item_type, item.path);

        offset += 1;
    }
}
