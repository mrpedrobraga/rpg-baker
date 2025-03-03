use futures_signals::signal_vec::MutableVec;
use hashmap_macro::hashmap;
use rpg_baker::{
    format::VariantValue,
    project::{Project, resource::ResourceLoadError},
    scripting::{
        BlockContent, BlockInstanceDescriptor, BlockScopeDescriptor, BlockSlotDescriptor,
        BlockSourceDescriptor, ScriptRecipe,
    },
};
use std::path::Path;

fn main() -> Result<(), ResourceLoadError> {
    let path = Path::new("./examples/test_project").to_path_buf();
    let mut project = Project::new(path).expect("Failed to create new project.");

    let add_two_numbers = BlockInstanceDescriptor {
        source: BlockSourceDescriptor::Builtin(rpg_baker::scripting::BuiltinBlockRef::Add),
        content: hashmap! {
            String::from("a") => BlockContent::Slot(BlockSlotDescriptor::Block(BlockInstanceDescriptor {
                source: BlockSourceDescriptor::Builtin(
                    rpg_baker::scripting::BuiltinBlockRef::Int,
                ),
                content: hashmap! {
                    String::from("v") =>
                    BlockContent::Slot(BlockSlotDescriptor::VariantValue(VariantValue::Int(1))),
                },
            })),
            String::from("b") => BlockContent::Slot(BlockSlotDescriptor::Block(BlockInstanceDescriptor {
                source: BlockSourceDescriptor::Builtin(
                    rpg_baker::scripting::BuiltinBlockRef::Int,
                ),
                content: hashmap! {
                    String::from("v") =>
                    BlockContent::Slot(BlockSlotDescriptor::VariantValue(VariantValue::Int(1))),
                },
            })),
        },
    };

    project.startup_routine = ScriptRecipe {
        blocks: BlockScopeDescriptor {
            blocks: MutableVec::new_with_values(vec![add_two_numbers]),
        },
    };

    project.save().expect("Failed to save!");

    Ok(())
}
