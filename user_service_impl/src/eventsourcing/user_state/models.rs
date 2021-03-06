use eventsourcing::{eventstore::MemoryEventStore, prelude::*, Result};
use crate::models::p_user::PUser;

#[derive(Debug)]
pub struct UserState {
    pub user: PUser,
    pub generation: u64
}

impl AggregateState for UserState {
    fn generation(&self) -> u64 {
        self.generation
    }
}
