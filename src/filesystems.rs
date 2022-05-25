use crate::filesystem::Filesystem;
use crate::zfs::ZFS;
use crate::btrfs::BTRFS;
use crate::bcachefs::BCacheFS;

pub fn filesystem(filesystem: String) -> Box<(dyn Filesystem + 'static)> {
    match filesystem.to_lowercase().as_str() {
        "zfs" => Box::new(ZFS {}),
        "btrfs" => Box::new(BTRFS {}),
        "bcachefs" => Box::new(BCacheFS {}),
        _ => panic!("Invalid filesystem: {}", filesystem),
    }
}
