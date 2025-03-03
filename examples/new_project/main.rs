use futures_signals::signal_vec::MutableVec;
use rpg_baker::{
    project::{Project, resource::ResourceLoadError},
    scripting::{BlockInstanceDescriptor, BlockScopeDescriptor, ScriptRecipe},
};
use std::path::Path;

macro_rules! get {
    ($type:ty => $content:tt) => {
        serde_json::from_value::<$type>(serde_json::json!($content))
    };
}

fn main() -> Result<(), ResourceLoadError> {
    let path = Path::new("./examples/test_project").to_path_buf();
    let mut project = Project::new(path).expect("Failed to create new project.");

    let add_two_numbers = get!( BlockInstanceDescriptor => {
        "source" : "builtin:log",
        "what" : {
            "source": "builtin:add",
            "a" : {
                "source" : "builtin:int",
                "v" : 1
            },
            "b" : {
                "source" : "builtin:int",
                "v" : 2
            }
        }
    })
    .unwrap();

    project.startup_routine = ScriptRecipe {
        blocks: BlockScopeDescriptor {
            blocks: MutableVec::new_with_values(vec![add_two_numbers]),
        },
    };

    project.save().expect("Failed to save!");

    Ok(())
}
