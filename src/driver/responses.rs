use alloc::collections::btree_map::BTreeMap;

use crate::object::{Object, command::ObjectCommandHandler};

#[derive(Debug)]
pub enum InterruptError {
    UnhandledInterrupt,
    DeviceFault,
    SpuriousInterrupt,
}

pub type InterruptHandler = fn(
    object: &Object,
    interrupt_id: u32,
) -> Result<(), InterruptError>;


pub struct DriverResponse {
    pub command_handler: ObjectCommandHandler,
    pub interrupt_handlers: BTreeMap<u32, InterruptHandler>,
    pub context: Option<usize>,
}
