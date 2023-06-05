use std::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserMetadata {
    pub user_name: String,
    pub num_total: u16,
    pub ac_easy: u16,
    pub ac_medium: u16,
    pub ac_hard: u16,
}

impl fmt::Display for UserMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:18}:{:>15}\n{qs:18}:\n{empty:8}{easy:10}:{ec:>15}\n{empty:8}{med:10}:{mc:>15}\n{empty:8}{hard:10}:{hc:>15}",
            "Username",
            self.user_name,
            ec = self.ac_easy,
            mc = self.ac_medium,
            hc = self.ac_hard,
            qs = "Questions Solved",
            easy = "- Easy",
            med = "- Medium",
            hard = "- Hard",
            empty = ""
        )
    }
}
