fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: {} <input file>", args[0]);
        std::process::exit(1);
    }

    let input = std::fs::read_to_string(&args[1]).unwrap();
    let reports = parse_input(&input);

    let safe_reports = count_safe_reports(&reports);

    println!("Safe reports: {}", safe_reports);
    
    let safe_reports_with_safety_dampner = count_safe_reports_with_safety_dampner(&reports);

    println!("Safe reports with safety dampner: {}", safe_reports_with_safety_dampner);
}

fn parse_input(input: &str) -> Vec<Report> {
    let mut reports = Vec::new();

    for line in input.lines() {
        let mut levels = Vec::new();
        for level in line.split_whitespace() {
            levels.push(level.parse::<i32>().unwrap());
        }
        reports.push(Report { levels });
    }

    reports
}

#[derive(Debug)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn analyze(&self) -> bool {
        let windows = self.levels.windows(2);
        let is_increasing = self.levels[1] > self.levels[0];

        for window in windows {
            let diff = (window[1] - window[0]).abs();
            if diff < 1 || diff > 3 {
                return false;
            }
            if is_increasing && window[1] <= window[0] {
                return false;
            } else if !is_increasing && window[1] >= window[0] {
                return false;
            }
        }

        true
    }

    // like analyze, but one bad level can be ignored 
    fn analyze_with_safety_dampner(&self) -> bool {
        if self.analyze() {
            return true;
        }

        // try removing one level, inputs are small so brute force is fine
        for i in 0..self.levels.len() {
            let mut levels = self.levels.clone();
            levels.remove(i);
            let report = Report { levels };
            if report.analyze() {
                return true;
            }
        }

        false
    }
}

fn count_safe_reports(reports: &Vec<Report>) -> i32 {
    let mut count = 0;

    for report in reports {
        if report.analyze() {
            count += 1;
        }
    }

    count
}

fn count_safe_reports_with_safety_dampner(reports: &Vec<Report>) -> i32 {
    let mut count = 0;

    for report in reports {
        if report.analyze_with_safety_dampner() {
            count += 1;
        }
    }

    count
}

