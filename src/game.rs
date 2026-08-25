use crate::scoring;
use std::collections::HashMap;
use rand::Rng;
use rand::rngs::ThreadRng;

pub struct PlayerStats {
    pub scores: HashMap<String, u32>,
    pub num: usize,
}

impl PlayerStats {
    pub fn new(num: usize) -> PlayerStats {
        PlayerStats { 
            scores: HashMap::new(),
            num: num
        }
    }
}

#[derive(Clone, Copy)]
pub enum TurnState {
    FirstRoll,
    ReRoll(u32),
    Score
}

pub struct GameState {
    players: Vec<PlayerStats>,
    turn_state: TurnState,
    current_player: usize,
    scoring: Vec<scoring::ScoringRule>,
    locked_dice: Vec<u32>,
    dice: Vec<u32>,
    rng: ThreadRng,
}

fn roll_dice(dice: &mut [u32], rng: &mut impl Rng)
{
    for i in dice.iter_mut() {
        *i = rng.gen_range(1..=6);
    }
}


impl GameState {
    pub fn new(player_count: usize, scoring_rules: &Vec<scoring::ScoringRule>) -> GameState {
        GameState {
            players: (0..player_count).map(|x| PlayerStats::new(x+1)).collect(),
            turn_state: TurnState::FirstRoll,
            current_player: 0,
            scoring: scoring_rules.clone(),
            locked_dice: Vec::new(),
            dice: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn turn_state(&self) -> TurnState {
        self.turn_state
    }

    pub fn players(&self) -> &Vec<PlayerStats> {
        &self.players
    }

    pub fn current_player(&self) -> &PlayerStats {
        &self.players[self.current_player]
    }

    pub fn lock(&mut self, selection: &[usize]) {
        match self.turn_state {
            TurnState::ReRoll(_) => {
                let mut keep = vec![true; self.dice.len()];
                for &ind in selection {
                    let die = self.dice[ind];
                    self.locked_dice.push(die);
                    keep[ind] = false;
                }
                let mut iter = keep.iter();
                self.dice.retain(|_| *iter.next().unwrap() );
                self.locked_dice.sort();
            },
            _ => {},
        }
    }

    pub fn reroll(&mut self) {
        match self.turn_state {
            TurnState::ReRoll(x) => {
                roll_dice(&mut self.dice, &mut self.rng);
                self.turn_state = TurnState::ReRoll(x+1);
            },
            _ => {},
        }

    }

    pub fn possible_scores(&self) -> Vec<(usize, scoring::ScoringRule, u32)> {
        let mut possible_scores = Vec::<(usize, scoring::ScoringRule, u32)>::new();
        let mut full_roll = self.locked_dice.clone();
        full_roll.append(&mut self.dice.clone());
        full_roll.sort();
        for (ind, rule) in self.scoring.iter().enumerate() {
            if !self.players[self.current_player].scores.contains_key(&rule.name()) {
                let score = rule.eval(&full_roll);
                possible_scores.push((ind, rule.clone(), score));
            }
        }
        return possible_scores;
    }

    pub fn pick_score(&mut self, score_category: usize) {
        if score_category < self.scoring.len(){
            let category = &self.scoring[score_category];
            let player = &mut self.players[self.current_player];
            if !player.scores.contains_key(&category.name()) {
                let mut full_roll = self.locked_dice.clone();
                full_roll.append(&mut self.dice.clone());
                full_roll.sort();
                let score = category.eval(&full_roll);
                player.scores.insert(category.name().clone(), score);
            }
        }
    }

    pub fn next(&mut self) {
        match self.turn_state {
            TurnState::FirstRoll => {
                self.locked_dice.clear();
                self.dice = vec![0; 5];
                roll_dice(&mut self.dice, &mut self.rng); 
                self.turn_state = TurnState::ReRoll(0);
            },
            TurnState::ReRoll(_) => {
                let select: Vec<usize> = (0..self.dice.len()).collect();
                self.lock(&select);
                self.turn_state = TurnState::Score;
            },
            TurnState::Score => {
                self.current_player = (self.current_player + 1) % self.players.len();
                self.turn_state = TurnState::FirstRoll;
            },
        }
    }

    pub fn locked_dice(&self) -> &[u32] {
        &self.locked_dice
    }

    pub fn dice(&self) -> &[u32] {
        &self.dice
    }

    pub fn game_over(&self) -> bool {
        return self.players.iter().all(|p| p.scores.len() == self.scoring.len());
    }
}
