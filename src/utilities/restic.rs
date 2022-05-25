use crate::utility;

pub struct Restic {}

impl utility::Utility for Restic {
    fn backup(&self, backup_path: &String, backup_name: &String, backup_options: &String) -> bool {
        true
    }
}
