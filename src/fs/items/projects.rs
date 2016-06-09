use libc::{ENOENT, EACCES};
use fuse::{FileType, ReplyAttr, ReplyEntry, ReplyDirectory, Request, ReplyEmpty};

use fs::constants;
use fs::GoodDataFS;
use fs::helpers::create_inode_directory_attributes;
use fs::inode;
use object::{Project, ProjectCreate, ProjectCreateBody, ProjectCreateMeta, ProjectCreateContent};

use super::item;

use std::path::Path;

// TODO: This probably needs to be generated dynamically
pub const PROJECTS_ITEMS: [item::ProjectItem; 0] = [];

pub fn getattr(fs: &mut GoodDataFS, _req: &Request, ino: u64, reply: ReplyAttr) {
    match ino {
        constants::INODE_PROJECTS => {
            reply.attr(&constants::DEFAULT_TTL, &fs.get_projects_dir_attributes())
        }
        _ => {
            let inode = inode::Inode::deserialize(ino);
            if inode.project > 0 && inode.category == constants::Category::Internal as u8 &&
               inode.item == 0 && inode.reserved == 0 {
                reply.attr(&constants::DEFAULT_TTL,
                           &create_inode_directory_attributes(ino))
            }
        }
    }
}

pub fn lookup(fs: &mut GoodDataFS, _req: &Request, _parent: u64, name: &Path, reply: ReplyEntry) {
    match name.to_str() {
        Some(constants::PROJECTS_JSON_FILENAME) => {
            reply.entry(&constants::DEFAULT_TTL,
                        &fs.get_projects_json_attributes(),
                        0);
        }
        _ => {
            let mut i: u64 = 0;
            for project in fs.client.projects_fetch_if_none() {
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

            if i < fs.client().projects().as_ref().unwrap().len() as u64 {
                let inode = (i + 1) << 48;
                let attr = create_inode_directory_attributes(inode);
                reply.entry(&constants::DEFAULT_TTL, &attr, 0);
            } else {
                reply.error(ENOENT);
            }
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
    for project in fs.client().projects().as_ref().unwrap().into_iter() {
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


pub fn create(fs: &mut GoodDataFS, name: &Path, reply: ReplyEntry) {
    match fs.client().token.clone() {
        Some(token) => {
            let project = ProjectCreate {
                project: ProjectCreateBody {
                    content: ProjectCreateContent {
                        guidedNavigation: "1".to_string(),
                        environment: "TESTING".to_string(),
                        driver: "Pg".to_string(),
                        authorizationToken: token,
                    },
                    meta: ProjectCreateMeta {
                        title: name.to_str().unwrap().to_string(),
                        summary: name.to_str().unwrap().to_string(),
                    },
                },
            };
            fs.client.create_project(project);
            fs.client.projects_fetch();

            let inode = (1000 + 1) << 48;
            let attr = create_inode_directory_attributes(inode);
            reply.entry(&constants::DEFAULT_TTL, &attr, 0);
        }
        None => reply.error(EACCES),
    }
}

pub fn rmdir(fs: &mut GoodDataFS, name: &Path, reply: ReplyEmpty) {
    let title = &name.to_str().unwrap().to_string();
    match find_project_by_title(fs.client().projects().as_ref().unwrap(), title) {
        Some(project) => {
            fs.client.delete_project(project);
            fs.client.projects_fetch();
            reply.ok();
        }
        None => {
            println!("WTF? Project not found: {}", title);
            reply.error(ENOENT);
        }
    }
}

fn find_project_by_title(projects: &Vec<Project>, title: &String) -> Option<Project> {
    for project in projects {
        if project.project().meta().title().as_ref().unwrap() == title {
            return Some(project.clone());
        }
    }
    None
}
