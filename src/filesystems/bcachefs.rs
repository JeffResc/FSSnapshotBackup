use crate::filesystem;

pub struct BCacheFS {}

impl filesystem::Filesystem for BCacheFS {
    fn create_snapshot(&self, _subvolume: &filesystem::Subvolume) -> filesystem::Snapshot {
        unimplemented!("The BCache filesystem backend is not yet implemented!");
    }

    fn delete_snapshot(&self, _snapshot: &filesystem::Snapshot) -> () {
        unimplemented!("The BCache filesystem backend is not yet implemented!");
    }

    fn list_snapshots(&self) -> Vec<filesystem::Snapshot> {
        unimplemented!("The BCache filesystem backend is not yet implemented!");
    }

    fn list_subvolumes(&self) -> Vec<filesystem::Subvolume> {
        unimplemented!("The BCache filesystem backend is not yet implemented!");
    }
}
