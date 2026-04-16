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
    context: Option<usize>,
}

impl Object {
    pub fn new(name: &'static str, obj_type: ObjectType, handler: ObjectCommandHandler) -> Self {
        Object {
            handler,
            name,
            obj_type,
            context: None,
        }
    }

    pub fn new_with_context(
        name: &'static str,
        obj_type: ObjectType,
        handler: ObjectCommandHandler,
        context: usize,
    ) -> Self {
        Object {
            handler,
            name,
            obj_type,
            context: Some(context),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn obj_type(&self) -> ObjectType {
        self.obj_type
    }

    pub fn context(&self) -> Option<usize> {
        self.context
    }

    pub fn handle_command(
        &self,
        command: ObjectCommandID,
        data: ObjectData,
    ) -> ObjectResult<ObjectData> {
        (self.handler)(self, command, data)
    }
}
