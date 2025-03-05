use futures_signals::signal_vec::MutableVec;
use rpg_baker::{
    behaviour::{BehaviourDescriptor, BlockInstanceDescriptor, BlockScopeDescriptor},
    project::{Project, resource::ResourceLoadError},
};
use std::path::Path;

macro_rules! make {
    ($type:ty => $content:tt) => {
        serde_json::from_value::<$type>(serde_json::json!($content))
    };
}

fn main() -> Result<(), ResourceLoadError> {
    let path = Path::new("./examples/test_project").to_path_buf();
    let mut project = Project::new(path).expect("Failed to create new project.");

    let add_two_numbers = make!(BlockInstanceDescriptor => {
        "source" : "builtin:change_screen",
    })
    .unwrap();

    project.startup_behaviour = BehaviourDescriptor {
        blocks: BlockScopeDescriptor {
            blocks: MutableVec::new_with_values(vec![add_two_numbers]),
        },
    };

    project.save().expect("Failed to save!");

    Ok(())
}
