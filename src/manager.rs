use alloc::{collections::BTreeMap, vec::Vec};
use log::{debug, info};

use crate::{
    driver::responses::InterruptHandler,
    object::{
        ObjectHandle, ObjectID,
        command::{CommandData, CommandError, CommandID, CommandResult, ObjectCommandHandler},
    },
};

pub struct ObjectManager {
    objects: BTreeMap<ObjectID, Option<ObjectCommandHandler>>,
    handles: BTreeMap<ObjectHandle, ObjectID>,
    interrupt_handlers: BTreeMap<ObjectID, BTreeMap<u32, InterruptHandler>>,
    next_handle: ObjectHandle,
}

impl ObjectManager {
    pub fn new() -> Self {
        Self {
            objects: BTreeMap::new(),
            handles: BTreeMap::new(),
            interrupt_handlers: BTreeMap::new(),
            next_handle: 1,
        }
    }

    pub fn register_object(
        &mut self,
        object: ObjectID,
        interrupt_handlers: BTreeMap<u32, InterruptHandler>,
    ) {
        if self.objects.contains_key(&object) {
            debug!("Object {} already registered, skipping", object);
            return;
        }

        debug!("Registering object: {}", object);
        self.objects.insert(object, None);
        self.interrupt_handlers.insert(object, interrupt_handlers);
    }

    pub fn register_object_commands(
        &mut self,
        object: ObjectID,
        command_handler: ObjectCommandHandler,
        interrupt_handlers: BTreeMap<u32, InterruptHandler>,
    ) {
        if self.objects.contains_key(&object) {
            debug!("Object {} already registered, skipping", object);
            return;
        }

        debug!("Registering object: {}", object);
        self.objects.insert(object, Some(command_handler));
        self.interrupt_handlers.insert(object, interrupt_handlers);
    }

    pub fn get_object(&self, id: ObjectHandle) -> Result<ObjectID, &'static str> {
        let object_id = self
            .handles
            .get(&id)
            .copied()
            .ok_or("Object handle not found")?;

        if self.objects.contains_key(&object_id) {
            Ok(object_id)
        } else {
            Err("Object not found")
        }
    }

    pub fn open_object(&mut self, name: &str) -> Result<ObjectHandle, &'static str> {
        let object_id = self
            .objects
            .get_key_value(name)
            .map(|(object_id, _)| *object_id)
            .ok_or("Object not found")?;

        let handle = self.next_handle;
        self.next_handle = self
            .next_handle
            .checked_add(1)
            .ok_or("Object handle overflow")?;

        debug!("Opening object: {} (handle: {})", name, handle);
        self.handles.insert(handle, object_id);
        Ok(handle)
    }

    pub fn enumerate_objects(&self) -> Vec<ObjectID> {
        self.objects.keys().copied().collect()
    }

    pub fn enumerate_handles(&self) -> Vec<(ObjectHandle, ObjectID)> {
        self.handles
            .iter()
            .filter_map(|(handle, object_id)| {
                self.objects.get(object_id).map(|_| (*handle, *object_id))
            })
            .collect()
    }

    pub fn enumerate_interrupt_handlers(&self) -> Vec<(ObjectID, BTreeMap<u32, InterruptHandler>)> {
        self.interrupt_handlers
            .iter()
            .filter_map(|(object_id, handlers)| {
                self.objects
                    .get(object_id)
                    .map(|_| (*object_id, handlers.clone()))
            })
            .collect()
    }

    pub fn close_object(&mut self, id: ObjectHandle) {
        debug!("Closing object handle: {}", id);
        self.handles.remove(&id);
    }

    pub fn unregister_object(&mut self, name: &str) {
        if self.objects.remove(name).is_some() {
            info!("Unregistering object: {}", name);
            self.interrupt_handlers.remove(name);
            self.handles.retain(|_, object_id| *object_id != name);
        }
    }

    pub fn handle_command(
        &self,
        id: ObjectHandle,
        command: CommandID,
        data: CommandData,
    ) -> CommandResult<CommandData> {
        let object_id = self
            .handles
            .get(&id)
            .copied()
            .ok_or(CommandError::NotFound)?;

        let command_handler: ObjectCommandHandler = *self
            .objects
            .get(object_id)
            .ok_or(CommandError::NotFound)?
            .as_ref()
            .ok_or(CommandError::UnsupportedOperation)?;

        command_handler(object_id, command, data)
    }
}
