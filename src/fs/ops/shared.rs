use fuse::FileType;

use fs::constants;
use fs::flags;

// Category
// ID ???
// Type
// Filename/Dirname

pub const ITEMS: [(u8, u8, FileType, &'static str); 6] =
    [(flags::Category::Internal as u8,
      flags::ReservedFile::FeatureFlagsJson as u8,
      FileType::RegularFile,
      constants::FEATURE_FLAGS_JSON_FILENAME),
     (flags::Category::Internal as u8,
      flags::ReservedFile::PermissionsJson as u8,
      FileType::RegularFile,
      constants::PERMISSIONS_JSON_FILENAME),
     (flags::Category::Internal as u8,
      flags::ReservedFile::ProjectJson as u8,
      FileType::RegularFile,
      constants::PROJECT_JSON_FILENAME),
     (flags::Category::Internal as u8,
      flags::ReservedFile::RolesJson as u8,
      FileType::RegularFile,
      constants::ROLES_JSON_FILENAME),
     (flags::Category::Ldm as u8,
      flags::ReservedFile::KeepMe as u8,
      FileType::Directory,
      constants::PROJECT_LDM_DIR),
     (flags::Category::Metadata as u8,
      flags::ReservedFile::KeepMe as u8,
      FileType::Directory,
      constants::PROJECT_METADATA_DIR)];
