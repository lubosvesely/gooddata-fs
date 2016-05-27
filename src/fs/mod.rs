extern crate chrono;
extern crate fuse;
extern crate libc;
extern crate regex;
extern crate rustc_serialize;
extern crate time;

use chrono::*;
use libc::ENOENT;
use time::Timespec;
use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr,
           ReplyDirectory};
use rustc_serialize::json;
use std::path::Path;
// use regex::Regex;

use gd;

const TTL: Timespec = Timespec { sec: 1, nsec: 0 }; // 1 second

const CREATE_TIME: Timespec = Timespec {
    sec: 1381237736,
    nsec: 0,
};    // 2013-10-08 08:56

const INODE_ROOT: u64 = 1;
const INODE_USER: u64 = 2;
const INODE_PROJECTS: u64 = 3;
const INODE_PROJECTS_JSON: u64 = 4;

const ROOT_DIR_ATTR: FileAttr = FileAttr {
    ino: INODE_ROOT,
    size: 0,
    blocks: 0,
    atime: CREATE_TIME,
    mtime: CREATE_TIME,
    ctime: CREATE_TIME,
    crtime: CREATE_TIME,
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
};

const PROJECTS_DIR_ATTR: FileAttr = FileAttr {
    ino: INODE_PROJECTS,
    size: 0,
    blocks: 0,
    atime: CREATE_TIME,
    mtime: CREATE_TIME,
    ctime: CREATE_TIME,
    crtime: CREATE_TIME,
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
};

pub struct GoodDataFS {
    pub client: gd::GoodDataClient,
}

impl Drop for GoodDataFS {
    fn drop(&mut self) {
        println!("Unmounting GoodData Filesystem");
    }
}

impl GoodDataFS {
    fn client(&self) -> &gd::GoodDataClient {
        &self.client
    }

    fn get_project_dir_attributes(&self, inode: u64) -> fuse::FileAttr {
        println!("GoodDataFS::get_project_dir_attributes() inode {}", inode);
        FileAttr {
            ino: inode,
            size: 0,
            blocks: 0,
            atime: CREATE_TIME,
            mtime: CREATE_TIME,
            ctime: CREATE_TIME,
            crtime: CREATE_TIME,
            kind: FileType::Directory,
            perm: 0o755,
            nlink: 0,
            uid: 501,
            gid: 20,
            rdev: 0,
            flags: 0,
        }
    }

    fn get_user_file_attributes(&self) -> fuse::FileAttr {
        let json = format!("{}\n",
                           json::as_pretty_json(&self.client.user()).to_string());

        let user = json::decode::<gd::AccountSetting>(&json);

        let ts = UTC.datetime_from_str(&user.unwrap().accountSetting.updated.unwrap()[..],
                               "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .timestamp();

        let updated = time::Timespec::new(ts, 0);

        FileAttr {
            ino: INODE_USER,
            size: json.len() as u64,
            blocks: 1,
            atime: updated,
            mtime: updated,
            ctime: updated,
            crtime: updated,
            kind: FileType::RegularFile,
            perm: 0o444,
            nlink: 1,
            uid: 501,
            gid: 20,
            rdev: 0,
            flags: 0,
        }
    }

    fn get_projects_file_attributes(&self) -> fuse::FileAttr {
        let json = format!("{}\n",
                           json::as_pretty_json(&self.client.projects()).to_string());

        FileAttr {
            ino: INODE_PROJECTS_JSON,
            size: json.len() as u64,
            blocks: 1,
            atime: CREATE_TIME,
            mtime: CREATE_TIME,
            ctime: CREATE_TIME,
            crtime: CREATE_TIME,
            kind: FileType::RegularFile,
            perm: 0o444,
            nlink: 1,
            uid: 501,
            gid: 20,
            rdev: 0,
            flags: 0,
        }
    }
}

impl Filesystem for GoodDataFS {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &Path, reply: ReplyEntry) {
        println!("GoodDataFS::lookup() - Reading parent {}, path {:?}",
                 parent,
                 name.to_str().unwrap());
        if parent == INODE_ROOT && name.to_str() == Some("user.json") {
            reply.entry(&TTL, &self.get_user_file_attributes(), 0);
        } else if parent == INODE_ROOT && name.to_str() == Some("projects") {
            reply.entry(&TTL, &PROJECTS_DIR_ATTR, 0);
        } else if parent == INODE_PROJECTS && name.to_str() == Some("projects.json") {
            reply.entry(&TTL, &self.get_projects_file_attributes(), 0);
        } else if parent == INODE_PROJECTS {
            let mut i: u64 = 0;
            let client: &gd::GoodDataClient = self.client();
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
            println!("GoodDataFS::lookup() - Adding path {:?}, inode {}",
                     name,
                     inode);
            let attr = FileAttr {
                ino: inode,
                size: 0,
                blocks: 0,
                atime: CREATE_TIME,
                mtime: CREATE_TIME,
                ctime: CREATE_TIME,
                crtime: CREATE_TIME,
                kind: FileType::Directory,
                perm: 0o755,
                nlink: 2,
                uid: 501,
                gid: 20,
                rdev: 0,
                flags: 0,
            };
            reply.entry(&TTL, &attr, 0);
        } else {
            let projectid = parent >> 48;
            if projectid > 0 {
                println!("GoodDataFS::lookup() - Adding project projectid {}",
                         projectid - 1);
                reply.error(ENOENT);
            } else {
                reply.error(ENOENT);
            }
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("GoodDataFS::getattr() - Reading inode {}", ino);

        if ino == INODE_ROOT {
            reply.attr(&TTL, &ROOT_DIR_ATTR);
        } else if ino == INODE_PROJECTS {
            reply.attr(&TTL, &PROJECTS_DIR_ATTR);
        } else if ino == INODE_PROJECTS_JSON {
            reply.attr(&TTL, &self.get_projects_file_attributes());
        } else if ino == INODE_USER {
            reply.attr(&TTL, &self.get_user_file_attributes());
        } else {
            let projectid = ino >> 48;
            if projectid > 0 {
                println!("GoodDataFS::getattr() - Project Specific Info {:?}",
                         projectid - 1);
                reply.attr(&TTL, &self.get_project_dir_attributes(ino));
            } else {
                println!("GoodDataFS::getattr() - Not found inode {:?}", ino);
                reply.error(ENOENT);
            }
        }
    }

    fn read(&mut self,
            _req: &Request,
            ino: u64,
            _fh: u64,
            offset: u64,
            _size: u32,
            reply: ReplyData) {
        println!("GoodDataFS::read() - Reading inode {}", ino);
        if ino == INODE_USER {
            let json = format!("{}\n",
                               json::as_pretty_json(&self.client.user()).to_string());
            reply.data(&json.as_bytes()[offset as usize..]);
        } else if ino == INODE_PROJECTS_JSON {
            println!("GoodDataFS::read() - Reading projects.json");
            let json = format!("{}\n",
                               json::as_pretty_json(&self.client.projects()).to_string());
            reply.data(&json.as_bytes()[offset as usize..]);
        } else {
            reply.error(ENOENT);
        }
    }

    fn readdir(&mut self,
               _req: &Request,
               ino: u64,
               _fh: u64,
               offset: u64,
               mut reply: ReplyDirectory) {
        println!("GoodDataFS::readdir() - Reading inode {}", ino);
        if ino == INODE_ROOT {
            if offset == 0 {
                // reply.add(INODE_ROOT, 0, FileType::Directory, ".");
                // reply.add(INODE_ROOT, 1, FileType::Directory, "..");

                reply.add(INODE_ROOT, 2, FileType::RegularFile, "user.json");
                reply.add(INODE_ROOT, 3, FileType::Directory, "projects");
            }
            reply.ok();
        } else if ino == INODE_PROJECTS {
            if offset == 0 {
                reply.add(INODE_PROJECTS, 0, FileType::Directory, ".");
                reply.add(INODE_PROJECTS, 1, FileType::Directory, "..");

                self.client.projects_fetch();

                let mut i: u64 = 0;
                let client: &gd::GoodDataClient = self.client();
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
                    reply.add(INODE_PROJECTS, i + 2, FileType::Directory, title);
                    i += 1;
                }

                reply.add(INODE_PROJECTS,
                          i + 3,
                          FileType::RegularFile,
                          "projects.json");
            }
            reply.ok();
        } else {
            let projectid = ino >> 48;
            if projectid > 0 {
                println!("GoodDataFS::readdir() - Reading project specific inode {}, projectid \
                          {:?}",
                         ino,
                         projectid - 1);

                reply.add(ino, 0, FileType::Directory, ".");
                reply.add(ino, 1, FileType::Directory, "..");

                println!("GoodDataFS::readdir() - Adding project.json");
                reply.add(ino, 2, FileType::Directory, "project.json");
            } else {
                println!("GoodDataFS::readdir() - Unknown inode {}", ino);
                reply.error(ENOENT);
            }
        }
    }
}

impl GoodDataFS {
    pub fn mount(self, mountpoint: String) {
        println!("Mounting GoodData as Filesystem, mountpoint: {}",
                 mountpoint);

        fuse::mount(self, &mountpoint, &[]);
    }
}
