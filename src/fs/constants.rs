use time::Timespec;

pub const DEFAULT_BLOCKS_COUNT: u64 = 1;

pub const DEFAULT_CREATE_TIME: Timespec = Timespec {
    sec: 1381237736,
    nsec: 0,
};

pub const DEFAULT_DIRECTORY_PERMISSIONS: u16 = 0o755;

pub const DEFAULT_FILE_PERMISSIONS: u16 = 0o444;

pub const DEFAULT_FLAGS: u32 = 0;

pub const DEFAULT_NLINKE_COUNT: u32 = 0;

pub const DEFAULT_RDEV: u32 = 0;

pub const DEFAULT_TTL: Timespec = Timespec { sec: 1, nsec: 0 };
