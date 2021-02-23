use crate::utils;
use hc_utils::WrappedEntryHash;
use hdk3::prelude::*;

#[hdk_entry(id = "calendar_event", visibility = "public")]
pub struct CalendarEvent {
    pub created_by: AgentPubKey,
    pub title: String,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub invitees: Vec<AgentPubKey>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreateCalendarEventInput {
    pub title: String,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub location: Option<EventLocation>,
    pub invitees: Vec<AgentPubKey>,
}
pub fn create_calendar_event(
    calendar_event_input: CreateCalendarEventInput,
) -> ExternResult<WrappedEntryHash> {
    let agent_info = agent_info()?;

    let calendar_event = CalendarEvent {
        created_by: agent_info.agent_latest_pubkey,
        title: calendar_event_input.title,
        start_time: calendar_event_input.start_time,
        end_time: calendar_event_input.end_time,
        invitees: calendar_event_input.invitees,
    };

    create_entry(&calendar_event)?;

    let calendar_event_hash = hash_entry(&calendar_event)?;

    let path = Path::from("calendar_events");

    path.ensure()?;

    create_link(path.hash()?, calendar_event_hash.clone(), ())?;

    Ok(WrappedEntryHash(calendar_event_hash))
}

pub fn get_all_calendar_events() -> ExternResult<Vec<(WrappedEntryHash, CalendarEvent)>> {
    let path = Path::from("calendar_events");

    let links = get_links(path.hash()?, None)?;

    links
        .into_inner()
        .iter()
        .map(|link| {
            let event: CalendarEvent = utils::try_get_and_convert(link.target.clone())?;
            Ok((WrappedEntryHash(link.target.clone()), event))
        })
        .collect()
}
