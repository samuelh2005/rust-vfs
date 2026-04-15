use crate::object::{
    command::{ObjectCommandHandler, ObjectCommandID, ObjectData, ObjectResult},
    types::ObjectType,
};

pub mod command;
pub mod types;

pub type ObjectHandle = u64;

pub struct Object {
    handler: ObjectCommandHandler,
    name: &'static str,
    obj_type: ObjectType,
}

impl Object {
    pub fn new(
        name: &'static str,
        obj_type: ObjectType,
        handler: ObjectCommandHandler,
    ) -> Self {
        Object {
            handler,
            name,
            obj_type,
        }
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
        (self.handler)(self, command, data)
    }
}
