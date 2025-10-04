use super::get_winner;
use crate::models::Player;

pub fn get_best_move(board: [char; 9], computer_player: &Player) -> usize {
    let mut best_score = i32::MIN;
    let mut move_index = 0;

    let mut board = board;

    for i in 0..9 {
        if board[i] == ' ' {
            board[i] = computer_player.char();
            let score = minimax(&mut board, computer_player, false);
            board[i] = ' ';
            if score > best_score {
                best_score = score;
                move_index = i;
            }
        }
    }
    move_index
}

fn minimax(baord: &mut [char; 9], computer_player: &Player, is_computers_turn: bool) -> i32 {
    let human_player = computer_player.other();

    let winner = get_winner(baord);

    if winner == Some(computer_player.clone()) {
        return 1;
    }
    if winner == Some(human_player.clone() ) {
        return -1;
    }
    if !baord.contains(&' ') {
        return 0;
    }

    if is_computers_turn {
        let mut best_score = i32::MIN;

        for i in 0..9 {
            if baord[i] == ' ' {
                baord[i] = computer_player.char();
                let score = minimax(baord, computer_player, false);
                baord[i] = ' ';
                best_score = best_score.max(score);
            }
        }
        best_score
    } else {
        let mut best_score = i32::MAX;

        for i in 0..9 {
            if baord[i] == ' ' {
                baord[i] = human_player.char();
                let score = minimax(baord, computer_player, true);
                baord[i] = ' ';
                best_score = best_score.min(score);
            }
        }
        best_score
    }
}
