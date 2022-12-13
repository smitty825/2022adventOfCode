struct Assignment {
    first_section: u32,
    last_section: u32 
}

fn get_assignments(input: &str) -> (Assignment,Assignment) {
    // First split on the comma to get 2 assignments
    let elves: Vec<&str> = input.split(",").collect(); 

    // Now create the assignments 
    let first_elf : Vec<&str>= elves.first().unwrap().split("-").collect(); 
    let second_elf: Vec<&str> = elves.last().unwrap().split("-").collect(); 

    let first_assignment = Assignment {first_section:first_elf.first().unwrap().parse::<u32>().unwrap(), last_section:first_elf.last().unwrap().parse::<u32>().unwrap()};
    let second_assignment = Assignment {first_section:second_elf.first().unwrap().parse::<u32>().unwrap(), last_section:second_elf.last().unwrap().parse::<u32>().unwrap()};

    return (first_assignment, second_assignment);
}

fn is_complete_overlap(first_elf: Assignment, second_elf: Assignment) -> bool {
    let mut overlap_detect = false; 
    if first_elf.first_section <= second_elf.first_section && 
       first_elf.last_section >= second_elf.last_section {
        overlap_detect = true; 
       }
    else if second_elf.first_section <= first_elf.first_section &&
        second_elf.last_section >= first_elf.last_section {
            overlap_detect = true;
       }

    return overlap_detect
}

fn is_any_overlap(first_elf: Assignment, second_elf: Assignment) -> bool {
    let mut overlap_detect = false; 

    if (first_elf.first_section >= second_elf.first_section && first_elf.first_section <= second_elf.last_section) ||
       (first_elf.last_section >= second_elf.first_section && first_elf.last_section <= second_elf.last_section) {
        overlap_detect = true; 
       }
    else if is_complete_overlap(first_elf, second_elf) {
        overlap_detect = true; 
    }

    return overlap_detect; 
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_overlaps = 0;
    let lines = input.lines();
    for assignments in lines {
        // First step...get the assignments in a better form to compare
        let these_assignment = get_assignments(assignments); 

        if is_complete_overlap( these_assignment.0, these_assignment.1) {
            total_overlaps += 1; 
        }
    }
    return Some(total_overlaps);
    
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_overlaps = 0;
    let lines = input.lines();
    for assignments in lines {
        // First step...get the assignments in a better form to compare
        let these_assignment = get_assignments(assignments); 

        if is_any_overlap( these_assignment.0, these_assignment.1) {
            total_overlaps += 1; 
        }
    }
    return Some(total_overlaps);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
