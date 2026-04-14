use crate::object::{command::{ObjectData, ObjectCommandID, ObjectResult, ObjectCommandHandler}, types::ObjectType};

pub mod command;
pub mod types;

pub type ObjectHandle = u64;

pub struct Object {
    handler: Option<ObjectCommandHandler>,
    name: &'static str,
    obj_type: ObjectType,
}

impl Object {
    pub fn new(name: &'static str, obj_type: ObjectType, handler: Option<ObjectCommandHandler>) -> Self {
        Object { handler, name, obj_type }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn obj_type(&self) -> ObjectType {
        self.obj_type
    }

    pub fn handle_command(
        &self,
        command: ObjectCommandID,
        data: ObjectData,
    ) -> ObjectResult<ObjectData> {
        if let Some(handler) = self.handler {
            (handler)(self, command, data)
        } else {
            Err(command::OperationError::UnsupportedOperation)
        }
    }

    pub fn set_handler(&mut self, handler: ObjectCommandHandler) {
        self.handler = Some(handler);
    }
}
