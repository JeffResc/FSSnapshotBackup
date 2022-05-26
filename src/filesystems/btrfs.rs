use crate::filesystem;

pub struct BTRFS {}

impl filesystem::Filesystem for BTRFS {
    fn create_snapshot(&self, _subvolume: &filesystem::Subvolume) -> filesystem::Snapshot {
        unimplemented!("The BTRFS filesystem backend is not yet implemented!");
    }

    fn delete_snapshot(&self, _snapshot: &filesystem::Snapshot) -> () {
        unimplemented!("The BTRFS filesystem backend is not yet implemented!");
    }

    fn list_snapshots(&self) -> Vec<filesystem::Snapshot> {
        unimplemented!("The BTRFS filesystem backend is not yet implemented!");
    }

    fn list_subvolumes(&self) -> Vec<filesystem::Subvolume> {
        unimplemented!("The BTRFS filesystem backend is not yet implemented!");
    }
}
