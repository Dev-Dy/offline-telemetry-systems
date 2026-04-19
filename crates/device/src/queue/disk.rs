use std::collections::VecDeque;
use std::fs::{File, OpenOptions, rename};
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
                            Err(e) => {
                                tracing::error!(error = %e, "skipping corrupted queue entry: {}", line)
                            }
                        },
                        Err(e) => tracing::error!(error = %e, "failed reading queue file: {}", e),
                    }
                }
            }
            Err(e) => {
                tracing::error!(error = %e, "failed reading queue file");
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
        file.sync_all()?;

        self.queue.push_back(msg);
        Ok(())
    }

    pub fn peek(&self) -> Option<&Message> {
        self.queue.front()
    }

    pub fn pop(&mut self) -> std::io::Result<()> {
        self.queue.pop_front();

        let temp_path = format!("{}.tmp", self.file_path);

        {
            let mut file = File::create(&temp_path)?;
            for msg in &self.queue {
                let line = serde_json::to_string(msg).map_err(std::io::Error::other)?;
                writeln!(file, "{}", line)?;
            }
            file.sync_all()?;
        }
        rename(&temp_path, &self.file_path)?;
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
