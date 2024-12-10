use std::str::FromStr;

fn main() {
    let input_file = std::env::args().nth(1).expect("Usage: <program> <input file>");

    let input_str = std::fs::read_to_string(input_file)
        .unwrap()
        .trim()
        .to_string();
    let mut frag_disk = input_str.parse::<Disk>().unwrap();
    let mut defrag_disk = frag_disk.clone();
    frag_disk.frag();
    println!("Checksum after frag: {}", frag_disk.checksum());
    defrag_disk.defrag();
    println!("Checksum after defrag: {}", defrag_disk.checksum());
}

#[derive(Debug, Clone)]
struct Disk {
    // Use -1 for empty space, otherwise file_id
    positions: Vec<i32>,
}

#[derive(Debug)]
struct DiskFromStrError;

impl std::error::Error for DiskFromStrError {}

impl std::fmt::Display for DiskFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid disk string")
    }
}

impl FromStr for Disk {
    type Err = DiskFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut positions = Vec::new();
        let mut file_count = 0;

        for (idx, c) in s.chars().enumerate() {
            let block_len = c.to_digit(10).ok_or(DiskFromStrError)? as i32;
            if idx % 2 == 0 {
                let file_id = file_count;
                file_count += 1;
                for _ in 0..block_len {
                    positions.push(file_id);
                }
            } else {
                for _ in 0..block_len {
                    positions.push(-1);
                }
            }
        }

        Ok(Disk { positions })
    }
}

impl std::fmt::Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for &pos in self.positions.iter() {
            if pos == -1 {
                result.push('.');
            } else {
                result.push_str(&pos.to_string());
            }
        }
        write!(f, "{}", result)
    }
}

impl Disk {
    fn frag(&mut self) {
        let mut left_idx = 0;
        let mut right_idx = self.positions.len() - 1;

        while left_idx < right_idx {
            let cur_part = self.positions[right_idx];
            if cur_part == -1 {
                right_idx -= 1;
            } else {
                let left_part = self.positions[left_idx];
                if left_part == -1 {
                    self.positions.swap(left_idx, right_idx);
                    left_idx += 1;
                    right_idx -= 1;
                } else {
                    left_idx += 1;
                }
            }
        }
    }

    fn defrag(&mut self) {
        let mut right_idx = self.positions.len() - 1;
        while right_idx > 0 {
            let cur_file_id = self.positions[right_idx];
            if cur_file_id == -1 {
                right_idx -= 1;
                continue;
            }

            // Find current file's length
            let mut file_len = 1;
            while (right_idx as i32 - file_len as i32) > 0
                && self.positions[right_idx - file_len] == cur_file_id
            {
                file_len += 1;
            }

            // Find the next empty space that has enough room
            let mut left_idx = 0;
            let mut empty_len = 0;
            while left_idx < right_idx {
                if self.positions[left_idx] == -1 {
                    empty_len = 1;
                    let mut next_idx = left_idx + 1;
                    while next_idx < right_idx && self.positions[next_idx] == -1 {
                        empty_len += 1;
                        next_idx += 1;
                    }

                    if empty_len >= file_len {
                        break;
                    }
                }

                left_idx += 1;
            }

            if left_idx == right_idx || empty_len < file_len {
                right_idx -= file_len;
                continue;
            }

            // Move the file to the empty space
            for i in 0..file_len {
                self.positions[left_idx + i] = cur_file_id;
                self.positions[right_idx - i] = -1;
            }
        }
    }

    fn checksum(&self) -> i64 {
        let mut result: i64 = 0;

        for (idx, &pos) in self.positions.iter().enumerate() {
            if pos == -1 {
                continue;
            }

            result += (pos as i64) * (idx as i64);
        }

        result
    }
}
