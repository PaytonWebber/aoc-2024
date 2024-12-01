use utils::read_input_file;

struct IdPair {
    left: u32,
    right: u32,
}

impl IdPair {
    fn new(left: u32, right: u32) -> IdPair {
        IdPair { left, right }
    }

    fn distance_between(&self) -> u32 {
        if self.left > self.right {
            self.left - self.right
        } else {
            self.right - self.left
        }
    }
}

struct Id {
    id: u32,
    occurrences: u32,
}

impl Id {
    fn new(id: u32, occurrences: u32) -> Id {
        Id { id, occurrences }
    }

    fn get_similarity_score(&self) -> u32 {
        self.id * self.occurrences
    }
}

fn populate_ids(content: &str) -> Vec<Id> {
    let mut ids = Vec::new();
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    let lines: Vec<&str> = content
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect();
    for line in lines {
        let ids: Vec<&str> = line.split_whitespace().collect();
        left_list.push(ids[0].parse::<u32>().unwrap());
        right_list.push(ids[1].parse::<u32>().unwrap());
    }

    right_list.sort();
    for id in left_list {
        let occurrences = right_list.iter().filter(|&x| *x == id).count() as u32;
        ids.push(Id::new(id, occurrences));
    }
    ids
}

fn populate_id_pairs(content: &str) -> Vec<IdPair> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    let lines: Vec<&str> = content
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect();
    for line in lines {
        let ids: Vec<&str> = line.split_whitespace().collect();
        left_list.push(ids[0].parse::<u32>().unwrap());
        right_list.push(ids[1].parse::<u32>().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let mut pairs = Vec::new();
    for i in 0..left_list.len() {
        pairs.push(IdPair::new(left_list[i], right_list[i]));
    }
    pairs
}

fn calculate_similarity(ids: Vec<Id>) -> u32 {
    ids.into_iter().map(|id| id.get_similarity_score()).sum()
}

fn calculate_total_distance(pairs: Vec<IdPair>) -> u32 {
    pairs.into_iter().map(|pair| pair.distance_between()).sum()
}

fn part1(content: &str) -> u32 {
    let ids = populate_ids(content);
    calculate_similarity(ids)
}

fn part2(content: &str) -> u32 {
    let pairs = populate_id_pairs(content);
    calculate_total_distance(pairs)
}

fn main() {
    let input = read_input_file();
    let content = match input {
        Ok(content) => content,
        Err(error) => panic!("Error: {:?}", error),
    };

    let similarity = part1(&content);
    println!("Part 1: Similarity: {}", similarity);

    let total_distance = part2(&content);
    println!("Part 2: Total Distance: {}", total_distance);
}
