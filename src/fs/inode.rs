#[derive(Debug, Copy, Clone)]
pub struct Inode {
    pub project: u16,
    pub category: u8,
    pub item: u32,
    pub reserved: u8,
}

impl Into<u64> for Inode {
    fn into(self) -> u64 {
        Inode::serialize(&self)
    }
}

impl Inode {
    pub fn get_project(inode: u64) -> u16 {
        (inode >> 48) as u16
    }

    pub fn get_category(inode: u64) -> u8 {
        ((inode >> 40) & 0xff) as u8
    }

    pub fn get_item(inode: u64) -> u32 {
        ((inode >> 8) & 0xffffffff) as u32
    }

    pub fn get_reserved(inode: u64) -> u8 {
        (inode & 0xff) as u8
    }

    pub fn create(project: u16, category: u8, item: u32, reserved: u8) -> u64 {
        let inode: u64 = ((project as u64) << 48) | ((category as u64) << 40) |
                         ((item as u64) << 8) | (reserved as u64);
        return inode;
    }

    pub fn serialize(inode: &Inode) -> u64 {
        Inode::create(inode.project, inode.category, inode.item, inode.reserved)
    }

    pub fn deserialize(inode: u64) -> Inode {
        Inode {
            project: Inode::get_project(inode),
            category: Inode::get_category(inode),
            item: Inode::get_item(inode),
            reserved: Inode::get_reserved(inode),
        }
    }
}
