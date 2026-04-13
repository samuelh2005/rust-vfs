use crate::object::{
    Object, ObjectData, ObjectId, ObjectOperation, ObjectResult, OperationError, OperationHandler,
};

pub struct ObjectManager {
    objects: alloc::collections::BTreeMap<ObjectId, Object>,
    next_id: ObjectId,
}

impl ObjectManager {
    pub fn new() -> Self {
        ObjectManager {
            objects: alloc::collections::BTreeMap::new(),
            next_id: 1,
        }
    }

    pub fn register_object(&mut self, handler: OperationHandler, name: &'static str) -> ObjectId {
        let id = self.next_id;
        self.next_id += 1;
        self.objects.insert(id, Object::new(id, name, handler));
        id
    }

    pub fn get_object(&self, id: ObjectId) -> Option<&Object> {
        self.objects.get(&id)
    }

    pub fn get_object_mut(&mut self, id: ObjectId) -> Option<&mut Object> {
        self.objects.get_mut(&id)
    }

    pub fn resolve_object(&self, name: &str) -> Option<ObjectId> {
        self.objects
            .values()
            .find(|obj| obj.name() == name)
            .map(Object::id)
    }

    pub fn unregister_object(&mut self, id: ObjectId) {
        self.objects.remove(&id);
    }

    pub fn handle_operation(
        &self,
        id: ObjectId,
        operation: ObjectOperation,
        data: ObjectData,
    ) -> ObjectResult<ObjectData> {
        if let Some(obj) = self.objects.get(&id) {
            obj.handle_operation(operation, data)
        } else {
            Err(OperationError::NotFound)
        }
    }
}
