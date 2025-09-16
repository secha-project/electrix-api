use crate::data::{
    event_item::DeviceEventItemWithId,
    event_mapped_item::DeviceEventMappedItemWithId
};


#[derive(Clone, Debug)]
pub enum EventItemWithId {
    Mapped(DeviceEventMappedItemWithId),
    Unmapped(DeviceEventItemWithId),
}
