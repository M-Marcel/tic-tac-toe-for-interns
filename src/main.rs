use crossterm::{
    cursor, execute,
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};
use tic_tac_teo_for_interns::screen::{choose_player, end_menu, game_play};

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();

    loop {
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::All),
            Clear(ClearType::Purge)
        )?;

        let (human_player, continue_game) = choose_player()?;

        if !continue_game {
            break;
        }

        let (winning_player,board, continue_game) = game_play(&human_player)?;

        if !continue_game {
            break;
        }
        let continue_game = end_menu(&winning_player, &human_player, &board);

        if !continue_game? {
            break;
        }
        break;
    }

    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        Clear(ClearType::All),
        Clear(ClearType::Purge)
    )?;

    stdout.flush()?;

    terminal::disable_raw_mode()?;
    Ok(())
}

// Core game

// fn main() {
//     let mut board = [' '; 9];

//     let players = ['X', 'O'];

//     let mut turn = 0;

//     print_board(&board);

//     loop {
//         print!("Enter position for {}", players[turn]);
//         let index = get_index_from_input();
//         if let Err(e) = index {
//             println!("{e}");

//             continue;
//         }

//         let index = index.unwrap();
//         if let None = index {
//             break;
//         }
//         let index = index.unwrap();
//         if board[index] != ' ' {
//             print!("The cell at postion {} is already occupied", index);
//             continue;
//         }
//         board[index] = players[turn];

//         print_board(&board);

//         turn = (turn + 1) % 2;
//     }
// }

// fn print_board(board: &[char; 9]) {
//     println!(
//         "
//     + - - - + - - - + - - - +
//     |   {}   |   {}   |   {}   |
//     + - - - + - - - + - - - +
//     |   {}   |   {}   |   {}   |
//     + - - - + - - - + - - - +
//     |   {}   |   {}   |   {}   |
//     + - - - + - - - + - - - +
//     ",
//         board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]
//     );
// }
// fn get_index_from_input() -> Result<Option<usize>, String> {
//     let mut input = String::new();

//     let _ = io::stdin()
//         .read_line(&mut input)
//         .map_err(|e| e.to_string())?;
//     let input = input.trim();
//     if input == "quit" {
//         return Ok(None);
//     }
//     let index = input
//         .parse::<usize>()
//         .map_err(|_| format!("Input should be an integer"))?;

//     if index < 1 || index > 9 {
//         return Err(format!("The postion should be an integer from 1 to 9"));
//     }
//     Ok(Some(index - 1))
// }
