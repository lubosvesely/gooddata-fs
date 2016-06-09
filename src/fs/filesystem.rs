extern crate chrono;
extern crate fuse;
extern crate libc;
extern crate regex;
extern crate rustc_serialize;
extern crate time;
extern crate users;

use libc::ENOSYS;
use chrono::*;
use fuse::{Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};
use rustc_serialize::json;
use std::path::Path;

use std::ffi::OsStr;

use fs;
use fs::inode;
use gd;
use object;

use fs::helpers::{create_inode_directory_attributes, create_inode_file_attributes};

pub struct GoodDataFS {
    pub client: gd::GoodDataClient,
    pub users_cache: users::UsersCache,
}

impl Drop for GoodDataFS {
    fn drop(&mut self) {
        println!("Unmounting GoodData Filesystem");
    }
}

#[allow(dead_code)]
impl GoodDataFS {
    pub fn client(&self) -> &gd::GoodDataClient {
        &self.client
    }

    pub fn get_projects_dir_attributes(&self) -> fuse::FileAttr {
        create_inode_directory_attributes(fs::constants::INODE_PROJECTS)
    }

    pub fn get_root_dir_attributes(&self) -> fuse::FileAttr {
        create_inode_directory_attributes(fs::constants::INODE_ROOT)
    }

    pub fn get_user_json_attributes(&self) -> fuse::FileAttr {
        let json = format!("{}\n",
                           json::as_pretty_json(&self.client.user()).to_string());

        let user = json::decode::<object::AccountSetting>(&json);

        let ts = UTC.datetime_from_str(&user.unwrap().accountSetting.updated.unwrap()[..],
                               "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .timestamp();

        let updated = time::Timespec::new(ts, 0);

        create_inode_file_attributes(fs::constants::INODE_USER, json.len() as u64, updated)
    }

    pub fn get_projects_json_attributes(&self) -> fuse::FileAttr {
        let json = format!("{}\n",
                           json::as_pretty_json(&self.client.projects()).to_string());

        create_inode_file_attributes(fs::constants::INODE_PROJECTS_JSON,
                                     json.len() as u64,
                                     fs::constants::DEFAULT_CREATE_TIME)
    }
}

impl Filesystem for GoodDataFS {
    fn getattr(&mut self, req: &Request, ino: u64, reply: ReplyAttr) {
        let inode = inode::Inode::deserialize(ino);
        println!("GoodDataFS::getattr() - Getting attributes {} - {:?}",
                 ino,
                 inode);

        match ino {
            fs::constants::INODE_ROOT => fs::root::getattr(self, req, ino, reply),
            fs::constants::INODE_PROJECTS => fs::projects::getattr(self, req, ino, reply),
            fs::constants::INODE_PROJECTS_JSON => {
                reply.attr(&fs::constants::DEFAULT_TTL,
                           &self.get_projects_json_attributes())
            }
            fs::constants::INODE_USER => {
                reply.attr(&fs::constants::DEFAULT_TTL,
                           &self.get_user_json_attributes())
            }
            _ => {
                if inode.project > 0 && inode.reserved == 0 {
                    fs::projects::getattr(self, req, ino, reply)
                } else if inode.project > 0 {
                    fs::project::getattr(self, req, ino, reply)
                } else {
                    println!("getattr() - HAPR")
                }
            }
        }
    }

    fn lookup(&mut self, req: &Request, parent: u64, name: &Path, reply: ReplyEntry) {
        let parent_inode = inode::Inode::deserialize(parent);
        println!("GoodDataFS::lookup() - Looking up parent {} - {:?}, path: {}",
                 parent,
                 parent_inode,
                 name.to_str().unwrap());

        match parent {
            fs::constants::INODE_ROOT => fs::root::lookup(self, req, parent, name, reply),
            fs::constants::INODE_PROJECTS => fs::projects::lookup(self, req, parent, name, reply),
            _ => {
                if parent_inode.project > 0 {
                    fs::project::lookup(self, req, parent, name, reply)
                }
            }
        }
    }

    fn read(&mut self,
            req: &Request,
            ino: u64,
            fh: u64,
            offset: u64,
            size: u32,
            reply: ReplyData) {

        match ino {
            fs::constants::INODE_PROJECTS_JSON => {
                let json = format!("{}\n",
                                   json::as_pretty_json(&self.client.projects()).to_string());
                // let json: String = fs.client.projects().clone().unwrap().into();
                reply.data(&json.as_bytes()[offset as usize..]);
            }
            fs::constants::INODE_USER => {
                let json: String = self.client.user().clone().unwrap().into();
                reply.data(&json.as_bytes()[offset as usize..]);
            }
            _ => {
                let inode = inode::Inode::deserialize(ino);
                if inode.project > 0 {
                    fs::project::read(self, req, ino, fh, offset, size, reply);
                } else {
                    println!("read() - HAPR")
                }
            }
        }
    }

    fn readdir(&mut self, req: &Request, ino: u64, fh: u64, offset: u64, reply: ReplyDirectory) {
        let inode = inode::Inode::deserialize(ino);
        println!("GoodDataFS::readdir() - Reading inode {} - {:?}, fh: {}, offset: {}",
                 ino,
                 inode,
                 fh,
                 offset);

        match ino {
            fs::constants::INODE_ROOT => {
                if offset == 0 {
                    fs::root::readdir(self, req, ino, fh, offset, reply)
                }
            }
            fs::constants::INODE_PROJECTS => {
                if offset == 0 {
                    fs::projects::readdir(self, req, ino, fh, offset, reply)
                }
            }
            _ => {
                let inode: fs::inode::Inode = fs::inode::Inode::deserialize(ino);
                if inode.project > 0 {
                    fs::project::readdir(self, req, ino, fh, offset, reply)
                }
            }
        }
    }

    fn mkdir (&mut self, _req: &Request, parent: u64, name: &Path, _mode: u32, reply: ReplyEntry) {
        let parent_inode = inode::Inode::deserialize(parent);
        println!("GoodDataFS::mkdir() - Making in parent {} - {:?}, path: {}",
                 parent,
                 parent_inode,
                 name.to_str().unwrap());
        match parent {
            fs::constants::INODE_PROJECTS => fs::projects::create(self, name, reply),
            _ => reply.error(ENOSYS)
        }
    }
}

impl GoodDataFS {
    pub fn mount(self, mountpoint: String) {
        const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        println!("Mounting GoodData as Filesystem ({}), mountpoint: {}",
                 VERSION,
                 mountpoint);

        fuse::mount(self, &mountpoint, &[&OsStr::new("debug")]);
    }
}
