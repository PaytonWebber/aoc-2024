use utils::read_input_file;

struct Report {
    levels: Vec<u32>,
}

impl Report {
    fn new(levels: Vec<u32>) -> Report {
        Report { levels }
    }

    fn is_safe(&self) -> bool {
        let mut increasing = true;
        let mut decreasing = true;

        for i in 0..self.levels.len() - 1 {
            let diff = abs(self.levels[i] as i32 - self.levels[i + 1] as i32);
            if !(1..=3).contains(&diff) {
                return false;
            }
            if self.levels[i] >= self.levels[i + 1] {
                increasing = false;
            }
            if self.levels[i] <= self.levels[i + 1] {
                decreasing = false;
            }
            if !increasing && !decreasing {
                return false;
            }
        }
        true
    }

    fn is_safe_with_tolerance(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for i in 0..self.levels.len() {
            let mut levels = self.levels.clone();
            levels.remove(i);
            let temp_report = Report::new(levels);
            if temp_report.is_safe() {
                return true;
            }
        }
        false
    }
}

fn abs(x: i32) -> u32 {
    if x < 0 {
        -x as u32
    } else {
        x as u32
    }
}

fn populate_reports(content: &str) -> Vec<Report> {
    let mut reports = Vec::new();
    let lines: Vec<&str> = content
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect();
    for line in lines {
        let levels: Vec<u32> = line
            .split_whitespace()
            .map(|level| level.parse::<u32>().unwrap())
            .collect();
        reports.push(Report::new(levels));
    }
    reports
}

fn part1(input: &str) -> u32 {
    let reports = populate_reports(input);
    reports
        .into_iter()
        .filter(|report| report.is_safe())
        .count() as u32
}

fn part2(input: &str) -> u32 {
    let reports = populate_reports(input);
    reports
        .into_iter()
        .filter(|report| report.is_safe_with_tolerance())
        .count() as u32
}

fn main() {
    let input = read_input_file();
    let contents = match input {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            return;
        }
    };

    let result = part1(&contents);
    println!("Part 1: {}", result);
    let result = part2(&contents);
    println!("Part 2: {}", result);
}
