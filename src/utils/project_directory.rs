use directories::ProjectDirs;

pub fn project_directory() -> Option<ProjectDirs> {
    ProjectDirs::from("dev", "evest", env!("CARGO_PKG_NAME"))
}
