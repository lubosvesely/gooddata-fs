use fuse::{Request, ReplyData};
use libc::ENOENT;
use rustc_serialize::json;

use fs::constants;
use fs::flags;
use fs::GoodDataFS;
use fs::inode;
use gd;
use object;

pub fn read(fs: &mut GoodDataFS,
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
    if ino == constants::INODE_USER {
        let json: String = fs.client.user().clone().unwrap().into();
        reply.data(&json.as_bytes()[offset as usize..]);
    } else if ino == constants::INODE_PROJECTS_JSON {
        println!("GoodDataFS::read() - Reading {}",
                 constants::PROJECTS_JSON_FILENAME);
        let json = format!("{}\n",
                           json::as_pretty_json(&fs.client.projects()).to_string());
        // let json: String = fs.client.projects().clone().unwrap().into();
        reply.data(&json.as_bytes()[offset as usize..]);
    } else {
        let inode = inode::Inode::deserialize(ino);
        if inode.project > 0 && (inode.reserved == flags::ReservedFile::FeatureFlagsJson as u8) {
            println!("GoodDataFS::read() - Reading {}",
                     constants::FEATURE_FLAGS_JSON_FILENAME);

            let pid = (inode.project - 1) as usize;
            let project: &object::Project = &fs.client().projects().as_ref().unwrap()[pid].clone();
            let feature_flags = project.feature_flags(&mut fs.client);
            if feature_flags.is_some() {
                let json: String = feature_flags.unwrap().into();
                reply.data(&json.as_bytes()[offset as usize..]);
            }
        } else if inode.project > 0 && (inode.reserved == flags::ReservedFile::ProjectJson as u8) {
            println!("GoodDataFS::read() - Reading {}",
                     constants::PROJECT_JSON_FILENAME);

            let client: &gd::GoodDataClient = fs.client();
            let projects = client.projects().as_ref();
            let json = json::as_pretty_json(&projects.unwrap()[(inode.project - 1) as usize])
                .to_string();
            reply.data(&json.as_bytes()[offset as usize..]);
        } else if inode.project > 0 && (inode.reserved == flags::ReservedFile::PermissionsJson as u8) {
            println!("GoodDataFS::read() - Reading {}",
                     constants::PERMISSIONS_JSON_FILENAME);

            let pid = (inode.project - 1) as usize;
            let project: &object::Project = &fs.client().projects().as_ref().unwrap()[pid].clone();
            let json: String = project.user_permissions(&mut fs.client).into();
            reply.data(&json.as_bytes()[offset as usize..]);
        } else if inode.project > 0 && (inode.reserved == flags::ReservedFile::RolesJson as u8) {
            println!("GoodDataFS::read() - Reading {}",
                     constants::ROLES_JSON_FILENAME);

            let pid = (inode.project - 1) as usize;
            let project: &object::Project = &fs.client().projects().as_ref().unwrap()[pid].clone();
            let json: String = project.user_roles(&mut fs.client).into();
            reply.data(&json.as_bytes()[offset as usize..]);
        } else {
            reply.error(ENOENT);
        }
    }
}
