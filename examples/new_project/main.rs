use rpg_baker::project::{
    Project,
    resource::{ExternalResource, ResourceLoadError},
};
use std::{path::Path, str::FromStr};
use uuid::Uuid;

fn main() -> Result<(), ResourceLoadError> {
    let path = Path::new("./examples/test_project").to_path_buf();
    let create_project = false;

    if create_project {
        Project::new(path).expect("Failed to create new project.");
    } else {
        let mut p = Project::load(path).expect("Failed to load project.");
        let mut room_a = ExternalResource {
            uuid: Uuid::from_str(&"3731293d-c748-453c-ba7d-091e8bc1b6fe")
                .expect("Not a valid UUID!"),
            handle: None,
        };
        p.resource_database.load(&mut room_a)?;
        dbg!(room_a);
    }

    Ok(())
}
