#[derive(Clone, Debug)]
enum Result {
    Win = 6, 
    Draw = 3, 
    Loss = 0
}

#[derive(PartialEq)]
#[derive(Clone, Copy)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}



fn convertToRPS(play: &str) -> Option<RPS> {
    use crate::RPS::*; 
    match play {
        "A"|"X" => return Some(Rock),
        "B"|"Y" => return Some(Paper),
        "C"|"Z" => return Some(Scissors),
        _ => return None
    }
}

fn isBeatenBy(hand: RPS) -> RPS {
    use crate::RPS::*; 
    match hand {
        Rock => return Paper,
        Paper => return Scissors,
        Scissors => return Rock
    }
}

fn needsToBeat(play: RPS) -> RPS {
    use crate::RPS::*; 
    match play {
        Rock => return Scissors,
        Paper => return Rock,
        Scissors => return Paper
    } 
}

fn getStrategy(strategyCode: &str) -> Option<Result> {
    use crate::Result::*;
    match strategyCode {
        "X" => Some(Loss),
        "Y" => Some(Draw),
        "Z" => Some(Win), 
        _ => None 
    }
}

fn figureOutPlay(elfPlay: RPS, strategy: Result) -> RPS {
    match strategy { 
        Result::Win => return isBeatenBy(elfPlay),
        Result::Loss => return needsToBeat(elfPlay),
        Result::Draw => return elfPlay
    }
}

fn compare_hands(elf: &str, me: &str) -> Result{
    let elfMove : RPS = convertToRPS(elf).unwrap();
    let myMove : RPS = convertToRPS(me).unwrap();

    return compare_hands_RPS(elfMove, myMove);
}

fn compare_hands_RPS(elf: RPS, me: RPS) -> Result {
    use crate::Result::*; 
    let is_beaten_by = isBeatenBy(me);
    if elf == is_beaten_by {
        return Loss; 
    } else if elf == me {
        return Draw;
    }
    else {
        return Win;
    }
}



pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut totalPoints:u32 = 0; 

    for line in lines {
        let mut split = line.split(" ");
        let mut elf : &str = split.next().unwrap();
        let mut me : &str = split.next().unwrap(); 

        let result = compare_hands(elf, me);

        let hand_points : u32 = convertToRPS(me).unwrap() as u32; 
        let result_points : u32 = result as u32; 

        totalPoints = totalPoints + hand_points + result_points; 
    }

    return Some(totalPoints);

}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut totalPoints:u32 = 0; 

    for line in lines {
        let mut split = line.split(" ");
        let elf : &str = split.next().unwrap();
        let me : &str = split.next().unwrap(); 

        let elfPlay = convertToRPS(elf).unwrap(); 
        let strategy  = getStrategy(me).unwrap(); 
        let myPlay : RPS = figureOutPlay(elfPlay, strategy);

        let result = compare_hands_RPS(elfPlay, myPlay);

        let hand_points : u32 = myPlay as u32; 
        let result_points : u32 = result as u32; 

        totalPoints = totalPoints + hand_points + result_points; 

    }

    return Some(totalPoints);

}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
