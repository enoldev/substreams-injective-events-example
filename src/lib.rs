mod pb;
use crate::pb::sf::substreams::cosmos::v1::EventList;
use substreams::errors::Error;
use crate::pb::cosmos::v1::MyEvent;
use crate::pb::cosmos::v1::MyEventList;

#[substreams::handlers::map]
pub fn map_events(event_list: EventList) -> Result<MyEventList, Error> {
    substreams::log::info!(event_list.events.len().to_string());

    let mut my_event_list: Vec<MyEvent> = Vec::new();

    for event_wrapper in event_list.events {
        if let Some(event) = event_wrapper.event {
            my_event_list.push(MyEvent { r#type: event.r#type }) 
        }
    }

    Ok(MyEventList { events: my_event_list })
}
