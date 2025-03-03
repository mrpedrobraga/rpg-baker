//! Collection of builtin blocks everyone can to make scripts.

pub use super::TypedBlock;
use super::{
    Block, BlockInstanceDescriptor, BlockSlot, BlockSlotDescriptor, BlockSlotPosition,
    FromDescriptorError,
};
use crate::format::VariantValue;

/// Describes a single integer value!
pub struct Int(pub i32);

impl TypedBlock for Int {
    type Output = i32;

    fn evaluate(&self) -> Self::Output {
        self.0
    }
}

impl Block for Int {
    fn description() -> &'static str {
        "Evaluates to the number that you input here."
    }

    fn create() -> Self {
        return Self(0);
    }

    fn from_descriptor(descriptor: &BlockInstanceDescriptor) -> Result<Self, FromDescriptorError> {
        let mut block = Self::create();
        let input = descriptor
            .parts
            .get(0)
            .expect("Failed to get part 0!")
            .phrase
            .get(0)
            .expect("Failed to get slot 0!");
        match input {
            BlockSlotDescriptor::VariantValue(variant_value) => match variant_value {
                VariantValue::Int(a) => {
                    block.0 = *a;
                    Ok(block)
                }
            },
            _ => Err(FromDescriptorError::ShouldBeAVariant(
                super::BlockSlotPosition::Phrase {
                    phrase_idx: 0,
                    slot_idx: 0,
                },
            )),
        }
    }
}

/// Sums two integer-typed values!
pub struct Add {
    pub slot_a: BlockSlot<i32, i32>,
    pub slot_b: BlockSlot<i32, i32>,
}

impl TypedBlock for Add {
    type Output = i32;

    fn evaluate(&self) -> Self::Output {
        let a = self.slot_a.just_evaluate();
        let b = self.slot_b.just_evaluate();
        a + b
    }
}

impl Block for Add {
    fn description() -> &'static str {
        "Adds two numbers and returns the result."
    }

    fn create() -> Self {
        return Add {
            slot_a: BlockSlot::new(),
            slot_b: BlockSlot::new(),
        };
    }

    fn from_descriptor(descriptor: &BlockInstanceDescriptor) -> Result<Self, FromDescriptorError> {
        let mut block = Self::create();
        let part = &descriptor
            .parts
            .get(0)
            .expect("Failed to get part 0!")
            .phrase;
        let a = part.get(0).expect("Failed to get slot 0!");
        let b = part.get(1).expect("Failed to get slot 1!");
        let slot_a: BlockSlot<i32, i32> = match a {
            BlockSlotDescriptor::VariantValue(variant_value) => match variant_value {
                VariantValue::Int(a) => BlockSlot::new_with_value(*a),
            },
            BlockSlotDescriptor::Block(b) => {
                let block = b.reify().map_err(|e| {
                    FromDescriptorError::Child(
                        BlockSlotPosition::Phrase {
                            phrase_idx: 0,
                            slot_idx: 0,
                        },
                        Box::new(e),
                    )
                })?;
                let mut slot = BlockSlot::new();
                slot.try_place(Box::new(block))
                    .map_err(FromDescriptorError::BlockPlaceError)?;
                slot
            }
        };
        let slot_b: BlockSlot<i32, i32> = match b {
            BlockSlotDescriptor::VariantValue(variant_value) => match variant_value {
                VariantValue::Int(a) => BlockSlot::new_with_value(*a),
            },
            BlockSlotDescriptor::Block(b) => {
                let block = b.reify().map_err(|e| {
                    FromDescriptorError::Child(
                        BlockSlotPosition::Phrase {
                            phrase_idx: 0,
                            slot_idx: 1,
                        },
                        Box::new(e),
                    )
                })?;
                let mut slot = BlockSlot::new();
                slot.try_place(Box::new(block))
                    .map_err(FromDescriptorError::BlockPlaceError)?;
                slot
            }
        };
        block.slot_a = slot_a;
        block.slot_b = slot_b;
        Ok(block)
    }
}
