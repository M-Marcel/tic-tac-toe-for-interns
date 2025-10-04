use crate::models::Player;

pub fn get_winner(board: &[char; 9]) -> Option<Player> {
    let winning_combination: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    let x = Player::X.char();
    let o = Player::O.char();

    for combo in winning_combination {
        let [a, b, c] = combo;

        if board[a] == x && board[b] == x && board[c] == x {
            return Some(Player::X);
        }
        if board[a] == o && board[b] == o && board[c] == o {
            return Some(Player::O);
        }
    }
    None
}