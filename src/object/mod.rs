use crate::object::{command::{ObjectData, ObjectOperation, ObjectResult, OperationHandler}, types::ObjectType};

pub mod command;
pub mod types;

pub type ObjectHandle = u64;

pub struct Object {
    handler: OperationHandler,
    name: &'static str,
    obj_type: ObjectType,
}

impl Object {
    pub fn new(name: &'static str, obj_type: ObjectType, handler: OperationHandler) -> Self {
        Object { handler, name, obj_type }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn obj_type(&self) -> ObjectType {
        self.obj_type
    }

    pub fn handle_operation(
        &self,
        operation: ObjectOperation,
        data: ObjectData,
    ) -> ObjectResult<ObjectData> {
        (self.handler)(self, operation, data)
    }
}
