use futures_signals::signal_vec::MutableVec;
use rpg_baker::{
    format::VariantValue,
    project::{Project, resource::ResourceLoadError},
    scripting::{
        BlockInstanceDescriptor, BlockPartDescriptor, BlockScopeDescriptor, BlockSlotDescriptor,
        BlockSourceDescriptor, ScriptRecipe,
    },
};
use std::path::Path;

fn main() -> Result<(), ResourceLoadError> {
    let path = Path::new("./examples/test_project").to_path_buf();
    let mut project = Project::new(path).expect("Failed to create new project.");

    let add_two_numbers = BlockInstanceDescriptor {
        source: BlockSourceDescriptor::Builtin(rpg_baker::scripting::BuiltinBlockRef::Add),
        parts: vec![BlockPartDescriptor {
            phrase: vec![
                BlockSlotDescriptor::Block(BlockInstanceDescriptor {
                    source: BlockSourceDescriptor::Builtin(
                        rpg_baker::scripting::BuiltinBlockRef::Int,
                    ),
                    parts: vec![BlockPartDescriptor {
                        phrase: vec![BlockSlotDescriptor::VariantValue(VariantValue::Int(1))],
                        body: None,
                    }],
                }),
                BlockSlotDescriptor::Block(BlockInstanceDescriptor {
                    source: BlockSourceDescriptor::Builtin(
                        rpg_baker::scripting::BuiltinBlockRef::Int,
                    ),
                    parts: vec![BlockPartDescriptor {
                        phrase: vec![BlockSlotDescriptor::VariantValue(VariantValue::Int(2))],
                        body: None,
                    }],
                }),
            ],
            body: None,
        }],
    };

    project.startup_routine = ScriptRecipe {
        blocks: BlockScopeDescriptor {
            blocks: MutableVec::new_with_values(vec![add_two_numbers]),
        },
    };

    project.save().expect("Failed to save!");

    Ok(())
}
