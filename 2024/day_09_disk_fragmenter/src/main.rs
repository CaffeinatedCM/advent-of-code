use std::str::FromStr;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: {} <input file>", args[0]);
        std::process::exit(1);
    }

    let input_str = std::fs::read_to_string(&args[1]).unwrap().trim().to_string();
    let mut disk = input_str.parse::<Disk>().unwrap();
    disk.frag();
    println!("Checksum after frag: {}", disk.checksum());
}

#[derive(Debug)]
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
