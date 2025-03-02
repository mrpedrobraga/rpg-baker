use std::any::Any;

use either::Either;
use futures_signals::signal::Mutable;
use serde::{Deserialize, Serialize};

/// A Recipe specifying a runtime behaviour (a script).
#[derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize)]
pub struct ScriptRecipe {}

impl ScriptRecipe {
    /// Returns a new, empty recipe.
    pub fn new() -> Self {
        ScriptRecipe {}
    }
}

pub trait Block {
    /// The type this block evaluates to.
    type Output;

    /// Evaluates the block and produces a value.
    fn evaluate(&self) -> Self::Output;
}

/// When a user puts a block onto another block,
/// where do they do it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockPlacePosition {
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

pub trait BlockExt: Block {
    /// Creates a new block into a recipe view, with default values.
    fn create() -> Self;
}

/// A slot for a block to be placed inside of.
pub struct BlockSlot<BlockType, TDefault: Default>(
    pub Mutable<Either<Mutable<Box<dyn Block<Output = BlockType>>>, TDefault>>,
);

impl<BlockType: 'static, TDefault: Default> BlockSlot<BlockType, TDefault> {
    /// Creates a new slot filled with a default TDefault.
    pub fn new() -> Self {
        BlockSlot(Mutable::new(Either::Right(TDefault::default())))
    }

    /// Attempts to place a block into this slot.
    /// Fails if the slot is full or if the output type of the block
    /// does not match the type of this slot.
    ///
    /// It's not possible to enforce this in Rust,
    /// but [`what`] here should be a `Box<Box<dyn Block<Output = ...>>>`.
    /// Failure to do that will simply return a [`BlockPlaceError::FormatMistach`].
    pub fn try_place(&mut self, what: Box<dyn Any>) -> Result<(), BlockPlaceError> {
        let mut lock = self.0.lock_mut();
        if (*lock).is_left() {
            return Err(BlockPlaceError::NotAvailable(what));
        }
        let block = *what
            .downcast::<Mutable<Box<dyn Block<Output = BlockType>>>>()
            .map_err(BlockPlaceError::FormatMismatch)?;
        *lock = Either::Left(block);
        Ok(())
    }

    /// Pops a block from this slot (if there is one).
    /// The slot is left with a default TDefault.
    pub fn pop(&mut self) -> Option<Mutable<Box<dyn Block<Output = BlockType>>>> {
        let lock = self.0.lock_ref();
        if lock.is_left() {
            drop(lock);
            self.0.replace(Either::Right(TDefault::default())).left()
        } else {
            None
        }
    }
}

impl<T: Clone + Default + 'static> BlockSlot<T, T> {
    /// For a slot that has a default type that's the same for when it's filled with a block,
    /// allow "just getting the value" out of this slot.
    fn just_evaluate(&self) -> T {
        let lock = self.0.lock_ref();
        match lock.as_ref() {
            Either::Left(block_a) => {
                let lock = block_a.lock_ref();
                (*lock).evaluate()
            }
            Either::Right(a) => a.clone(),
        }
    }
}

pub mod standard {
    pub use super::Block;
    use super::{BlockExt, BlockSlot};

    pub struct Int(pub i32);

    impl Block for Int {
        type Output = i32;

        fn evaluate(&self) -> Self::Output {
            self.0
        }
    }

    impl BlockExt for Int {
        fn create() -> Self {
            return Self(0);
        }
    }

    pub struct And {
        pub slot_a: BlockSlot<i32, i32>,
        pub slot_b: BlockSlot<i32, i32>,
    }

    impl Block for And {
        type Output = i32;

        fn evaluate(&self) -> Self::Output {
            let a = self.slot_a.just_evaluate();
            let b = self.slot_b.just_evaluate();
            a + b
        }
    }

    impl BlockExt for And {
        fn create() -> Self {
            return And {
                slot_a: BlockSlot::new(),
                slot_b: BlockSlot::new(),
            };
        }
    }
}
