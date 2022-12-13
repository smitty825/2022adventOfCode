use std::collections::{ HashSet, hash_map::RandomState};

fn find_shared_characters(compartments:[&str;2]) -> Vec<char> {

    let first: HashSet<char, RandomState> = HashSet::from_iter(compartments[0].chars());
    let second: HashSet<char, RandomState> = HashSet::from_iter(compartments[1].chars());

    let mut result:Vec<char> = Vec::new();

    for x in first.intersection(&second) {
        result.push(*x);
    }

    return result; 

}

fn calculate_values(matches : Vec<char>) -> u32 {
    let mut ret_val: u32= 0; 
    for item in matches {
        match item {
            'a'..='z' => { ret_val = ret_val + (item as u32 - 96)},
            'A'..='Z' => { ret_val = ret_val + (item as u32 - 38)},
            _ => {ret_val = ret_val + 0}
        };
    }
    return ret_val; 
}


fn split_rucksack_into_compartments(rucksack: &str) -> [&str;2] {
    let len = rucksack.len();

    let (first, second) = rucksack.split_at(len / 2);

    return [first, second]; 
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_priority = 0;
    let lines = input.lines();
    for rucksack in lines {
        // First step...split the rucksack into compartments
        let compartments = split_rucksack_into_compartments(rucksack);

        // next find shared items 
        let shared_chars : Vec<char> = find_shared_characters(compartments);

        //Finally, add up the score of these items 
        total_priority = total_priority + calculate_values(shared_chars); 
    }


    return Some(total_priority);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_score : u32 = 0; 
    let mut lines = input.lines(); 
    while let Some(first) = lines.next() {
        let second = lines.next(); 
        let third = lines.next(); 

        let rsp = find_shared_characters([first, second.unwrap()]); 
        let first_set = String::from_iter(rsp.iter()); 

        let rsp2 = find_shared_characters([third.unwrap(), &first_set]);

        total_score = total_score + calculate_values(rsp2); 

    }

    return Some(total_score); 
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
