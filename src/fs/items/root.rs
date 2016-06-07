use fuse::{FileType, ReplyAttr, ReplyEntry, ReplyDirectory, Request};
use libc::ENOENT;

use fs::constants;
use fs::GoodDataFS;

use super::item;

use std::path::Path;

pub const PROJECTS_DIR: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Internal as u8,
    reserved: 0,
    item_type: FileType::Directory,
    path: constants::PROJECTS_DIRNAME,
};

pub const USER_JSON: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Internal as u8,
    reserved: 0,
    item_type: FileType::RegularFile,
    path: constants::USER_JSON_FILENAME,
};

pub const ROOT_ITEMS: [item::ProjectItem; 2] = [PROJECTS_DIR, USER_JSON];

pub fn getattr(fs: &mut GoodDataFS, req: &Request, ino: u64, reply: ReplyAttr) {
    reply.attr(&constants::DEFAULT_TTL, &fs.get_root_dir_attributes());
}


pub fn lookup(fs: &mut GoodDataFS, _req: &Request, parent: u64, name: &Path, reply: ReplyEntry) {
    match name.to_str() {
        Some(constants::USER_JSON_FILENAME) => {
            reply.entry(&constants::DEFAULT_TTL, &fs.get_user_json_attributes(), 0);
        }
        Some(constants::PROJECTS_DIRNAME) => {
            reply.entry(&constants::DEFAULT_TTL,
                        &fs.get_projects_dir_attributes(),
                        0);
        }
        _ => reply.error(ENOENT),
    }
}

pub fn readdir(fs: &mut GoodDataFS,
               _req: &Request,
               ino: u64,
               _fh: u64,
               in_offset: u64,
               mut reply: ReplyDirectory) {

    let mut offset = in_offset;

    // Iterate over all root::ROOT_ITEMS
    for item in ROOT_ITEMS.into_iter().skip(offset as usize) {
        reply.add(constants::INODE_PROJECTS, offset, item.item_type, item.path);

        offset += 1;
    }

    reply.ok();
}
