use alloc::boxed::Box;

use crate::object::{Object, ObjectHandle, command::{ObjectData, ObjectOperation, ObjectResult, OperationError, OperationHandler}, types::ObjectType};


pub struct ObjectManager {
    objects: alloc::vec::Vec<&'static Object>,
    handles: alloc::collections::BTreeMap<ObjectHandle, &'static Object>,
    type_counters: alloc::collections::BTreeMap<ObjectType, usize>,
    next_id: ObjectHandle,
}

impl ObjectManager {
    pub fn new() -> Self {
        ObjectManager {
            objects: alloc::vec::Vec::new(),
            handles: alloc::collections::BTreeMap::new(),
            type_counters: alloc::collections::BTreeMap::new(),
            next_id: 1,
        }
    }

    /// Register an object with an auto-allocated canonical `<type><count>` name.
    /// The caller MUST supply an `ObjectType`. The manager allocates the next
    /// numeric index for the type and returns the leaked `'static` name.
    pub fn register_object(&mut self, handler: OperationHandler, obj_type: ObjectType) -> &'static str {
        // Determine next index for this object type
        let idx = match self.type_counters.get(&obj_type) {
            Some(v) => *v,
            None => 0,
        };

        // Update counter for next allocation
        self.type_counters.insert(obj_type, idx + 1);

        // Allocate the name and leak it to `'static` for manager ownership
        let name_owned = alloc::format!("{}{}", obj_type.label(), idx);
        let name_static: &'static str = Box::leak(name_owned.into_boxed_str());

        let obj = Box::leak(Box::new(Object::new(name_static, obj_type, handler)));
        self.objects.push(obj);

        name_static
    }

    pub fn get_object(&self, id: ObjectHandle) -> Option<&'static Object> {
        self.handles.get(&id).copied()
    }

    pub fn get_object_mut(&mut self, id: ObjectHandle) -> Option<&mut &'static Object> {
        self.handles.get_mut(&id)
    }

    pub fn open_object(&mut self, name: &str) -> Option<ObjectHandle> {
        for obj in &self.objects {
            if obj.name() == name {
                let id = self.next_id;
                self.next_id += 1;
                self.handles.insert(id, obj);
                return Some(id);
            }
        }
        None
    }

    pub fn enumerate_objects(&self) -> alloc::vec::Vec<&'static str> {
        self.objects.iter().map(|obj| obj.name()).collect()
    }

    pub fn enumerate_handles(&self) -> alloc::vec::Vec<(ObjectHandle, &'static str)> {
        self.handles
            .iter()
            .map(|(id, obj)| (*id, obj.name()))
            .collect()
    }

    pub fn close_object(&mut self, id: ObjectHandle) {
        self.handles.remove(&id);
    }

    pub fn unregister_object(&mut self, id: ObjectHandle) {
        self.handles.remove(&id);
    }

    pub fn handle_operation(
        &self,
        id: ObjectHandle,
        operation: ObjectOperation,
        data: ObjectData,
    ) -> ObjectResult<ObjectData> {
        if let Some(obj) = self.handles.get(&id) {
            obj.handle_operation(operation, data)
        } else {
            Err(OperationError::NotFound)
        }
    }
}
