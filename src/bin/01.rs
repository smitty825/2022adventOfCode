


fn do_part_one(input: &str) -> Option<u32> {
    let mut max : u32 = 0; 
    let mut current_elf = 0; 
    
    let lines = input.lines();
    for line in lines {
        let int_val = line.trim().parse::<u32>(); 
        if int_val.is_err() {
            // end of elf
            if current_elf > max {
                max = current_elf;
            }
            current_elf = 0;
        }
        else {
            current_elf += int_val.unwrap();
        }

    } 

    return Some(max); 
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = do_part_one(input);
    return result; 
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut all_elves : Vec<u32> = Vec::new(); 
    let mut current_elf = 0; 
    
    let lines = input.lines();
    for line in lines {
        let int_val = line.trim().parse::<u32>(); 
        if int_val.is_err() {
            // end of elf
            all_elves.push(current_elf); 
            current_elf = 0;
        }
        else {
            current_elf += int_val.unwrap();
        }
    } 

    all_elves.sort(); 
    let len = all_elves.len(); 
    let sum = all_elves[len-1] + all_elves[len-2] + all_elves[len-3];
    let optional_sum : Option<u32> = Some(sum); 

    println!("{}", sum);

    return optional_sum; 
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
