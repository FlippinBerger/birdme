use chrono::Utc;
use std::path::PathBuf;
use std::{fs, io};

pub struct Logger {
    log_path: PathBuf,
    buf: RingBuffer,
}

impl Logger {
    pub fn new(file_name: String, buffer_size: usize) -> Self {
        let buf = RingBuffer::new(buffer_size);

        // check for log directory at project root
        let log_path = get_or_create_log_dir().join(file_name);
        fs::write(&log_path, "").expect("should be able to create log file");

        Self { log_path, buf }
    }

    // log stores a log string to later be written to the log file
    pub fn log(&mut self, l: &str) {
        let now = Utc::now();

        let s = format!("{} {}", now.to_string(), l.to_owned());
        self.buf.add(s);
    }

    // flush writes all stored logs to the log file
    pub fn flush(&mut self) -> Result<(), io::Error> {
        for s in self.buf.iter() {
            println!("{}", s);
        }

        Ok(())
    }
}

fn get_or_create_log_dir() -> PathBuf {
    let log_path = std::env::current_dir().unwrap().join(".logs");

    match log_path.try_exists() {
        Ok(v) => {
            if !v {
                fs::create_dir(log_path.clone()).expect("directory creation should work");
            }
        }
        Err(_) => fs::create_dir(log_path.clone()).expect("dir creation should work"),
    }

    log_path
}

// RingBuffer is a simple ring buffer where the underlying vector gets filled
// up and then will wrap back around to overwrite its data once filled
struct RingBuffer {
    v: Vec<String>,
    cap: usize,
    length: usize,
    head: usize,
}

impl RingBuffer {
    fn new(cap: usize) -> Self {
        Self {
            v: Vec::with_capacity(cap),
            cap,
            head: 0,
            length: 0,
        }
    }

    // fills the entire vector and then keeps track of where to loop
    // back around to overwrite
    fn add(&mut self, s: String) {
        if self.v.len() < self.cap {
            self.v.push(s);
            self.length += 1;
        } else {
            self.v[self.head] = s;
            self.head = (self.head + 1) % self.cap;
        }
    }

    // clear deletes everything in the ring buffer and resets the ptr
    fn clear(&mut self) {
        self.v.clear();
        self.head = 0;
    }

    pub fn iter(&self) -> RingBufferIterator {
        RingBufferIterator {
            buffer: self,
            counter: 0,
            head: self.head,
            length: self.length,
        }
    }
}

struct RingBufferIterator<'a> {
    buffer: &'a RingBuffer,
    counter: usize,
    head: usize,
    length: usize,
}

impl<'a> Iterator for RingBufferIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == self.length {
            return None;
        }

        let ind = (self.head + self.counter) % self.buffer.cap;
        self.counter += 1;

        Some(self.buffer.v[ind].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let mut logger = Logger::new("blah".to_owned(), 2);

        assert_eq!(logger.buf.head, 0);
        assert_eq!(logger.buf.cap, 2);
        assert_eq!(logger.buf.v.len(), 0);
        assert_eq!(logger.buf.v.capacity(), 2);

        logger.log("hello there");
        assert_eq!(logger.buf.v.len(), 1);
        assert_eq!(logger.buf.v.capacity(), 2);
        assert_eq!(logger.buf.head, 0);

        logger.log("hello there");
        assert_eq!(logger.buf.v.len(), 2);
        assert_eq!(logger.buf.v.capacity(), 2);
        assert_eq!(logger.buf.head, 0);

        logger.log("hello there");
        assert_eq!(logger.buf.v.len(), 2);
        assert_eq!(logger.buf.v.capacity(), 2);
        assert_eq!(logger.buf.head, 1);
    }

    #[test]
    fn iter() {
        let mut logger = Logger::new("blah".to_owned(), 2);

        let logs = vec!["hello", "world"];

        for i in 0..logs.len() {
            println!("{}: adding {}", i, logs[i]);
            logger.log(logs[i]);
        }

        for (i, s) in logger.buf.iter().enumerate() {
            println!("loop {}: {}", i, s);
            assert!(s.ends_with(logs[i]));
        }
    }

    #[test]
    fn iter_empty() {
        let logger = Logger::new("blah".to_owned(), 2);

        for _ in logger.buf.iter() {
            panic!();
        }
    }

    #[test]
    fn iter_single() {
        let mut logger = Logger::new("blah".to_owned(), 2);

        let logs = vec!["hello"];

        for i in 0..logs.len() {
            println!("{}: adding {}", i, logs[i]);
            logger.log(logs[i]);
        }

        for (i, s) in logger.buf.iter().enumerate() {
            println!("loop {}: {}", i, s);
            assert!(s.ends_with(logs[i]));
        }
    }

    #[test]
    fn iter_wrapped() {
        let mut logger = Logger::new("blah".to_owned(), 2);

        let logs = vec!["hello", "world", "I'm", "Yours"];

        for i in 0..logs.len() {
            println!("{}: adding {}", i, logs[i]);
            logger.log(logs[i]);
        }

        for (i, s) in logger.buf.iter().enumerate() {
            println!("loop {}: {}", i, s);
            assert!(s.ends_with(logs[i + 2]));
        }
    }

    #[test]
    fn iter_wrapped2() {
        let mut logger = Logger::new("blah".to_owned(), 2);

        let logs = vec!["hello", "world", "I'm", "Yours", "today"];

        for i in 0..logs.len() {
            println!("{}: adding {}", i, logs[i]);
            logger.log(logs[i]);
        }

        for (i, s) in logger.buf.iter().enumerate() {
            println!("loop {}: {}", i, s);
            assert!(s.ends_with(logs[i + 3]));
        }
    }
}
