use std::collections::VecDeque;

use common::message::Message;

pub struct MemoryQueue {
    queue: VecDeque<Message>,
}

impl MemoryQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn push(&mut self, msg: Message) {
        self.queue.push_back(msg);
    }

    pub fn peek(&self) -> Option<&Message> {
        self.queue.front()
    }

    pub fn pop(&mut self) {
        self.queue.pop_front();
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
