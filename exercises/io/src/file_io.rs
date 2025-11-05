#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::{BufRead, BufReader, Write};
    use tracing::info;
    use utils::log::configuration::init_logger;

    #[test]
    fn test_write_read_file() {
        init_logger();
        let path = "test_io.txt";
        let content = "Hello, file I/O in Rust!";

        // Write to file
        {
            let mut file = File::create(path).expect("Failed to create file");
            file.write(content.as_bytes())
                .expect("Failed to write to file");
            file.flush().expect("Failed to flush file");
            // file.write_all(content.as_bytes())
            // .expect("Failed to write to file");
        }

        // Read from file
        {
            let mut file = File::open(path).expect("Failed to open file");
            let mut read_content = String::new();
            let mut buff = BufReader::new(file);
            loop {
                let mut line = String::new();
                let bytes_read = buff.read_line(&mut line).expect("Failed to read line");
                if bytes_read == 0 {
                    break; // EOF reached
                }
                read_content.push_str(&line);
            }
            // file.read_to_string(&mut read_content)
            // .expect("Failed to read from file");
            assert_eq!(content, read_content);
            info!("File content: {}", read_content);
        }

        // Clean up
        fs::remove_file(path).expect("Failed to delete test file");
    }
}
