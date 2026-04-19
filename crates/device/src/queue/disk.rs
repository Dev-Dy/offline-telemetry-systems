use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

use common::message::Message;

pub struct DiskQueue {
    queue: VecDeque<Message>,
    file_path: String,
}

impl DiskQueue {
    pub fn new(file_path: &str) -> std::io::Result<Self> {
        let mut queue = VecDeque::new();

        // Try loading existing queue from disk
        match File::open(file_path) {
            Ok(file) => {
                let reader = BufReader::new(file);

                for line in reader.lines() {
                    match line {
                        Ok(line) => match serde_json::from_str::<Message>(&line) {
                            Ok(msg) => queue.push_back(msg),
                            Err(e) => eprintln!("failed to parse line: {}", e),
                        },
                        Err(e) => eprintln!("failed to read line: {}", e),
                    }
                }
            }
            Err(e) => {
                // Not fatal: file may not exist yet
                eprintln!("queue file not found or unreadable: {}", e);
            }
        }

        Ok(Self {
            queue,
            file_path: file_path.to_string(),
        })
    }

    pub fn push(&mut self, msg: Message) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        let line = serde_json::to_string(&msg).map_err(std::io::Error::other)?;

        writeln!(file, "{}", line)?;

        self.queue.push_back(msg);
        Ok(())
    }

    pub fn peek(&self) -> Option<&Message> {
        self.queue.front()
    }

    pub fn pop(&mut self) -> std::io::Result<()> {
        self.queue.pop_front();

        // Rewrite file (simple but correct approach)
        let mut file = File::create(&self.file_path)?;

        for msg in &self.queue {
            let line = serde_json::to_string(msg).map_err(std::io::Error::other)?;
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
