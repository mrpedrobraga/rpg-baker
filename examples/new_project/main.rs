use rpg_baker::project::{
    Project,
    resource::{ExternalResource, ResourceLoadError},
};
use std::{path::Path, str::FromStr};
use uuid::Uuid;

fn main() -> Result<(), ResourceLoadError> {
    let path = Path::new("./examples/test_project").to_path_buf();
    let mut p = Project::load(path).expect("Failed to load project.");

    let mut room_a = ExternalResource {
        uuid: Uuid::from_str(&"3018f3cf-016e-4df5-907c-60435d033d8d").expect("Not a valid UUID!"),
        handle: None,
    };
    p.resource_database.load(&mut room_a)?;

    Ok(())
}
