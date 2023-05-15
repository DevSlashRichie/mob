use serenity::{model::prelude::MessageId, prelude::TypeMapKey};
use chashmap::CHashMap;

#[derive(Clone)]
pub struct QueryMessage {
    pub id: MessageId,
    pub query: String,

    pub current_index: usize,
    pub size: usize,
}

#[derive(Default)]
pub struct QueryHandler {
    messages: CHashMap<MessageId, QueryMessage>
}

impl TypeMapKey for QueryHandler {
    type Value = QueryHandler;
}

impl QueryHandler {

    pub fn create_query(&self, id: MessageId, query: QueryMessage) {
        self.messages.insert_new(id, query);
    }

    pub fn current(&self, id: &MessageId) -> Option<QueryMessage> {
        match self.messages.get_mut(id) {
            Some(msg) => Some(msg.clone()),
            None => None
        }
    }

    pub fn next(&self, id: &MessageId) -> Option<QueryMessage> {
        match self.messages.get_mut(id) {
            Some(mut msg) =>  {
                if msg.current_index == msg.size {
                    return None;
                }

                msg.current_index += 1;
                Some(msg.clone())
            }
            None => None
        }
    }

    pub fn previous(&self, id: &MessageId) -> Option<QueryMessage> {
        match self.messages.get_mut(id) {
            Some(mut msg) =>  {
                if msg.current_index == 0 {
                    return None;
                }

                msg.current_index -= 1;
                Some(msg.clone())
            }
            None => None
        }
    }

}


impl QueryMessage {

    pub fn has_next(&self) -> bool {
        self.current_index + 1 < self.size
    }

    pub fn has_previous(&self) -> bool {
        self.current_index > 0
    }

}
