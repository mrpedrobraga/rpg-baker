use crate::{format::VariantValue, plugin::BlockContributionRef};
use either::Either;
use futures_signals::signal_vec::MutableVec;
use serde::{Deserialize, Serialize};
use std::{any::Any, collections::HashMap};
use strum::{Display, EnumString};
pub mod std_blocks;

/// A Recipe specifying a runtime behaviour (a script).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptRecipe {
    #[serde(flatten)]
    pub blocks: BlockScopeDescriptor,
}

impl ScriptRecipe {
    /// Returns a new, empty recipe.
    pub fn new() -> Self {
        ScriptRecipe {
            blocks: BlockScopeDescriptor {
                blocks: MutableVec::new(),
            },
        }
    }
}

/// Describes a block in a recipe while not running yet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockScopeDescriptor {
    pub blocks: MutableVec<BlockInstanceDescriptor>,
}
/// Describes a block in a recipe while not running yet.
#[derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize)]
pub struct BlockInstanceDescriptor {
    pub source: BlockSourceDescriptor,
    #[serde(flatten, serialize_with = "ordered_map")]
    pub content: HashMap<String, BlockContent>,
}

/// For use with serde's [serialize_with] attribute
fn ordered_map<S, K: Ord + Serialize, V: Serialize>(
    value: &HashMap<K, V>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let ordered: std::collections::BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

#[derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockContent {
    Slot(BlockSlotDescriptor),
}

impl BlockInstanceDescriptor {
    /// Transforms a block descriptor into a real block that can be executed and whatnot!
    pub fn reify(&self) -> Result<Box<dyn TypedBlock<Output = i32>>, ReifyError> {
        match &self.source {
            BlockSourceDescriptor::Builtin(builtin_block_ref) => match builtin_block_ref {
                BuiltinBlockRef::Exit => todo!(),
                BuiltinBlockRef::Int => Ok(Box::new(std_blocks::Int::from_descriptor(&self)?)),
                BuiltinBlockRef::Add => Ok(Box::new(std_blocks::Add::from_descriptor(&self)?)),
                BuiltinBlockRef::Log => Ok(Box::new(std_blocks::Log::from_descriptor(&self)?)),
            },
            BlockSourceDescriptor::Plugin(_) => unimplemented!(),
        }
    }
}

/// Describes a part of a block, which contains a phrase and a body.
///
/// For example, block that reads `if <cond> then { block } else { block2 }` has two parts,
/// the "if" and the "else." The phrases of the parts are the "if <cond> then" and "else",
/// and the blocks are the "{ block }" and "{block2}".
#[derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize)]
pub struct BlockPartDescriptor {
    pub phrase: Vec<BlockSlotDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<BlockInstanceDescriptor>>,
}

/// Describes what goes in a block's slot, which can be a block or a value.
#[derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum BlockSlotDescriptor {
    Block(BlockInstanceDescriptor),
    VariantValue(VariantValue),
}

/// Describes which block to be created for a block descriptor.
#[derive(Debug, PartialEq, Clone, Eq)]
pub enum BlockSourceDescriptor {
    Plugin(BlockContributionRef),
    Builtin(BuiltinBlockRef),
}

impl Serialize for BlockSourceDescriptor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            BlockSourceDescriptor::Plugin(BlockContributionRef {
                plugin_id,
                block_id,
            }) => {
                let plugin_id = plugin_id;
                serializer.serialize_str(&format!("{}:{}", plugin_id, block_id))
            }
            BlockSourceDescriptor::Builtin(builtin) => {
                let block_id = builtin.to_string();
                serializer.serialize_str(&format!("builtin:{}", block_id))
            }
        }
    }
}

impl<'de> Deserialize<'de> for BlockSourceDescriptor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BlockSourceDescriptorVisitor;

        impl<'de> serde::de::Visitor<'de> for BlockSourceDescriptorVisitor {
            type Value = BlockSourceDescriptor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(
                    "a string in the format `<plugin_id>:<block_id>` or `builtin:<block_id>`",
                )
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let parts: Vec<&str> = value.split(':').collect();
                if parts.len() != 2 {
                    return Err(E::custom("expected a single ':' separator"));
                }

                match parts[0] {
                    "builtin" => {
                        // Use the existing Deserialize implementation for BuiltinBlockRef
                        let builtin_block_ref = parts[1]
                            .parse::<BuiltinBlockRef>()
                            .map_err(|_| E::custom("unknown builtin block"))?;

                        Ok(BlockSourceDescriptor::Builtin(builtin_block_ref))
                    }
                    plugin_id => {
                        let block_id = parts[1].to_string();
                        let plugin_id = plugin_id.to_string();
                        Ok(BlockSourceDescriptor::Plugin(BlockContributionRef {
                            plugin_id,
                            block_id,
                        }))
                    }
                }
            }
        }

        deserializer.deserialize_str(BlockSourceDescriptorVisitor)
    }
}

/// Describes a builtin block.
#[derive(Debug, PartialEq, Clone, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum BuiltinBlockRef {
    /// Exits the current screen.
    Exit,
    /// Describes [`standard::Int`].
    Int,
    /// Describes [`standard::Add`].
    Add,
    /// Describes [`standard::Log`].
    Log,
}

pub trait Block {
    /// Returns information about what this block does (and what it returns).
    fn description() -> &'static str;

    /// Creates a new block with default values.
    fn create() -> Self;

    /// Creates a block from a [`BlockRecipeDescriptor`].
    fn from_descriptor(descriptor: &BlockInstanceDescriptor) -> Result<Self, ReifyError>
    where
        Self: Sized;
}

/// Describes an error when creating a [`Block`] from a [`BlockInstanceDescriptor`].
#[derive(Debug)]
pub enum ReifyError {
    ShouldBeAVariant(BlockSlotPosition),
    BlockPlaceError(BlockPlaceError),
    Child(BlockSlotPosition, Box<ReifyError>),
    MissingField(&'static str),
}

pub trait TypedBlock {
    /// The type this block evaluates to.
    type Output;

    /// Evaluates the block and produces a value.
    fn evaluate(&self) -> Self::Output;
}

/// Describes the position a slot occupies within its block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockSlotPosition {
    /// Place onto a slot in one of the block's phrases.
    Phrase { phrase_idx: usize, slot_idx: usize },
    /// Place onto a slot in one of the block's bodies.
    Body { body_idx: usize, idx: usize },
}

/// If a user failed to snap a block to another one,
/// this enum carries the motive why.
#[derive(Debug)]
pub enum BlockPlaceError {
    /// The place where the block is being placed does not exist or is not free.
    NotAvailable(Box<dyn Any>),
    /// The format of the block being placed and the format of the slot are mismatched.
    FormatMismatch(Box<dyn Any>),
}

/// A slot for a block to be placed inside of.
pub struct BlockSlot<BlockType, TDefault: Default>(
    pub Either<Box<dyn TypedBlock<Output = BlockType>>, TDefault>,
);

impl<BlockType: 'static, TDefault: Default> BlockSlot<BlockType, TDefault> {
    /// Creates a new slot filled with a default TDefault.
    pub fn new() -> Self {
        BlockSlot(Either::Right(TDefault::default()))
    }

    /// Creates a new slot with a given value.
    pub fn new_with_value(value: TDefault) -> Self {
        BlockSlot(Either::Right(value))
    }

    /// Attempts to place a block into this slot.
    /// Fails if the slot is full or if the output type of the block
    /// does not match the type of this slot.
    ///
    /// It's not possible to enforce this in Rust,
    /// but [`what`] here should be a `Box<Box<dyn Block<Output = ...>>>`.
    /// Failure to do that will simply return a [`BlockPlaceError::FormatMistach`].
    pub fn try_place(&mut self, what: Box<dyn Any>) -> Result<(), BlockPlaceError> {
        if self.0.is_left() {
            return Err(BlockPlaceError::NotAvailable(what));
        }
        let block = *what
            .downcast::<Box<dyn TypedBlock<Output = BlockType>>>()
            .map_err(BlockPlaceError::FormatMismatch)?;
        self.0 = Either::Left(block);
        Ok(())
    }

    /// Pops a block from this slot (if there is one).
    /// The slot is left with a default TDefault.
    pub fn pop(&mut self) -> Option<Box<dyn TypedBlock<Output = BlockType>>> {
        if self.0.is_left() {
            let mut tmp = Either::Right(TDefault::default());
            std::mem::swap(&mut self.0, &mut tmp);
            tmp.left()
        } else {
            None
        }
    }
}

impl<T: Clone + Default + 'static> BlockSlot<T, T> {
    /// For a slot that has a default type that's the same for when it's filled with a block,
    /// allow "just getting the value" out of this slot.
    fn just_evaluate(&self) -> T {
        match self.0.as_ref() {
            Either::Left(block_a) => block_a.evaluate(),
            Either::Right(a) => a.clone(),
        }
    }
}
