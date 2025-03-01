use std::path::Path;

use rpg_baker::project::resource::{Resource, ResourceLoadError};

fn main() -> Result<(), ResourceLoadError> {
    let path = Path::new("./examples/test_project/formats/item.json");
    let resource = Resource::load(path)?;
    dbg!(resource);
    Ok(())
}
