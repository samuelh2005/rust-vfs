use crate::object::ObjectID;

pub type ObjectEventType = usize;

pub struct ObjectEvent {
    pub event_type: ObjectEventType,
    pub object_id: ObjectID
}

pub type EventCallback = fn(event: ObjectEvent);
pub type EventCallbackID = usize;
