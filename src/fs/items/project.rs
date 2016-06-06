use fuse::FileType;

use fs::constants;

use super::item;

pub const FEATURE_FLAGS_JSON: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Internal as u8,
    reserved: constants::ReservedFile::FeatureFlagsJson as u8,
    item_type: FileType::RegularFile,
    path: constants::FEATURE_FLAGS_JSON_FILENAME,
};

pub const PERMISSIONS_JSON: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Internal as u8,
    reserved: constants::ReservedFile::PermissionsJson as u8,
    item_type: FileType::RegularFile,
    path: constants::USER_PERMISSIONS_JSON_FILENAME,
};

pub const PROJECT_JSON: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Internal as u8,
    reserved: constants::ReservedFile::ProjectJson as u8,
    item_type: FileType::RegularFile,
    path: constants::PROJECT_JSON_FILENAME,
};

pub const USER_ROLES_JSON: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Ldm as u8,
    reserved: constants::ReservedFile::RolesJson as u8,
    item_type: FileType::RegularFile,
    path: constants::USER_ROLES_JSON_FILENAME,
};

pub const LDM_DIR: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Internal as u8,
    reserved: constants::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: constants::PROJECT_LDM_DIR,
};

pub const METADATA_DIR: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Metadata as u8,
    reserved: constants::ReservedFile::KeepMe as u8,
    item_type: FileType::Directory,
    path: constants::PROJECT_METADATA_DIR,
};

pub const ITEMS: [item::ProjectItem; 6] =
    [FEATURE_FLAGS_JSON, PERMISSIONS_JSON, PROJECT_JSON, USER_ROLES_JSON, LDM_DIR, METADATA_DIR];
