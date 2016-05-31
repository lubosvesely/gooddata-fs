use time::Timespec;

pub const DEFAULT_CREATE_TIME: Timespec = Timespec {
    sec: 1381237736,
    nsec: 0,
};

pub const DEFAULT_TTL: Timespec = Timespec { sec: 1, nsec: 0 };
