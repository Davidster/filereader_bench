use std::io::BufRead;

const INPUTS_FOLDER: &str = "../inputs";

fn main() {
    let start_time = std::time::Instant::now();

    for file_entry in std::fs::read_dir(INPUTS_FOLDER).unwrap() {
        let file_start_time = std::time::Instant::now();

        let file_entry = file_entry.unwrap();

        let file = std::fs::File::open(file_entry.path()).unwrap();
        let mut reader = std::io::BufReader::new(file);
        let mut read_buffer = String::new();
        let mut hash = 0;
        loop {
            read_buffer.clear();

            let bytes_read = reader.read_line(&mut read_buffer).unwrap();

            if bytes_read == 0 {
                // reached EOF
                break;
            }

            for char in read_buffer.chars() {
                if char == '\n' {
                    continue;
                }

                let char_as_number = u32::from(char);
                hash += char_as_number;
            }
        }

        println!(
            "file_name={:?}, hash={hash:?}, elapsed={:?}",
            file_entry.file_name(),
            file_start_time.elapsed()
        );
    }

    println!("overall_elapsed={:?}", start_time.elapsed());
}
