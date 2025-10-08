use std::collections::VecDeque;

const DEFAULT_MESSAGES_SIZE: usize = 5;

pub struct Messages {
    pub max_messages: usize,
    messages: VecDeque<String>,
}

impl Default for Messages {
    fn default() -> Self {
        Messages {
            max_messages: DEFAULT_MESSAGES_SIZE,
            messages: VecDeque::with_capacity(DEFAULT_MESSAGES_SIZE),
        }
    }
}

impl Messages {
    pub fn push(&mut self, msg: String) {
        self.messages.push_front(msg);
        if self.messages.len() > self.max_messages {
            self.messages.pop_back();
        }
    }
}
