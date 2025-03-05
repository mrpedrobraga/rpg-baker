/// Defines a new block for the scripting language
#[macro_export]
macro_rules! block_define {
    (
        $name:ident {
            $($field:ident),* $(,)?
        },
        description: $description:expr,
        evaluate: $evaluate:expr
    ) => {
        #[doc = concat!("Block `", stringify!($name),"` : ", $description)]
        pub struct $name {
            $(pub $field: BlockSlot),*
        }

        impl TypedBlock for $name {
            fn evaluate(&self) -> VariantValue {
                // Use the provided evaluation function
                $evaluate(self)
            }
        }

        impl Block for $name {
            fn description() -> &'static str {
                // Use the provided description
                $description
            }

            fn create() -> Self {
                $name {
                    $($field: BlockSlot::new()),*
                }
            }

            fn from_descriptor(descriptor: &BlockInstanceDescriptor) -> Result<Self, ReifyError> {
                let mut block = Self::create();
                $(
                    let field = descriptor
                        .content
                        .get(stringify!($field))
                        .ok_or(ReifyError::MissingField(stringify!($field)))?;
                    let slot: BlockSlot = match field {
                        super::BlockContentDescriptor::Slot(block_slot_descriptor) => {
                            match block_slot_descriptor {
                                BlockSlotDescriptor::VariantValue(variant_value) => BlockSlot::new_with_value(variant_value.clone()),
                                BlockSlotDescriptor::Block(child_block) => {
                                    let block = child_block
                                        .reify()
                                        .map_err(|e| ReifyError::Child(BlockSlotRef(stringify!($field)), Box::new(e)))?;
                                    let mut slot = BlockSlot::new();
                                    slot.try_place(Box::new(block))
                                        .map_err(ReifyError::BlockPlaceError)?;
                                    slot
                                }
                            }
                        }
                    };
                    block.$field = slot;
                )*
                Ok(block)
            }
        }
    };
}
