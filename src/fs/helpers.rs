extern crate users;

pub fn default_guid() -> u32 {
    users::get_current_gid()
}

pub fn default_uid() -> u32 {
    users::get_current_uid()
}
