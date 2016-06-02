use fuse::{FileType, Request, ReplyDirectory};
use libc::ENOENT;

use fs::constants;
use fs::flags;
use fs::GoodDataFS;
use fs::inode;
use gd;

fn readdir_project(projectid: u16, reply: &mut ReplyDirectory) {
    let inode = inode::Inode {
        project: projectid as u16,
        category: flags::Category::Internal as u8,
        item: 0,
        reserved: flags::ReservedFile::FeatureFlagsJson as u8,
    };
    let fileinode: u64 = inode.into();
    println!("GoodDataFS::readdir() - Adding inode {} - {:?}, project {}, path {}",
             fileinode,
             &inode,
             projectid - 1,
             constants::FEATURE_FLAGS_JSON_FILENAME);
    reply.add(fileinode,
              2,
              FileType::RegularFile,
              constants::FEATURE_FLAGS_JSON_FILENAME);

    let inode = inode::Inode {
        project: projectid as u16,
        category: flags::Category::Internal as u8,
        item: 0,
        reserved: flags::ReservedFile::PermissionsJson as u8,
    };
    let fileinode: u64 = inode.into();
    println!("GoodDataFS::readdir() - Adding inode {} - {:?}, project {}, path {}",
             fileinode,
             &inode,
             projectid - 1,
             constants::PERMISSIONS_JSON_FILENAME);
    reply.add(fileinode,
              3,
              FileType::RegularFile,
              constants::PERMISSIONS_JSON_FILENAME);

    let inode = inode::Inode {
        project: projectid as u16,
        category: flags::Category::Internal as u8,
        item: 0,
        reserved: flags::ReservedFile::ProjectJson as u8,
    };
    let fileinode: u64 = inode.into();
    println!("GoodDataFS::readdir() - Adding inode {} - {:?}, project {}, path {}",
             fileinode,
             &inode,
             projectid - 1,
             constants::PROJECT_JSON_FILENAME);
    reply.add(fileinode,
              4,
              FileType::RegularFile,
              constants::PROJECT_JSON_FILENAME);

    let inode = inode::Inode {
        project: projectid as u16,
        category: flags::Category::Internal as u8,
        item: 0,
        reserved: flags::ReservedFile::RolesJson as u8,
    };
    let fileinode: u64 = inode.into();
    println!("GoodDataFS::readdir() - Adding inode {} - {:?}, project {}, path {}",
             fileinode,
             &inode,
             projectid - 1,
             constants::ROLES_JSON_FILENAME);
    reply.add(fileinode,
              5,
              FileType::RegularFile,
              constants::ROLES_JSON_FILENAME);


    let inode = inode::Inode {
        project: projectid as u16,
        category: flags::Category::Metadata as u8,
        item: 0,
        reserved: 1,
    };
    let fileinode: u64 = inode.into();
    println!("GoodDataFS::readdir() - Adding inode {} - {:?}, project {}, path \
              metadata",
             fileinode,
             &inode,
             projectid - 1);
    reply.add(fileinode, 6, FileType::Directory, "metadata");
}

pub fn readdir(fs: &mut GoodDataFS,
               _req: &Request,
               ino: u64,
               _fh: u64,
               offset: u64,
               mut reply: ReplyDirectory) {
    println!("GoodDataFS::readdir() - Reading inode {} - {:?}",
             ino,
             inode::Inode::deserialize(ino));
    if ino == constants::INODE_ROOT {
        if offset == 0 {
            reply.add(constants::INODE_ROOT,
                      2,
                      FileType::RegularFile,
                      constants::USER_JSON_FILENAME);
            reply.add(constants::INODE_ROOT,
                      3,
                      FileType::Directory,
                      constants::PROJECTS_DIRNAME);
        }
        reply.ok();
    } else if ino == constants::INODE_PROJECTS {
        if offset == 0 {
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
        reply.ok();
    } else {
        let inode = inode::Inode::deserialize(ino);
        if inode.project > 0 {
            if offset == 0 {
                readdir_project(inode.project as u16, &mut reply);
            }
            reply.ok();
        } else {
            println!("GoodDataFS::readdir() - Unknown inode {}", ino);
            reply.error(ENOENT);
        }
    }
}
