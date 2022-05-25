use crate::utility::Utility;
use crate::borg::Borg;
use crate::restic::Restic;

pub fn utility(utility: String) -> Box<(dyn Utility + 'static)> {
    match utility.to_lowercase().as_str() {
        "restic" => Box::new(Restic {}),
        "borg" => Box::new(Borg {}),
        _ => panic!("Invalid utility: {}", utility),
    }
}
