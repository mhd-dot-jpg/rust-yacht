use crate::game;
use crate::scoring::{self, ScoringRule};
use pancurses::{initscr, endwin, noecho, start_color, Input, curs_set};
use pancurses::colorpair::ColorPair;

pub fn init_ui() -> pancurses::Window {
    pancurses::resize_term(24, 128);
    let window = initscr();
    window.keypad(true);
    noecho();
    start_color();
    pancurses::init_pair(1, pancurses::COLOR_RED, pancurses::COLOR_BLACK);
    pancurses::init_pair(2, pancurses::COLOR_BLACK, pancurses::COLOR_WHITE);
    curs_set(0);
    return window;
}

pub fn close() {
    endwin();
}

pub fn init_score_windows(parent_window: &pancurses::Window, num_rules: usize) -> Vec<pancurses::Window> {
    let line_width = 18;
    let num_lines = num_rules as i32 + 2;
    let subwin_width = 4 + line_width;
    let x1 = 2;
    let x2 = parent_window.get_max_x() - (subwin_width + 2);
    let y = 2;
    let mut windows = Vec::new();
    windows.push(
        parent_window.subwin(num_lines + 2, subwin_width, y, x1).unwrap()
    );
    windows.push(
        parent_window.subwin(num_lines + 2, subwin_width, y, x2).unwrap()
    );
    return windows;
}

pub fn init_play_area(parent_window: &pancurses::Window) -> pancurses::Window{
    parent_window.subwin(24, 78, 0, 25).unwrap()
}

fn draw_die(window: &pancurses::Window, y: i32, x: i32, value: u32){
    window.mv(y, x+8);
    window.vline('|', 4);
    window.mv(y, x);
    window.vline('|', 4);
    window.printw("┌-------┐");
    window.mv(y+4, x);
    window.printw("└-------┘");
    window.mv(y+1, x+2);
    let top_row = match value {
        1 => "     ", 
        2 => "o    ",
        3 => "o    ",
        _ => "o   o",
    };
    let middle_row = match value {
        x if x % 2 == 1 => "  o  ", 
        6 => "o   o",
        _ => "     ",
    };
    let bottom_row = match value {
        1 => "     ", 
        2 => "    o",
        3 => "    o",
        _ => "o   o",
    };
    window.printw(top_row);
    window.mv(y+2, x+2);
    window.printw(middle_row);
    window.mv(y+3, x+2);
    window.printw(bottom_row);
}

fn draw_locked_die(window: &pancurses::Window, y: i32, x: i32, value: u32){
    window.mv(y, x+4);
    window.vline('|', 2);
    window.mv(y, x);
    window.vline('|', 2);
    window.printw("┌---┐");
    window.mv(y+2, x);
    window.printw("└---┘");
    window.mv(y+1, x+1);
    let face = match value {
        1 => " · ", 
        2 => ". ˙",
        3 => ".·˙",
        4 => ": :",
        5 => ":·:",
        6 => ":::",
        _ => "",
    };
    window.printw(face);
}

pub fn draw_dice(window: &pancurses::Window, dice: &[u32], center_x: i32, start_y: i32, selection: Option<usize>){
    let die_width = 10;
    let start_x = (2*center_x - (dice.len() as i32)*(die_width))/2;
    for (ind, die) in dice.iter().enumerate() {
        let pos_x = start_x + (ind as i32 * die_width);
        match selection {
            Some(x) if x == ind => {
                window.attron(ColorPair(1));
            }
            _ => {
                window.attroff(ColorPair(1));
            }
        }
        draw_die(window, start_y, pos_x, *die);
    }
}

pub fn draw_locked_dice(window: &pancurses::Window, dice: &[u32], center_x: i32){
    let die_width = 6;
    let start_x = (2*center_x - (dice.len() as i32)*(die_width))/2;
    let start_y = 4;
    for (ind, die) in dice.iter().enumerate() {
        let pos_x = start_x + (ind as i32 * die_width);
        draw_locked_die(window, start_y, pos_x, *die);
    }
}

pub fn draw_temp_locked_dice(window: &pancurses::Window, locked_dice: &[u32], dice: &[u32], selection: &[usize], center_x: i32){
    let die_width = 6;
    let length = locked_dice.len() + selection.len();
    let start_x = (2*center_x - (length as i32)*(die_width))/2;
    let start_y = 4;
    let mut current_x = start_x;
    for die in locked_dice {
        draw_locked_die(window, start_y, current_x, *die);
        current_x += die_width;
    }
    for (ind, die) in dice.iter().enumerate() {
        if selection.contains(&ind) {
            draw_locked_die(window, start_y, current_x, *die);
            current_x += die_width;
        }
    }
}

pub fn draw_temp_dice(window: &pancurses::Window, dice: &[u32], selection: &[usize], center_x: i32, highlight: Option<usize>){
    let die_width = 10;
    let length = dice.len() - selection.len();
    let start_x = (2*center_x - (length as i32)*(die_width))/2;
    let start_y = 10;
    let mut current_x = start_x;
    for (ind, die) in dice.iter().enumerate() {
        if !selection.contains(&ind) {
            match highlight {
                Some(x) if x == ind => {
                    window.attron(ColorPair(1));
                }
                _ => {
                    window.attroff(ColorPair(1));
                }
            }
            draw_die(window, start_y, current_x, *die);
            current_x += die_width;
        }
    }
    window.attroff(ColorPair(1));
}

pub fn pick_locked_dice(window: &pancurses::Window, dice: &[u32], locked: &[u32]) -> Vec<usize> {
    let mut select: Option<usize> = None;
    let mut new_selection: Vec<usize> = Vec::new();
    let subwin = window.subwin(20, 74, window.get_beg_y() + 2, window.get_beg_x() + 2).unwrap();
    let center_x = subwin.get_max_x()/2;
    loop {
        subwin.clear();
        draw_temp_dice(&subwin, dice, &new_selection, center_x, select);
        draw_temp_locked_dice(&subwin, locked, dice, &new_selection, center_x);
        subwin.refresh();
        match subwin.getch() {
            Some(Input::KeyRight) => {
                match select {
                    Some(i) => {
                        let mut x = i;
                        loop {
                            x += 1;
                            x = x % dice.len();
                            if !new_selection.contains(&x) {
                                break;
                            }
                        }
                        select = Some(x);
                    }
                    None => {
                        select = Some(0);
                    } 
                }
            }
            Some(Input::KeyLeft) => {
                match select {
                    Some(i) => {
                        if i == 0 {
                            select = Some(dice.len() - 1);
                        }
                        else {
                            let mut x = i;
                            loop {
                                if x == 0 {
                                    x = dice.len() - 1;
                                }
                                else {
                                    x -= 1;
                                }
                                if !new_selection.contains(&x) {
                                    break;
                                }
                            } 
                            select = Some(x);
                        }
                    }
                    None => {
                        select = Some(dice.len() - 1);
                    } 
                }
            }
            Some(Input::Character('u')) => {
                new_selection.pop();
            }
            Some(Input::Character(' ')) => {
                match select {
                    Some(i) => {
                        if !new_selection.contains(&i) {
                            new_selection.push(i);
                        }
                    } 
                    _ => {}
                }
            }
            Some(Input::Character('\n')) => {
                break;
            }
            _ => {}
        }
    }
    new_selection
}

fn print_scores(window: &pancurses::Window, scores: &Vec<(usize, scoring::ScoringRule, u32)>, line_width: i32, selected_row: usize) {
    let mut line = 0;
    for (_, rule, score) in scores {
        let padding_length = line_width - (rule.name().len() + score.to_string().len()) as i32 - 1;
        let padding = (0..padding_length).map(|_| ' ').collect::<String>();
        let string = format!("{}:{}{}", rule.name(), padding, *score);
        if line == selected_row as i32 {
            window.attron(ColorPair(2));
        } 
        else {
            window.attroff(ColorPair(2));
        }
        window.mv(1 + line, 2);
        window.printw(string);
        line += 1;
    }
    window.attroff(ColorPair(2));
}

pub fn pick_score(window: &pancurses::Window, scores: &Vec<(usize, scoring::ScoringRule, u32)>) -> usize {
    // Large Straight: 0
    let line_width = 19;
    let num_lines = scores.len() as i32;
    let subwin_width = 4 + line_width;
    let x = window.get_beg_x() + 39 - subwin_width/2;
    let y = 7;
    let subwin = window.subwin(num_lines + 2, subwin_width, y, x).unwrap();
    subwin.border(
        '║',
        '║',
        '═',
        '═',
        '╔',
        '╗',
        '╚',
        '╝',
    );
    let mut selected_row = 0;
    loop {
        print_scores(&subwin, scores, line_width, selected_row);
        match subwin.getch() {
            Some(Input::KeyUp) => {
                if selected_row == 0 {
                    selected_row = scores.len() - 1;
                }
                else {
                    selected_row -= 1;
                }
            }
            Some(Input::KeyDown) => {
                selected_row += 1;
                selected_row %= scores.len();
            }
            Some(Input::Character(c)) if c == ' ' || c == '\n' => {
                break;
            }
            _ => {}
        }
    }
    return scores[selected_row].0;
}

pub fn print_player_score(window: &pancurses::Window, rules: &Vec<scoring::ScoringRule>, player: &game::PlayerStats, total: u32) {
    window.border(
        '║',
        '║',
        '═',
        '═',
        '╔',
        '╗',
        '╚',
        '╝',
    );
    window.mv(0, 1);
    window.printw(format!(" Player {} ", player.num));
    let mut line = 0;
    let line_width = 19;
    for rule in rules {
        let score = player.scores.get(&rule.name());
        let string = match score {
            Some(x) => {
                let padding_length = line_width - (rule.name().len() + x.to_string().len()) as i32 - 1;
                let padding = (0..padding_length).map(|_| ' ').collect::<String>();
                format!("{}{}{}", rule.name(), padding, x)
            }
            None => {
                rule.name()
            }
        };
        window.mv(1 + line, 2);
        window.printw(string);
        line += 1;
    }
    window.mv(line + 2, 2);
    window.printw(format!("Total{:>13}", total));
    window.refresh();
}

pub fn print_turn_marker(window: &pancurses::Window, player: &game::PlayerStats) {
    window.mv(1, 35);
    window.printw(format!("Player  {}", player.num));
}
