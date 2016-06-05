use time::Timespec;

pub const DEFAULT_BLOCKS_COUNT: u64 = 1;

pub const DEFAULT_CREATE_TIME: Timespec = Timespec {
    sec: 1381237736,
    nsec: 0,
};

pub const PROJECTS_DIRNAME: &'static str = "projects";
pub const PROJECTS_JSON_FILENAME: &'static str = "projects.json";

pub const PROJECT_LDM: &'static str = "ldm";
pub const PROJECT_METADATA: &'static str = "metadata";

pub const FEATURE_FLAGS_JSON_FILENAME: &'static str = "featureFlags.json";
pub const PERMISSIONS_JSON_FILENAME: &'static str = "permissions.json";
pub const PROJECT_JSON_FILENAME: &'static str = "project.json";
pub const ROLES_JSON_FILENAME: &'static str = "roles.json";
pub const USER_JSON_FILENAME: &'static str = "user.json";

pub const DEFAULT_DIRECTORY_PERMISSIONS: u16 = 0o755;

pub const DEFAULT_FILE_PERMISSIONS: u16 = 0o444;

pub const DEFAULT_FLAGS: u32 = 0;

pub const DEFAULT_NLINKE_COUNT: u32 = 0;

pub const DEFAULT_RDEV: u32 = 0;

pub const DEFAULT_SIZE: u64 = 0;

pub const DEFAULT_TTL: Timespec = Timespec { sec: 1, nsec: 0 };

pub const INODE_ROOT: u64 = 1;
pub const INODE_USER: u64 = 2;
pub const INODE_PROJECTS: u64 = 3;
pub const INODE_PROJECTS_JSON: u64 = 4;
