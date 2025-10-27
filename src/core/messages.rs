use std::collections::VecDeque;

const DEFAULT_MESSAGES_SIZE: usize = 5;

#[derive(Debug, Clone)]
pub struct Console {
    pub max_messages: usize,
    pub messages: VecDeque<String>,
}

impl Default for Console {
    fn default() -> Self {
        Self {
            max_messages: DEFAULT_MESSAGES_SIZE,
            messages: VecDeque::with_capacity(DEFAULT_MESSAGES_SIZE),
        }
    }
}

impl Console {
    pub fn push(&mut self, msg: impl Into<String>) {
        self.messages.push_back(msg.into());
        if self.messages.len() > self.max_messages {
            self.messages.pop_front();
        }
    }
}
