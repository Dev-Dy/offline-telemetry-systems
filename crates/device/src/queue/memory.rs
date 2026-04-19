use std::collections::VecDeque;

use common::message::Message;

pub struct MemoryQueue {
    queue: VecDeque<Message>,
}

impl MemoryQueue {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
    #[allow(unused)]
    pub fn push(&mut self, msg: Message) {
        self.queue.push_back(msg);
    }
    #[allow(unused)]
    pub fn peek(&self) -> Option<&Message> {
        self.queue.front()
    }
    #[allow(unused)]
    pub fn pop(&mut self) {
        self.queue.pop_front();
    }
    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
