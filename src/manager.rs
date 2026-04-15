use alloc::{boxed::Box, collections::BTreeMap, format, vec::Vec};

use crate::{driver::responses::InterruptHandler, object::{
    Object, ObjectHandle,
    command::{ObjectCommandID, ObjectData, ObjectResult, OperationError},
    types::ObjectType,
}};

pub struct ObjectManager {
    objects: Vec<Box<Object>>,
    handles: BTreeMap<ObjectHandle, usize>,
    type_counters: BTreeMap<ObjectType, usize>,
    interrupt_handlers: BTreeMap<usize, BTreeMap<u32, InterruptHandler>>,
    next_id: ObjectHandle,
}

impl ObjectManager {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            handles: BTreeMap::new(),
            type_counters: BTreeMap::new(),
            interrupt_handlers: BTreeMap::new(),
            next_id: 1,
        }
    }

    pub fn get_next_name(&self, obj_type: ObjectType) -> &'static str {
        let idx = self.type_counters.get(&obj_type).unwrap_or(&0);
        let name_owned = format!("{}{}", obj_type.label(), idx);
        Box::leak(name_owned.into_boxed_str())
    }

    pub fn register_object(&mut self, object: Object, interrupt_handlers: BTreeMap<u32, InterruptHandler>) {
        let obj_type = object.obj_type();
        let name = object.name();

        if self.objects.iter().any(|obj| obj.name() == name) {
            panic!("Object with name '{}' already exists", name);
        }

        self.objects.push(Box::new(object));

        self.interrupt_handlers.insert(self.objects.len() - 1, interrupt_handlers);

        let counter = self.type_counters.entry(obj_type).or_insert(0);
        *counter += 1;
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

    pub fn enumerate_interrupt_handlers(&self) -> Vec<(&'static str, BTreeMap<u32, InterruptHandler>)> {
        self.interrupt_handlers
            .iter()
            .filter_map(|(idx, handlers)| {
                self.objects.get(*idx).map(|obj| (obj.name(), handlers.clone()))
            })
            .collect()
    }

    pub fn close_object(&mut self, id: ObjectHandle) {
        self.handles.remove(&id);
    }

    pub fn unregister_object(&mut self, name: &str) {
        if let Some(idx) = self.objects.iter().position(|obj| obj.name() == name) {
            self.objects.remove(idx);
            self.handles.retain(|_, &mut v| v != idx);
            self.interrupt_handlers.remove(&idx);
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
