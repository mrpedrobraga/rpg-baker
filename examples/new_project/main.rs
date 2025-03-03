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
    let project = Project::new(path).expect("Failed to create new project.");

    let test_block = BlockInstanceDescriptor {
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

    {
        let text = serde_json::to_string_pretty(&test_block);
        println!("{}", text.expect("Failed to serialize!"));

        let block = test_block.clone().reify();

        match block {
            Ok(block) => {
                dbg!(block.evaluate());
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }

    let startup_sequence = ScriptRecipe {
        content: BlockScopeDescriptor {
            blocks: MutableVec::new_with_values(vec![test_block]),
        },
    };

    Ok(())
}
