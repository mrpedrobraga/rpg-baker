//! Collection of builtin blocks everyone can to make scripts.

use either::Either;

pub use super::TypedBlock;
use super::{
    Block, BlockInstanceDescriptor, BlockSlot, BlockSlotDescriptor, BlockSlotRef, ReifyError,
};
use crate::format::{BaseType, VariantValue};

/// Describes a single integer value!
pub struct Int(pub i32);

impl TypedBlock for Int {
    fn evaluate(&self) -> VariantValue {
        VariantValue::Int(self.0)
    }
}

impl Block for Int {
    fn description() -> &'static str {
        "Evaluates to the number that you input here."
    }

    fn create() -> Self {
        return Self(0);
    }

    fn from_descriptor(descriptor: &BlockInstanceDescriptor) -> Result<Self, ReifyError> {
        let mut block = Self::create();
        let input = descriptor
            .content
            .get("v")
            .ok_or(ReifyError::MissingField("v"))?;
        let slot_position = super::BlockSlotRef("v");
        match input {
            super::BlockContentDescriptor::Slot(block_slot_descriptor) => {
                match block_slot_descriptor {
                    BlockSlotDescriptor::VariantValue(variant_value) => match variant_value {
                        VariantValue::Int(a) => {
                            block.0 = *a;
                            Ok(block)
                        }
                        _ => Err(ReifyError::MismatchedType(slot_position, BaseType::Int)),
                    },
                    _ => Err(ReifyError::ShouldBeAVariant(slot_position)),
                }
            }
        }
    }
}

/// Sums two integer-typed values!
pub struct Add {
    pub slot_a: BlockSlot<i32>,
    pub slot_b: BlockSlot<i32>,
}

impl TypedBlock for Add {
    fn evaluate(&self) -> VariantValue {
        match (&self.slot_a.0, &self.slot_b.0) {
            (Either::Left(a), Either::Left(b)) => match (a.evaluate(), b.evaluate()) {
                (VariantValue::Int(a), VariantValue::Int(b)) => VariantValue::Int(a + b),
                _ => panic!("TypeError!"),
            },
            (Either::Left(a), Either::Right(b)) => match (a.evaluate(), b) {
                (VariantValue::Int(a), b) => VariantValue::Int(a + b),
                _ => panic!("TypeError!"),
            },
            (Either::Right(a), Either::Left(b)) => match (a, b.evaluate()) {
                (a, VariantValue::Int(b)) => VariantValue::Int(a + b),
                _ => panic!("TypeError!"),
            },
            (Either::Right(a), Either::Right(b)) => VariantValue::Int(a + b),
        }
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

    fn from_descriptor(descriptor: &BlockInstanceDescriptor) -> Result<Self, ReifyError> {
        let mut block = Self::create();
        let a = descriptor
            .content
            .get("a")
            .ok_or(ReifyError::MissingField("a"))?;
        let b = descriptor
            .content
            .get("b")
            .ok_or(ReifyError::MissingField("b"))?;
        let slot_a: BlockSlot<i32> = match a {
            super::BlockContentDescriptor::Slot(block_slot_descriptor) => {
                match block_slot_descriptor {
                    BlockSlotDescriptor::VariantValue(variant_value) => match variant_value {
                        VariantValue::Int(value) => BlockSlot::new_with_value(*value),
                        _ => Err(ReifyError::MismatchedType(BlockSlotRef("a"), BaseType::Int))?,
                    },
                    BlockSlotDescriptor::Block(child_block) => {
                        let block = child_block
                            .reify()
                            .map_err(|e| ReifyError::Child(BlockSlotRef("a"), Box::new(e)))?;
                        let mut slot = BlockSlot::new();
                        slot.try_place(Box::new(block))
                            .map_err(ReifyError::BlockPlaceError)?;
                        slot
                    }
                }
            }
        };
        let slot_b: BlockSlot<i32> = match b {
            super::BlockContentDescriptor::Slot(block_slot_descriptor) => {
                match block_slot_descriptor {
                    BlockSlotDescriptor::VariantValue(variant_value) => match variant_value {
                        VariantValue::Int(value) => BlockSlot::new_with_value(*value),
                        _ => Err(ReifyError::MismatchedType(BlockSlotRef("b"), BaseType::Int))?,
                    },
                    BlockSlotDescriptor::Block(child_block) => {
                        let block = child_block
                            .reify()
                            .map_err(|e| ReifyError::Child(BlockSlotRef("b"), Box::new(e)))?;
                        let mut slot = BlockSlot::new();
                        slot.try_place(Box::new(block))
                            .map_err(ReifyError::BlockPlaceError)?;
                        slot
                    }
                }
            }
        };
        block.slot_a = slot_a;
        block.slot_b = slot_b;
        Ok(block)
    }
}

/// Logs a single value to the standard output!
pub struct Log(BlockSlot<()>);

impl TypedBlock for Log {
    fn evaluate(&self) -> VariantValue {
        let inner = match &self.0.0 {
            Either::Left(a) => a.evaluate(),
            Either::Right(_) => VariantValue::Void,
        };
        println!("LOG {:?}", inner);
        inner
    }
}

impl Block for Log {
    fn description() -> &'static str {
        "Evaluates to the number that you input here."
    }

    fn create() -> Self {
        return Self(BlockSlot::new());
    }

    fn from_descriptor(descriptor: &BlockInstanceDescriptor) -> Result<Self, ReifyError> {
        let mut block = Self::create();
        let slot = descriptor
            .content
            .get("what")
            .ok_or(ReifyError::MissingField("what"))?;
        let slot: BlockSlot<()> = match slot {
            super::BlockContentDescriptor::Slot(block_slot_descriptor) => {
                match block_slot_descriptor {
                    BlockSlotDescriptor::Block(b) => {
                        let block = b
                            .reify()
                            .map_err(|e| ReifyError::Child(BlockSlotRef("what"), Box::new(e)))?;
                        let mut slot = BlockSlot::new();
                        slot.try_place(Box::new(block))
                            .map_err(ReifyError::BlockPlaceError)?;
                        slot
                    }
                    _ => BlockSlot::new_with_value(()),
                }
            }
        };
        block.0 = slot;
        Ok(block)
    }
}
