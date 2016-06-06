use fuse::{FileType, Request, ReplyDirectory};
use libc::ENOENT;

use super::super::items;

use fs::constants;
use fs::GoodDataFS;
use fs::inode;
use gd;

pub fn readdir(fs: &mut GoodDataFS,
               _req: &Request,
               ino: u64,
               _fh: u64,
               offset: u64,
               mut reply: ReplyDirectory) {
    let inode = inode::Inode::deserialize(ino);
    println!("GoodDataFS::readdir() - Reading inode {} - {:?}",
             ino,
             inode);

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

    match ino {
        constants::INODE_ROOT => {
            if offset == 0 {
                root(&mut reply);
            }
            reply.ok();
        }
        constants::INODE_PROJECTS => {
            if offset == 0 {
                projects(fs, &mut reply);
            }
            reply.ok();
        }
        _ => {
            if inode.project > 0 {
                if offset == 0 {
                    project(inode.project as u16, &mut reply);
                }
                reply.ok();
            } else {
                println!("GoodDataFS::readdir() - Unknown inode {}", ino);
                reply.error(ENOENT);
            }
        }
    }
}

fn project(projectid: u16, reply: &mut ReplyDirectory) {
    let mut offset = 0;

    // Iterate over all project::ITEMS
    for item in items::project::PROJECT_ITEMS.into_iter() {
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

fn projects(fs: &mut GoodDataFS, reply: &mut ReplyDirectory) {
    fs.client.projects_fetch();

    let mut i: u64 = 0;
    let client: &gd::GoodDataClient = fs.client();
    let projects = client.projects().as_ref();
    for project in projects.unwrap().into_iter() {
        let title: &String = project.project()
            .meta()
            .title()
            .as_ref()
            .unwrap();

        let inode = (i + 1) << 48;
        println!("GoodDataFS::readdir() - Adding path {:?}, inode {}",
                 title,
                 inode);
        // let sanitized = re.replace_all(&title[..], "_");
        reply.add(constants::INODE_PROJECTS, i + 2, FileType::Directory, title);
        i += 1;
    }

    reply.add(constants::INODE_PROJECTS_JSON,
              i + 3,
              FileType::RegularFile,
              constants::PROJECTS_JSON_FILENAME);
}

fn root(reply: &mut ReplyDirectory) {
    reply.add(constants::INODE_ROOT,
              2,
              FileType::RegularFile,
              constants::USER_JSON_FILENAME);
    reply.add(constants::INODE_ROOT,
              3,
              FileType::Directory,
              constants::PROJECTS_DIRNAME);
}
