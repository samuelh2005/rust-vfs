use alloc::{boxed::Box, collections::BTreeMap, format, vec::Vec};

use crate::object::{
    Object, ObjectHandle,
    command::{ObjectCommandHandler, ObjectCommandID, ObjectData, ObjectResult, OperationError},
    types::ObjectType,
};

pub struct ObjectManager {
    objects: Vec<Box<Object>>,
    handles: BTreeMap<ObjectHandle, usize>,
    type_counters: BTreeMap<ObjectType, usize>,
    next_id: ObjectHandle,
}

impl ObjectManager {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            handles: BTreeMap::new(),
            type_counters: BTreeMap::new(),
            next_id: 1,
        }
    }

    /// Register an object with an auto-allocated canonical `<type><count>` name.
    /// The name is leaked because `Object` stores `&'static str`.
    pub fn register_object(&mut self, obj_type: ObjectType) -> &'static str {
        let idx = self.type_counters.entry(obj_type).or_insert(0);

        let name_owned = format!("{}{}", obj_type.label(), *idx);
        *idx += 1;

        let name_static: &'static str = Box::leak(name_owned.into_boxed_str());
        self.objects
            .push(Box::new(Object::new(name_static, obj_type, None)));

        name_static
    }

    pub fn set_object_handler(&mut self, name: &str, handler: ObjectCommandHandler) -> bool {
        if let Some(obj) = self.objects.iter_mut().find(|obj| obj.name() == name) {
            obj.set_handler(handler);
            true
        } else {
            false
        }
    }

    pub fn get_object(&self, id: ObjectHandle) -> Option<&Object> {
        let idx = *self.handles.get(&id)?;
        self.objects.get(idx).map(|obj| obj.as_ref())
    }

    pub fn get_object_mut(&mut self, id: ObjectHandle) -> Option<&mut Object> {
        let idx = *self.handles.get(&id)?;
        self.objects.get_mut(idx).map(|obj| obj.as_mut())
    }

    pub fn open_object(&mut self, name: &str) -> Option<ObjectHandle> {
        let idx = self.objects.iter().position(|obj| obj.name() == name)?;

        let id = self.next_id;
        self.next_id += 1;

        self.handles.insert(id, idx);
        Some(id)
    }

    pub fn enumerate_objects(&self) -> Vec<&'static str> {
        self.objects.iter().map(|obj| obj.name()).collect()
    }

    pub fn enumerate_handles(&self) -> Vec<(ObjectHandle, &'static str)> {
        self.handles
            .iter()
            .map(|(id, idx)| (*id, self.objects[*idx].name()))
            .collect()
    }

    pub fn close_object(&mut self, id: ObjectHandle) {
        self.handles.remove(&id);
    }

    /// Removes the object owned by this handle and invalidates any other handles
    /// that point at the same object.
    pub fn unregister_object(&mut self, id: ObjectHandle) {
        let Some(removed_idx) = self.handles.remove(&id) else {
            return;
        };

        // Remove all other handles to the same object.
        let other_handles: Vec<ObjectHandle> = self
            .handles
            .iter()
            .filter_map(|(handle, idx)| {
                if *idx == removed_idx {
                    Some(*handle)
                } else {
                    None
                }
            })
            .collect();

        for handle in other_handles {
            self.handles.remove(&handle);
        }

        let last_index = self.objects.len() - 1;
        self.objects.swap_remove(removed_idx);

        // If we swapped the last object into the removed slot, fix any handles
        // that pointed to the old last index.
        if removed_idx != last_index {
            for idx in self.handles.values_mut() {
                if *idx == last_index {
                    *idx = removed_idx;
                }
            }
        }
    }

    pub fn handle_command(
        &self,
        id: ObjectHandle,
        command: ObjectCommandID,
        data: ObjectData,
    ) -> ObjectResult<ObjectData> {
        let idx = *self.handles.get(&id).ok_or(OperationError::NotFound)?;
        let obj = self.objects.get(idx).ok_or(OperationError::NotFound)?;
        obj.handle_command(command, data)
    }
}
