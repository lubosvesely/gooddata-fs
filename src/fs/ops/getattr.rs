use fuse::{Request, ReplyAttr};
use libc::ENOENT;
use rustc_serialize::json;

use fs::constants;
use fs::GoodDataFS;
use fs::helpers::create_inode_file_attributes;
use fs::flags;
use fs::inode;
use gd;
use object;

pub fn getattr(fs: &mut GoodDataFS, _req: &Request, ino: u64, reply: ReplyAttr) {
    let inode = inode::Inode::deserialize(ino);
    println!("GoodDataFS::getattr() - Getting attributes inode {} - {:?}",
             ino,
             inode);

    if ino == constants::INODE_ROOT {
        reply.attr(&constants::DEFAULT_TTL, &fs.get_root_dir_attributes());
    } else if ino == constants::INODE_PROJECTS {
        reply.attr(&constants::DEFAULT_TTL, &fs.get_projects_dir_attributes());
    } else if ino == constants::INODE_PROJECTS_JSON {
        reply.attr(&constants::DEFAULT_TTL, &fs.get_projects_file_attributes());
    } else if ino == constants::INODE_USER {
        reply.attr(&constants::DEFAULT_TTL, &fs.get_user_file_attributes());
    } else {
        if inode.project > 0 {
            if inode.reserved == 0 {
                reply.attr(&constants::DEFAULT_TTL, &fs.get_project_dir_attributes(ino));
            } else if inode.reserved == flags::ReservedFile::FeatureFlagsJson as u8 {
                let pid = (inode.project - 1) as usize;
                let project: &object::Project = &fs.client().projects().as_ref().unwrap()[pid]
                    .clone();

                let feature_flags = project.feature_flags(&mut fs.client);
                if feature_flags.is_some() {
                    let json: String = feature_flags.unwrap().into();

                    let attr = create_inode_file_attributes(ino,
                                                            json.len() as u64,
                                                            constants::DEFAULT_CREATE_TIME);
                    reply.attr(&constants::DEFAULT_TTL, &attr);
                }
            } else if inode.reserved == flags::ReservedFile::ProjectJson as u8 {
                let client: &gd::GoodDataClient = fs.client();
                let projects = client.projects().as_ref();
                let json = json::as_pretty_json(&projects.unwrap()[(inode.project - 1) as usize])
                    .to_string();

                let attr = create_inode_file_attributes(ino,
                                                        json.len() as u64,
                                                        constants::DEFAULT_CREATE_TIME);
                reply.attr(&constants::DEFAULT_TTL, &attr);
            } else if inode.reserved == flags::ReservedFile::PermissionsJson as u8 {
                let pid = (inode.project - 1) as usize;
                let project: &object::Project = &fs.client().projects().as_ref().unwrap()[pid]
                    .clone();
                let json: String = project.user_permissions(&mut fs.client).into();

                let attr = create_inode_file_attributes(ino,
                                                        json.len() as u64,
                                                        constants::DEFAULT_CREATE_TIME);
                reply.attr(&constants::DEFAULT_TTL, &attr);
            } else if inode.reserved == flags::ReservedFile::RolesJson as u8 {
                let pid = (inode.project - 1) as usize;
                let project: &object::Project = &fs.client().projects().as_ref().unwrap()[pid]
                    .clone();
                let json: String = project.user_roles(&mut fs.client).into();

                let attr = create_inode_file_attributes(ino,
                                                        json.len() as u64,
                                                        constants::DEFAULT_CREATE_TIME);
                reply.attr(&constants::DEFAULT_TTL, &attr);
            }
        } else {
            println!("GoodDataFS::getattr() - Not found inode {:?}", ino);
            reply.error(ENOENT);
        }
    }
}
