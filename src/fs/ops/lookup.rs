use fuse::{Request, ReplyEntry};
use libc::ENOENT;
use rustc_serialize::json;
use std::path::Path;

use fs::constants;
use fs::flags;
use fs::GoodDataFS;
use fs::helpers::{create_inode_directory_attributes, create_inode_file_attributes};
use fs::inode;
use gd;
use object;

fn feature_flags_json(fs: &mut GoodDataFS, inode_parent: &inode::Inode, reply: ReplyEntry) {
    let inode = inode::Inode::serialize(&inode::Inode {
        project: inode_parent.project,
        category: flags::Category::Internal as u8,
        item: 0,
        reserved: flags::ReservedFile::FeatureFlagsJson as u8,
    });

    let pid = (inode_parent.project - 1) as usize;
    let project: &object::Project = &fs.client().projects().as_ref().unwrap()[pid].clone();

    let feature_flags = project.feature_flags(&mut fs.client);
    if feature_flags.is_some() {
        let json: String = feature_flags.unwrap().into();

        let attr =
            create_inode_file_attributes(inode, json.len() as u64, constants::DEFAULT_CREATE_TIME);
        reply.entry(&constants::DEFAULT_TTL, &attr, 0);
    }
}

fn project_json(fs: &mut GoodDataFS, inode_parent: &inode::Inode, reply: ReplyEntry) {
    let inode = inode::Inode::serialize(&inode::Inode {
        project: inode_parent.project,
        category: flags::Category::Internal as u8,
        item: 0,
        reserved: flags::ReservedFile::ProjectJson as u8,
    });

    let client: &gd::GoodDataClient = fs.client();
    let projects = client.projects().as_ref();
    let json = json::as_pretty_json(&projects.unwrap()[(inode_parent.project - 1) as usize])
        .to_string();
    let attr =
        create_inode_file_attributes(inode, json.len() as u64, constants::DEFAULT_CREATE_TIME);
    reply.entry(&constants::DEFAULT_TTL, &attr, 0);
}

fn permissions_json(fs: &mut GoodDataFS, inode_parent: &inode::Inode, reply: ReplyEntry) {
    let inode = inode::Inode::serialize(&inode::Inode {
        project: inode_parent.project,
        category: flags::Category::Internal as u8,
        item: 0,
        reserved: flags::ReservedFile::PermissionsJson as u8,
    });

    let pid = (inode_parent.project - 1) as usize;
    let project: &object::Project = &fs.client().projects().as_ref().unwrap()[pid].clone();
    let json: String = project.user_permissions(&mut fs.client).into();

    let attr =
        create_inode_file_attributes(inode, json.len() as u64, constants::DEFAULT_CREATE_TIME);
    reply.entry(&constants::DEFAULT_TTL, &attr, 0);
}

fn roles_json(fs: &mut GoodDataFS, inode_parent: &inode::Inode, reply: ReplyEntry) {
    let inode = inode::Inode::serialize(&inode::Inode {
        project: inode_parent.project,
        category: flags::Category::Internal as u8,
        item: 0,
        reserved: flags::ReservedFile::RolesJson as u8,
    });

    let pid = (inode_parent.project - 1) as usize;
    let project: &object::Project = &fs.client().projects().as_ref().unwrap()[pid].clone();
    let json: String = project.user_roles(&mut fs.client).into();

    let attr =
        create_inode_file_attributes(inode, json.len() as u64, constants::DEFAULT_CREATE_TIME);
    reply.entry(&constants::DEFAULT_TTL, &attr, 0);
}

pub fn lookup(fs: &mut GoodDataFS, _req: &Request, parent: u64, name: &Path, reply: ReplyEntry) {
    println!("GoodDataFS::lookup() - Reading parent {} - {:?}, path {:?}",
             parent,
             inode::Inode::deserialize(parent),
             name.to_str().unwrap());
    if parent == constants::INODE_ROOT && name.to_str() == Some(constants::USER_JSON_FILENAME) {
        reply.entry(&constants::DEFAULT_TTL, &fs.get_user_file_attributes(), 0);
    } else if parent == constants::INODE_ROOT && name.to_str() == Some(constants::PROJECTS_DIRNAME) {
        reply.entry(&constants::DEFAULT_TTL,
                    &fs.get_projects_dir_attributes(),
                    0);
    } else if parent == constants::INODE_PROJECTS &&
       name.to_str() == Some(constants::PROJECTS_JSON_FILENAME) {
        reply.entry(&constants::DEFAULT_TTL,
                    &fs.get_projects_file_attributes(),
                    0);
    } else if parent == constants::INODE_PROJECTS {
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
        println!("GoodDataFS::lookup() - Adding path {:?}, inode {} - {:?}",
                 name,
                 inode,
                 inode::Inode::deserialize(inode));
        let attr = create_inode_directory_attributes(inode);
        reply.entry(&constants::DEFAULT_TTL, &attr, 0);
    } else {
        let inode_parent = inode::Inode::deserialize(parent);
        if inode_parent.project > 0 {
            match name.to_str() {
                Some(constants::FEATURE_FLAGS_JSON_FILENAME) => {
                    feature_flags_json(fs, &inode_parent, reply)
                }
                Some(constants::PROJECT_JSON_FILENAME) => project_json(fs, &inode_parent, reply),
                Some(constants::PERMISSIONS_JSON_FILENAME) => {
                    permissions_json(fs, &inode_parent, reply)
                }
                Some(constants::ROLES_JSON_FILENAME) => roles_json(fs, &inode_parent, reply),
                _ => reply.error(ENOENT),
            }
        } else {
            reply.error(ENOENT);
        }
    }
}
