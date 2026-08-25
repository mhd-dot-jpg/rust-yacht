#[derive(Clone)]
pub struct ScoringRule {
    name: String,
    func: fn(&[u32]) -> u32,
    max_score: u32,
}

impl ScoringRule {
    pub fn new(name: String, eval: fn(&[u32]) -> u32, max_score: u32) -> ScoringRule {
        ScoringRule {
            name: name,
            func: eval,
            max_score: max_score
        }
    }

    pub fn eval(&self, dice: &[u32]) -> u32{
        return (self.func)(dice);
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub fn chance(dice: &[u32]) -> u32 {
    dice.iter().sum()
}

pub fn count(dice: &[u32], value: u32) -> u32 {
    dice.iter().filter(|&x| *x == value ).count() as u32 * value
}

pub fn full_house(dice: &[u32]) -> u32 {
    let mut counts = [0; 6];
    for &roll in dice {
        let index = roll as usize - 1;
        counts[index] += 1;
    }
    let indexed_counts : Vec<(usize, &u32)> = counts.iter().enumerate().filter(|&(_, x)| *x > 0).collect(); 
    if indexed_counts.len() == 2 {
        let (_, first_count) = indexed_counts[0];
        if *first_count == 3 || *first_count == 2 {
            return indexed_counts.iter().fold(0, |acc, (ind, &x)| acc + (x * (*ind as u32 + 1)) );
        }
    }
    0
}

fn max(x: u32, y: u32) -> u32 {
    if x > y {
        return x;
    }
    y
}

pub fn x_of_a_kind(dice: &[u32], x: u32) -> u32 {
    let mut counts = [0; 6];
    for &roll in dice {
        let index = roll as usize - 1;
        counts[index] += 1;
    }
    let indexed_counts : Vec<(usize, &u32)> = counts.iter().enumerate().filter(|&(_, x)| *x > 0).collect(); 
    let mut max_score = 0;
    for (side, count) in indexed_counts {
        if *count >= x {
            let score = (side+1) as u32 * *count; 
            max_score = max(max_score, score);
        }
    }
    max_score
}

fn run_length(dice: &[u32]) -> u32 {
    let mut next = 0;
    let mut run = 0;
    let mut max_run = 0;
    for &i in dice {
        if i != next {
            run = 0;
        }
        run += 1;
        next = i + 1;
        max_run = max(max_run, run);
    }
    max_run
}

pub fn small_straight(dice: &[u32]) -> u32 {
    if run_length(dice) == 4 {
        return 30;
    }
    0
}

pub fn large_straight(dice: &[u32]) -> u32 {
    if run_length(dice) == 5 {
        return 40;
    }
    0
}

pub fn yacht(dice: &[u32]) -> u32 {
    if dice.iter().filter(|&x| *x == dice[0]).count() == 5 {
        return 50;
    }
    0
}