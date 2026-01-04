#[derive(Debug, Clone, PartialEq, Copy)]
pub enum WorkspaceRole {
    Admin,
    Attendee,
    Guest,
}

impl From<String> for WorkspaceRole {
    fn from(s: String) -> Self {
        match s.trim().to_lowercase().as_str() {
            "admin" => WorkspaceRole::Admin,
            "attendee" => WorkspaceRole::Attendee,
            _ => WorkspaceRole::Guest,
        }
    }
}
