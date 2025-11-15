use std::{borrow::Cow, collections::VecDeque};

use serde::{Deserialize, Serialize};

const DEFAULT_MESSAGES_SIZE: usize = 5;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Console {
    pub max_messages: usize,
    pub messages: VecDeque<Cow<'static, str>>,
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
    pub fn push(&mut self, msg: impl Into<Cow<'static, str>>) {
        self.messages.push_back(msg.into());
        if self.messages.len() > self.max_messages {
            self.messages.pop_front();
        }
    }
}
