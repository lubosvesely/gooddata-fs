use fuse::{FileType, ReplyAttr, ReplyEntry, ReplyDirectory, Request};

use fs::constants;
use fs::GoodDataFS;
use fs::helpers::{create_inode_directory_attributes, create_inode_file_attributes};
use fs::inode;
use gd;

use super::item;

use std::path::Path;

// TODO: This probably needs to be generated dynamically
pub const PROJECTS_ITEMS: [item::ProjectItem; 0] = [];

pub fn getattr(fs: &mut GoodDataFS, req: &Request, ino: u64, reply: ReplyAttr) {
    if ino == constants::INODE_PROJECTS {
        reply.attr(&constants::DEFAULT_TTL, &fs.get_projects_dir_attributes())
    }
}

pub fn lookup(fs: &mut GoodDataFS, _req: &Request, parent: u64, name: &Path, reply: ReplyEntry) {
    match name.to_str() {
        Some(constants::PROJECTS_JSON_FILENAME) => {
            reply.entry(&constants::DEFAULT_TTL,
                        &fs.get_projects_json_attributes(),
                        0);
        }
        _ => {
            let mut i: u64 = 0;
            let client: &gd::GoodDataClient = fs.client();
            let projects = client.projects().as_ref();
            for project in projects.unwrap().into_iter() {
                let title: &String = project.project()
                    .meta()
                    .title()
                    .as_ref()
                    .unwrap();

                if title == name.to_str().unwrap() {
                    break;
                }
                i += 1;
            }

            let inode = (i + 1) << 48;
            let attr = create_inode_directory_attributes(inode);
            reply.entry(&constants::DEFAULT_TTL, &attr, 0);
        }
    }
}

pub fn readdir(fs: &mut GoodDataFS,
               _req: &Request,
               ino: u64,
               _fh: u64,
               in_offset: u64,
               mut reply: ReplyDirectory) {

    fs.client.projects_fetch();

    let mut offset: u64 = in_offset;
    let client: &gd::GoodDataClient = fs.client();
    let projects = client.projects().as_ref();
    for project in projects.unwrap().into_iter() {
        let title: &String = project.project()
            .meta()
            .title()
            .as_ref()
            .unwrap();

        let inode = inode::Inode {
            project: (offset + 1) as u16,
            category: 0,
            item: 0,
            reserved: 0,
        };
        println!("GoodDataFS::readdir() - Adding path {:?}, inode {:?}",
                 title,
                 inode);
        // let sanitized = re.replace_all(&title[..], "_");
        reply.add(ino, in_offset, FileType::Directory, title);
        offset += 1;
    }

    reply.add(ino,
              offset,
              FileType::RegularFile,
              constants::PROJECTS_JSON_FILENAME);

    reply.ok();
    // TODO: Refactor items above in this
    // let mut offset = 0;
    //
    // // Iterate over all projects::PROJECTS_ITEMS
    // for item in items::projects::PROJECTS_ITEMS.into_iter() {
    // }
}
