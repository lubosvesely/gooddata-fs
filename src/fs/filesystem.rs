extern crate chrono;
extern crate fuse;
extern crate libc;
extern crate regex;
extern crate rustc_serialize;
extern crate time;
extern crate users;

use chrono::*;
use fuse::{Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};
use rustc_serialize::json;
use std::path::Path;

use fs;
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
        fs::ops::getattr(self, req, ino, reply)
    }

    fn lookup(&mut self, req: &Request, parent: u64, name: &Path, reply: ReplyEntry) {
        fs::ops::lookup(self, req, parent, &name, reply)
    }

    fn read(&mut self,
            req: &Request,
            ino: u64,
            fh: u64,
            offset: u64,
            size: u32,
            reply: ReplyData) {
        fs::ops::read(self, req, ino, fh, offset, size, reply)
    }

    fn readdir(&mut self, req: &Request, ino: u64, fh: u64, offset: u64, reply: ReplyDirectory) {
        fs::ops::readdir(self, req, ino, fh, offset, reply)
    }
}

impl GoodDataFS {
    pub fn mount(self, mountpoint: String) {
        const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        println!("Mounting GoodData as Filesystem ({}), mountpoint: {}",
                 VERSION,
                 mountpoint);

        fuse::mount(self, &mountpoint, &[]);
    }
}
