use fuse::FileType;

use fs::constants;
use fs::flags;

pub struct ProjectItem {
    pub category: u8,
    pub reserved: u8,
    pub item_type: FileType,
    pub path: &'static str,
}

pub const PROJECT_FEATURE_FLAGS_JSON: ProjectItem = ProjectItem {
    category: flags::Category::Internal as u8,
    reserved: flags::ReservedFile::FeatureFlagsJson as u8,
    item_type: FileType::RegularFile,
    path: constants::FEATURE_FLAGS_JSON_FILENAME,
};

pub const PROJECT_PERMISSIONS_JSON: ProjectItem = ProjectItem {
    category: flags::Category::Internal as u8,
    reserved: flags::ReservedFile::PermissionsJson as u8,
    item_type: FileType::RegularFile,
    path: constants::USER_PERMISSIONS_JSON_FILENAME,
};

pub const PROJECT_JSON: ProjectItem = ProjectItem {
    category: flags::Category::Internal as u8,
    reserved: flags::ReservedFile::ProjectJson as u8,
    item_type: FileType::RegularFile,
    path: constants::PROJECT_JSON_FILENAME,
};

pub const USER_ROLES_JSON: ProjectItem = ProjectItem {
    category: flags::Category::Ldm as u8,
    reserved: flags::ReservedFile::RolesJson as u8,
    item_type: FileType::RegularFile,
    path: constants::USER_ROLES_JSON_FILENAME,
};

pub const LDM_DIR: ProjectItem = ProjectItem {
    category: flags::Category::Internal as u8,
    reserved: flags::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: constants::PROJECT_LDM_DIR,
};

pub const METADATA_DIR: ProjectItem = ProjectItem {
    category: flags::Category::Metadata as u8,
    reserved: flags::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: constants::PROJECT_METADATA_DIR,
};

pub const ITEMS: [ProjectItem; 6] = [PROJECT_FEATURE_FLAGS_JSON,
                                     PROJECT_PERMISSIONS_JSON,
                                     PROJECT_JSON,
                                     USER_ROLES_JSON,
                                     LDM_DIR,
                                     METADATA_DIR];
