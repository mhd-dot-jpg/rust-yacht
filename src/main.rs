#![windows_subsystem = "windows"]
mod scoring;
mod game;
mod ui;

use game::*;
fn main() {

    let scores: Vec<scoring::ScoringRule> = vec![ 
    scoring::ScoringRule::new(
        "Ones".to_string(),
        |dice| scoring::count(dice, 1),
        5,
    ),
    scoring::ScoringRule::new(
        "Twos".to_string(),
        |dice| scoring::count(dice, 2),
        10,
    ),
    scoring::ScoringRule::new(
        "Threes".to_string(),
        |dice| scoring::count(dice, 3),
        15,
    ),
    scoring::ScoringRule::new(
        "Fours".to_string(),
        |dice| scoring::count(dice, 4),
        20,
    ),
    scoring::ScoringRule::new(
        "Fives".to_string(),
        |dice| scoring::count(dice, 5),
        25,
    ),
    scoring::ScoringRule::new(
        "Sixes".to_string(),
        |dice| scoring::count(dice, 6),
        30,
    ),

    scoring::ScoringRule::new(
        "Chance".to_string(),
        scoring::chance,
        30,
    ),
    scoring::ScoringRule::new(
        "3-of-a-Kind".to_string(),
        |dice| scoring::x_of_a_kind(dice, 3),
        30,
    ),
    scoring::ScoringRule::new(
        "4-of-a-Kind".to_string(),
        |dice| scoring::x_of_a_kind(dice, 4),
        30,
    ),
    scoring::ScoringRule::new(
        "Full House".to_string(),
        scoring::full_house,
        25,
    ),
    scoring::ScoringRule::new(
        "Small Straight".to_string(),
        scoring::small_straight,
        30,
    ),
    scoring::ScoringRule::new(
        "Large Straight".to_string(),
        scoring::large_straight,
        40,
    ),
    scoring::ScoringRule::new(
        "Yacht".to_string(),
        scoring::yacht,
        50,
    ),
    ];

    let mut game_state = GameState::new(2, &scores);
    let window = ui::init_ui();
    let num_rules = scores.len();
    let score_windows = ui::init_score_windows(&window, num_rules);
    let play_area = ui::init_play_area(&window);

    loop {
        window.clear();
        play_area.border(
            '║',
            '║',
            '═',
            '═',
            '╔',
            '╗',
            '╚',
            '╝',
        );
        ui::print_turn_marker(&play_area, game_state.current_player());
        play_area.refresh();
        for (player, score_sheet) in game_state.players().iter().zip(score_windows.iter()) {
            let total = player.scores.values().fold(0, |acc, &x| acc + x);
            ui::print_player_score(score_sheet, &scores, player, total);
        }
        match game_state.turn_state() {
            TurnState::FirstRoll => {
                game_state.next();
            },
            TurnState::ReRoll(x) => {
                let locked_dice = game_state.locked_dice();
                let dice = game_state.dice();
                if x >= 2 || dice.len() == 0 {
                    game_state.next();
                }
                else {
                    let selection = ui::pick_locked_dice(&play_area, &dice, &locked_dice);
                    game_state.lock(&selection);
                    game_state.reroll();
                }
            },
            TurnState::Score => {
                let scores = &game_state.possible_scores();
                play_area.clear();
                play_area.border(
                    '║',
                    '║',
                    '═',
                    '═',
                    '╔',
                    '╗',
                    '╚',
                    '╝',
                );
                ui::draw_dice(&play_area, game_state.locked_dice(), 40, 2, None);
                ui::print_turn_marker(&play_area, game_state.current_player());
                window.refresh();
                let selected_score = ui::pick_score(&play_area, scores);
                game_state.pick_score(selected_score);
                game_state.next();
            },
        }
        if game_state.game_over(){
            play_area.clear();
            play_area.border(
                '║',
                '║',
                '═',
                '═',
                '╔',
                '╗',
                '╚',
                '╝',
            );
            play_area.refresh();
            let player_1_total = game_state.players()[0].scores.values().fold(0, |acc, &x| acc + x);
            let player_2_total = game_state.players()[1].scores.values().fold(0, |acc, &x| acc + x);
            let winner = match player_1_total >= player_2_total {
                true => 1,
                false => 2
            };
            let winner_box = window.subwin(5, 20, window.get_max_y()/2 - 3, window.get_max_x()/2 - 10).unwrap();
            winner_box.border(
                '║',
                '║',
                '═',
                '═',
                '╔',
                '╗',
                '╚',
                '╝',
            );
            winner_box.mv(0, 6);
            winner_box.printw(" Winner ".to_string());
            winner_box.mv(2, 3);
            winner_box.printw(format!("Player {} wins!", winner));
            winner_box.refresh();
            winner_box.getch();
            break;
        }
    }
    ui::close();
}
