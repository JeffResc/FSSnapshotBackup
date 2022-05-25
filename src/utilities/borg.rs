use crate::utility;

pub struct Borg {}

impl utility::Utility for Borg {
    fn backup(&self, backup_path: &String, backup_name: &String, backup_options: &String) -> bool {
        true
    }
}
