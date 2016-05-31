extern crate chrono;
extern crate fuse;
extern crate libc;
extern crate regex;
extern crate rustc_serialize;
extern crate time;
extern crate users;

use chrono::*;
use libc::ENOENT;
use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr,
           ReplyDirectory};
use rustc_serialize::json;
use std::path::Path;

use fs;
use gd;
use object;

const INODE_ROOT: u64 = 1;
const INODE_USER: u64 = 2;
const INODE_PROJECTS: u64 = 3;
const INODE_PROJECTS_JSON: u64 = 4;

pub struct GoodDataFS {
    pub client: gd::GoodDataClient,
    pub users_cache: users::UsersCache,
}

impl Drop for GoodDataFS {
    fn drop(&mut self) {
        println!("Unmounting GoodData Filesystem");
    }
}

fn create_inode_directory_attributes(inode: u64) -> FileAttr {
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

fn create_inode_file_attributes(inode: u64, size: u64, updated: time::Timespec) -> FileAttr {
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

#[allow(dead_code)]
impl GoodDataFS {
    fn client(&self) -> &gd::GoodDataClient {
        &self.client
    }

    fn get_project_dir_attributes(&self, inode: u64) -> fuse::FileAttr {
        println!("GoodDataFS::get_project_dir_attributes() inode {} - {:?}",
                 inode,
                 fs::inode::Inode::deserialize(inode));
        create_inode_directory_attributes(inode)
    }

    fn get_projects_dir_attributes(&self) -> fuse::FileAttr {
        create_inode_directory_attributes(INODE_PROJECTS)
    }

    fn get_root_dir_attributes(&self) -> fuse::FileAttr {
        create_inode_directory_attributes(INODE_ROOT)
    }

    fn get_user_file_attributes(&self) -> fuse::FileAttr {
        let json = format!("{}\n",
                           json::as_pretty_json(&self.client.user()).to_string());

        let user = json::decode::<object::AccountSetting>(&json);

        let ts = UTC.datetime_from_str(&user.unwrap().accountSetting.updated.unwrap()[..],
                               "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .timestamp();

        let updated = time::Timespec::new(ts, 0);

        create_inode_file_attributes(INODE_USER, json.len() as u64, updated)
    }

    fn get_projects_file_attributes(&self) -> fuse::FileAttr {
        let json = format!("{}\n",
                           json::as_pretty_json(&self.client.projects()).to_string());

        create_inode_file_attributes(INODE_PROJECTS_JSON,
                                     json.len() as u64,
                                     fs::constants::DEFAULT_CREATE_TIME)
    }

    pub fn readdir_project(&self, projectid: u16, reply: &mut ReplyDirectory) {
        let inode = fs::inode::Inode {
            project: projectid as u16,
            category: fs::flags::Category::Internal as u8,
            item: 0,
            reserved: fs::flags::ReservedFile::FeatureFlagsJson as u8,
        };
        let fileinode: u64 = inode.into();
        println!("GoodDataFS::readdir() - Adding inode {} - {:?}, project {}, path \
                  featureflags.json",
                 fileinode,
                 &inode,
                 projectid - 1);
        reply.add(fileinode, 2, FileType::RegularFile, "featureflags.json");

        let inode = fs::inode::Inode {
            project: projectid as u16,
            category: fs::flags::Category::Internal as u8,
            item: 0,
            reserved: fs::flags::ReservedFile::PermissionsJson as u8,
        };
        let fileinode: u64 = inode.into();
        println!("GoodDataFS::readdir() - Adding inode {} - {:?}, project {}, path \
                  permissions.json",
                 fileinode,
                 &inode,
                 projectid - 1);
        reply.add(fileinode, 3, FileType::RegularFile, "permissions.json");

        let inode = fs::inode::Inode {
            project: projectid as u16,
            category: fs::flags::Category::Internal as u8,
            item: 0,
            reserved: fs::flags::ReservedFile::ProjectJson as u8,
        };
        let fileinode: u64 = inode.into();
        println!("GoodDataFS::readdir() - Adding inode {} - {:?}, project {}, path \
                  project.json",
                 fileinode,
                 &inode,
                 projectid - 1);
        reply.add(fileinode, 4, FileType::RegularFile, "project.json");

        let inode = fs::inode::Inode {
            project: projectid as u16,
            category: fs::flags::Category::Internal as u8,
            item: 0,
            reserved: fs::flags::ReservedFile::RolesJson as u8,
        };
        let fileinode: u64 = inode.into();
        println!("GoodDataFS::readdir() - Adding inode {} - {:?}, project {}, path \
                  roles.json",
                 fileinode,
                 &inode,
                 projectid - 1);
        reply.add(fileinode, 5, FileType::RegularFile, "roles.json");


        // let inode = fs::inode::Inode {
        //     project: projectid as u16,
        //     category: fs::flags::Category::Metadata as u8,
        //     item: 0,
        //     reserved: 0,
        // };
        // let fileinode: u64 = inode.into();
        // println!("GoodDataFS::readdir() - Adding inode {} - {:?}, project {}, path \
        //           metadata",
        //          fileinode,
        //          &inode,
        //          projectid - 1);
        // reply.add(fileinode, 6, FileType::Directory, "metadata");
    }
}

impl Filesystem for GoodDataFS {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &Path, reply: ReplyEntry) {
        println!("GoodDataFS::lookup() - Reading parent {} - {:?}, path {:?}",
                 parent,
                 fs::inode::Inode::deserialize(parent),
                 name.to_str().unwrap());
        if parent == INODE_ROOT && name.to_str() == Some("user.json") {
            reply.entry(&fs::constants::DEFAULT_TTL,
                        &self.get_user_file_attributes(),
                        0);
        } else if parent == INODE_ROOT && name.to_str() == Some("projects") {
            reply.entry(&fs::constants::DEFAULT_TTL,
                        &self.get_projects_dir_attributes(),
                        0);
        } else if parent == INODE_PROJECTS && name.to_str() == Some("projects.json") {
            reply.entry(&fs::constants::DEFAULT_TTL,
                        &self.get_projects_file_attributes(),
                        0);
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
            println!("GoodDataFS::lookup() - Adding path {:?}, inode {} - {:?}",
                     name,
                     inode,
                     fs::inode::Inode::deserialize(inode));
            let attr = create_inode_directory_attributes(inode);
            reply.entry(&fs::constants::DEFAULT_TTL, &attr, 0);
        } else {
            let inode_parent = fs::inode::Inode::deserialize(parent);
            if inode_parent.project > 0 {
                if name.to_str() == Some("featureflags.json") {
                    let inode = fs::inode::Inode::serialize(&fs::inode::Inode {
                        project: inode_parent.project,
                        category: fs::flags::Category::Internal as u8,
                        item: 0,
                        reserved: fs::flags::ReservedFile::FeatureFlagsJson as u8,
                    });

                    let pid = (inode_parent.project - 1) as usize;
                    let project: &object::Project =
                        &self.client().projects().as_ref().unwrap()[pid].clone();

                    let feature_flags = project.feature_flags(&mut self.client);
                    if feature_flags.is_some() {
                        let json: String = feature_flags.unwrap().into();

                        let attr = create_inode_file_attributes(inode,
                                                                json.len() as u64,
                                                                fs::constants::DEFAULT_CREATE_TIME);
                        reply.entry(&fs::constants::DEFAULT_TTL, &attr, 0);
                    }
                } else if name.to_str() == Some("project.json") {
                    let inode = fs::inode::Inode::serialize(&fs::inode::Inode {
                        project: inode_parent.project,
                        category: fs::flags::Category::Internal as u8,
                        item: 0,
                        reserved: fs::flags::ReservedFile::ProjectJson as u8,
                    });

                    let client: &gd::GoodDataClient = self.client();
                    let projects = client.projects().as_ref();
                    let json =
                        json::as_pretty_json(&projects.unwrap()[(inode_parent.project - 1) as usize])
                            .to_string();
                    let attr = create_inode_file_attributes(inode,
                                                            json.len() as u64,
                                                            fs::constants::DEFAULT_CREATE_TIME);
                    reply.entry(&fs::constants::DEFAULT_TTL, &attr, 0);
                } else if name.to_str() == Some("permissions.json") {
                    let inode = fs::inode::Inode::serialize(&fs::inode::Inode {
                        project: inode_parent.project,
                        category: fs::flags::Category::Internal as u8,
                        item: 0,
                        reserved: fs::flags::ReservedFile::PermissionsJson as u8,
                    });

                    let pid = (inode_parent.project - 1) as usize;
                    let project: &object::Project =
                        &self.client().projects().as_ref().unwrap()[pid].clone();
                    let json: String = project.user_permissions(&mut self.client).into();

                    let attr = create_inode_file_attributes(inode,
                                                            json.len() as u64,
                                                            fs::constants::DEFAULT_CREATE_TIME);
                    reply.entry(&fs::constants::DEFAULT_TTL, &attr, 0);
                } else if name.to_str() == Some("roles.json") {
                    let inode = fs::inode::Inode::serialize(&fs::inode::Inode {
                        project: inode_parent.project,
                        category: fs::flags::Category::Internal as u8,
                        item: 0,
                        reserved: fs::flags::ReservedFile::RolesJson as u8,
                    });

                    let pid = (inode_parent.project - 1) as usize;
                    let project: &object::Project =
                        &self.client().projects().as_ref().unwrap()[pid].clone();
                    let json: String = project.user_roles(&mut self.client).into();

                    let attr = create_inode_file_attributes(inode,
                                                            json.len() as u64,
                                                            fs::constants::DEFAULT_CREATE_TIME);
                    reply.entry(&fs::constants::DEFAULT_TTL, &attr, 0);
                } else {
                    reply.error(ENOENT);
                }
            } else {
                reply.error(ENOENT);
            }
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        let inode = fs::inode::Inode::deserialize(ino);
        println!("GoodDataFS::getattr() - Getting attributes inode {} - {:?}",
                 ino,
                 inode);

        if ino == INODE_ROOT {
            reply.attr(&fs::constants::DEFAULT_TTL, &self.get_root_dir_attributes());
        } else if ino == INODE_PROJECTS {
            reply.attr(&fs::constants::DEFAULT_TTL,
                       &self.get_projects_dir_attributes());
        } else if ino == INODE_PROJECTS_JSON {
            reply.attr(&fs::constants::DEFAULT_TTL,
                       &self.get_projects_file_attributes());
        } else if ino == INODE_USER {
            reply.attr(&fs::constants::DEFAULT_TTL,
                       &self.get_user_file_attributes());
        } else {
            if inode.project > 0 {
                if inode.reserved == 0 {
                    reply.attr(&fs::constants::DEFAULT_TTL,
                               &self.get_project_dir_attributes(ino));
                } else if inode.reserved == fs::flags::ReservedFile::FeatureFlagsJson as u8 {
                    let pid = (inode.project - 1) as usize;
                    let project: &object::Project =
                        &self.client().projects().as_ref().unwrap()[pid].clone();

                    let feature_flags = project.feature_flags(&mut self.client);
                    if feature_flags.is_some() {
                        let json: String = feature_flags.unwrap().into();

                        let attr = create_inode_file_attributes(ino,
                                                                json.len() as u64,
                                                                fs::constants::DEFAULT_CREATE_TIME);
                        reply.attr(&fs::constants::DEFAULT_TTL, &attr);
                    }
                } else if inode.reserved == fs::flags::ReservedFile::ProjectJson as u8 {
                    let client: &gd::GoodDataClient = self.client();
                    let projects = client.projects().as_ref();
                    let json =
                        json::as_pretty_json(&projects.unwrap()[(inode.project - 1) as usize])
                            .to_string();

                    let attr = create_inode_file_attributes(ino,
                                                            json.len() as u64,
                                                            fs::constants::DEFAULT_CREATE_TIME);
                    reply.attr(&fs::constants::DEFAULT_TTL, &attr);
                } else if inode.reserved == fs::flags::ReservedFile::PermissionsJson as u8 {
                    let pid = (inode.project - 1) as usize;
                    let project: &object::Project =
                        &self.client().projects().as_ref().unwrap()[pid].clone();
                    let json: String = project.user_permissions(&mut self.client).into();

                    let attr = create_inode_file_attributes(ino,
                                                            json.len() as u64,
                                                            fs::constants::DEFAULT_CREATE_TIME);
                    reply.attr(&fs::constants::DEFAULT_TTL, &attr);
                } else if inode.reserved == fs::flags::ReservedFile::RolesJson as u8 {
                    let pid = (inode.project - 1) as usize;
                    let project: &object::Project =
                        &self.client().projects().as_ref().unwrap()[pid].clone();
                    let json: String = project.user_roles(&mut self.client).into();

                    let attr = create_inode_file_attributes(ino,
                                                            json.len() as u64,
                                                            fs::constants::DEFAULT_CREATE_TIME);
                    reply.attr(&fs::constants::DEFAULT_TTL, &attr);
                }
            } else {
                println!("GoodDataFS::getattr() - Not found inode {:?}", ino);
                reply.error(ENOENT);
            }
        }
    }

    fn read(&mut self,
            _req: &Request,
            ino: u64,
            fh: u64,
            offset: u64,
            size: u32,
            reply: ReplyData) {
        println!("GoodDataFS::read() - Reading inode {}, fh {}, offset {}, size {}",
                 ino,
                 fh,
                 offset,
                 size);
        if ino == INODE_USER {
            let json: String = self.client.user().clone().unwrap().into();
            reply.data(&json.as_bytes()[offset as usize..]);
        } else if ino == INODE_PROJECTS_JSON {
            println!("GoodDataFS::read() - Reading projects.json");
            let json = format!("{}\n",
                               json::as_pretty_json(&self.client.projects()).to_string());
            // let json: String = self.client.projects().clone().unwrap().into();
            reply.data(&json.as_bytes()[offset as usize..]);
        } else {
            let inode = fs::inode::Inode::deserialize(ino);
            if inode.project > 0 &&
               (inode.reserved == fs::flags::ReservedFile::FeatureFlagsJson as u8) {
                println!("GoodDataFS::read() - Reading featureflags.json");

                let pid = (inode.project - 1) as usize;
                let project: &object::Project = &self.client().projects().as_ref().unwrap()[pid]
                    .clone();
                let feature_flags = project.feature_flags(&mut self.client);
                if feature_flags.is_some() {
                    let json: String = feature_flags.unwrap().into();
                    reply.data(&json.as_bytes()[offset as usize..]);
                }
            } else if inode.project > 0 && (inode.reserved == fs::flags::ReservedFile::ProjectJson as u8) {
                println!("GoodDataFS::read() - Reading project.json");

                let client: &gd::GoodDataClient = self.client();
                let projects = client.projects().as_ref();
                let json = json::as_pretty_json(&projects.unwrap()[(inode.project - 1) as usize])
                    .to_string();
                reply.data(&json.as_bytes()[offset as usize..]);
            } else if inode.project > 0 &&
               (inode.reserved == fs::flags::ReservedFile::PermissionsJson as u8) {
                println!("GoodDataFS::read() - Reading permissions.json");

                let pid = (inode.project - 1) as usize;
                let project: &object::Project = &self.client().projects().as_ref().unwrap()[pid]
                    .clone();
                let json: String = project.user_permissions(&mut self.client).into();
                reply.data(&json.as_bytes()[offset as usize..]);
            } else if inode.project > 0 && (inode.reserved == fs::flags::ReservedFile::RolesJson as u8) {
                println!("GoodDataFS::read() - Reading roles.json");

                let pid = (inode.project - 1) as usize;
                let project: &object::Project = &self.client().projects().as_ref().unwrap()[pid]
                    .clone();
                let json: String = project.user_roles(&mut self.client).into();
                reply.data(&json.as_bytes()[offset as usize..]);
            } else {
                reply.error(ENOENT);
            }
        }
    }

    fn readdir(&mut self,
               _req: &Request,
               ino: u64,
               _fh: u64,
               offset: u64,
               mut reply: ReplyDirectory) {
        println!("GoodDataFS::readdir() - Reading inode {} - {:?}",
                 ino,
                 fs::inode::Inode::deserialize(ino));
        if ino == INODE_ROOT {
            if offset == 0 {
                reply.add(INODE_ROOT, 2, FileType::RegularFile, "user.json");
                reply.add(INODE_ROOT, 3, FileType::Directory, "projects");
            }
            reply.ok();
        } else if ino == INODE_PROJECTS {
            if offset == 0 {
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

                reply.add(INODE_PROJECTS_JSON,
                          i + 3,
                          FileType::RegularFile,
                          "projects.json");
            }
            reply.ok();
        } else {
            let inode = fs::inode::Inode::deserialize(ino);
            if inode.project > 0 {
                if offset == 0 {
                    self.readdir_project(inode.project as u16, &mut reply);
                }
                reply.ok();
            } else {
                println!("GoodDataFS::readdir() - Unknown inode {}", ino);
                reply.error(ENOENT);
            }
        }
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
