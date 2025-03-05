use rpg_baker::project::{Project, resource::ResourceLoadError};
use std::path::Path;

fn main() -> Result<(), ResourceLoadError> {
    let path = Path::new("./examples/test_project").to_path_buf();
    let project = Project::load(path)?;

    project.run_from_start();

    Ok(())
}
