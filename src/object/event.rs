use crate::object::ObjectID;

pub type ObjectEventType = usize;

#[derive(Copy, Clone, Debug)]
pub enum EventFilter {
    None,
    Specific((ObjectID, ObjectEventType)),
}

#[derive(Copy, Clone, Debug)]
pub struct EventCallback {
    pub filter: EventFilter,
    pub callback: fn(object_id: ObjectID, event_type: ObjectEventType)
}

pub type EventCallbackID = usize;
