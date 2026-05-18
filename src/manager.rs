use alloc::{collections::BTreeMap, vec::Vec};
use log::{debug, info};

use crate::{
    driver::responses::InterruptHandler,
    object::{
        ObjectHandle, ObjectID,
        command::{CommandData, CommandError, CommandID, CommandResult, ObjectCommandHandler}, event::{EventCallback, EventCallbackID, ObjectEventType},
    },
};

pub struct ObjectManager {
    objects: BTreeMap<ObjectID, Option<ObjectCommandHandler>>,
    handles: BTreeMap<ObjectHandle, ObjectID>,
    interrupt_handlers: BTreeMap<ObjectID, BTreeMap<u32, InterruptHandler>>,
    event_handlers: BTreeMap<ObjectID, BTreeMap<(EventCallbackID, ObjectEventType), EventCallback>>,
    next_handle: ObjectHandle,
}

impl ObjectManager {
    pub fn new() -> Self {
        Self {
            objects: BTreeMap::new(),
            handles: BTreeMap::new(),
            interrupt_handlers: BTreeMap::new(),
            event_handlers: BTreeMap::new(),
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

    pub fn get_handle_object(&self, handle: ObjectHandle) -> Result<ObjectID, &'static str> {
        let object_id = self
            .handles
            .get(&handle)
            .copied()
            .ok_or("Object handle not found")?;

        if self.objects.contains_key(&object_id) {
            Ok(object_id)
        } else {
            Err("Object not found")
        }
    }

    pub fn register_event_handler(&mut self, object_id: ObjectID, callback: EventCallback, event_type: usize) -> Result<EventCallbackID, &'static str> {
        if !self.objects.contains_key(&object_id) {
            return Err("Object not found");
        }

        let callback_id = self
            .event_handlers
            .entry(object_id)
            .or_insert_with(BTreeMap::new)
            .keys()
            .map(|(id, _)| *id)
            .max()
            .unwrap_or(0)
            .checked_add(1)
            .ok_or("Event callback ID overflow")?;

        self.event_handlers
            .entry(object_id)
            .or_insert_with(BTreeMap::new)
            .insert((callback_id, event_type), callback);
        Ok(callback_id)
    }

    pub fn unregister_event_handler(&mut self, object_id: ObjectID, callback_id: EventCallbackID) -> Result<(), &'static str> {
        if let Some(handlers) = self.event_handlers.get_mut(&object_id) {
            handlers.retain(|(id, _), _| *id != callback_id);
            Ok(())
        } else {
            Err("Object not found")
        }
    }

    pub fn get_event_handlers(&self, object_id: ObjectID, event_type: ObjectEventType) -> Vec<EventCallback> {
        self.event_handlers
            .get(&object_id)
            .map(|handlers| {
                handlers
                    .iter()
                    .filter_map(|((_, et), callback)| if *et == event_type { Some(*callback) } else { None })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn handle_command(
        &self,
        object_id: ObjectID,
        command: CommandID,
        data: CommandData,
    ) -> CommandResult {
        if !self.objects.contains_key(object_id) {
            return CommandResult::Err(CommandError::NotFound);
        }

        let object = self
            .objects
            .get(object_id)
            .expect("object existence was checked above");

        let command_handler: ObjectCommandHandler = match object {
            Some(handler) => *handler,
            None => return CommandResult::Err(CommandError::UnsupportedOperation),
        };

        command_handler(object_id, command, data)
    }
}
