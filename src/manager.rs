use alloc::vec::Vec;

use crate::object::{Object, ObjectId, ObjectOperation, ObjectResult, ObjectType, OperationError, OperationHandler};

pub struct ObjectManager {
    objects: alloc::collections::BTreeMap<ObjectId, Object>,
    next_id: ObjectId,
}

impl ObjectManager {
    pub fn new() -> Self {
        ObjectManager { objects: alloc::collections::BTreeMap::new(), next_id: 1 }
    }

    pub fn register_object(&mut self, handler: OperationHandler, object_type: ObjectType) -> ObjectId {
        let id = self.next_id;
        self.next_id += 1;
        self.objects.insert(id, Object::new(id, object_type, handler));
        id
    }

    pub fn get_object(&self, id: ObjectId) -> Option<&Object> {
        self.objects.get(&id)
    }

    pub fn get_object_mut(&mut self, id: ObjectId) -> Option<&mut Object> {
        self.objects.get_mut(&id)
    }

    pub fn get_objects_for_type(&self, object_type: ObjectType) -> Vec<ObjectId> {
        self.objects.iter()
            .filter(|(_, obj)| obj.object_type() == object_type)
            .map(|(&id, _)| id)
            .collect()
    }

    pub fn unregister_object(&mut self, id: ObjectId) {
        self.objects.remove(&id);
    }

    pub fn handle_operation(&mut self, id: ObjectId, operation: ObjectOperation, data: Option<*const [u8]>) -> ObjectResult<Option<*const [u8]>> {
        if let Some(obj) = self.objects.get(&id) {
            obj.handle_operation(operation, data)
        } else {
            Err(OperationError::NotFound)
        }
    }
}