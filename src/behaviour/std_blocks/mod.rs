//! Collection of builtin blocks everyone can to make scripts.
pub use super::TypedBlock;
use super::{
    Block, BlockInstanceDescriptor, BlockSlot, BlockSlotDescriptor, BlockSlotRef, ReifyError,
};
use crate::{block_define, format::VariantValue};

macro_rules! StdBlocks {
    ($($block_name:ident),*) => {
        /// Describes a builtin block.
        #[derive(Debug, PartialEq, Clone, Eq, strum::EnumString, strum::Display)]
        #[strum(serialize_all = "snake_case")]
        pub enum BuiltinBlockRef {
            $(
                #[doc = concat!("Describes the [`", stringify!($block_name) ,"`] block.")]
                $block_name
            ),*
        }
    };
}

StdBlocks! {
    Int, Add, Log
}

block_define! {
    Int { v },
    description: "Returns an integer.",
    evaluate: |block: &Int| {
        block.v.just_evaluate()
    }
}

block_define! {
    Add { a, b },
    description: "Adds two numbers and returns them.",
    evaluate: |block: &Add| {
        let a = block.a.just_evaluate();
        let b = block.b.just_evaluate();

        match (a, b) {
            (VariantValue::Int(a), VariantValue::Int(b)) => {
                VariantValue::Int(a + b)
            },
            _ => panic!("Type Error!")
        }
    }
}

block_define! {
    Log { what },
    description: "Logs a value to the standard output.",
    evaluate: |block: &Log| {
        println!("LOG {:?}", block.what.just_evaluate());
        VariantValue::Void
    }
}
