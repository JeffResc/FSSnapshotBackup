use crate::utility;

pub struct Restic {}

impl utility::Utility for Restic {
    fn backup(&self, _backup_path: &String, _backup_name: &String, _backup_options: &String) -> bool {
        unimplemented!("The Restic utility backend is not yet implemented!");
    }
}
