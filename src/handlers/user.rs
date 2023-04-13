use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserMetadata {
    pub user_name: String,
    pub num_total: u16,
    pub ac_easy: u16,
    pub ac_medium: u16,
    pub ac_hard: u16,
}

impl UserMetadata {
    pub fn display(&self) {
        println!("{}", self);
    }
}

impl std::fmt::Display for UserMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "User name        :\t{}\nQuestions Solved :\n\t- Easy   :\t{}\n\t- Medium :\t{}\n\t- Hard   :\t{}",
            self.user_name, self.ac_easy, self.ac_medium, self.ac_hard
        )
    }
}
