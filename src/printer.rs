pub fn print_8x8_grid(board: &Vec<char>) {
  for row_index in 0..8 {
    let row = &board[8*row_index..8*row_index + 8];
    println!("{}", row.iter().map(|cell| cell).cloned().collect::<String>());
  }
}

pub fn print_in_fen(board: &Vec<char>) {
  println!("{}", convert_to_fen(board));
}

fn convert_to_fen(board: &Vec<char>) -> String {
  let mut result: String = String::from("");
  for row_index in 0..8 {
    let row: String = board[8*row_index..8*row_index + 8].iter().map(|cell| cell).cloned().collect::<String>();
    result.push_str(&collapse_empty_cells(row)[..]);
    if row_index != 7 {
      result.push_str("/");
    }
  }
  result.push_str(" w - - 0 1");
  result
}

fn collapse_empty_cells(row: String) -> String {
  let row = row.replace("........", "8");
  let row = row.replace(".......",  "7");
  let row = row.replace("......",   "6");
  let row = row.replace(".....",    "5");
  let row = row.replace("....",     "4");
  let row = row.replace("...",      "3");
  let row = row.replace("..",       "2");
  let row = row.replace(".",        "1");
  row
}

#[cfg(test)]
mod tests {
  use super::*;

  fn empty_board() -> Vec<char> {
    (0..64).map(|_| '.').collect()
  }

  #[test]
  fn it_should_convert_empty_board_into_rows_of_8() {
    let board = empty_board();
    let result = convert_to_fen(&board);
    assert_eq!(result, "8/8/8/8/8/8/8/8 w - - 0 1");
  }

  #[test]
  fn pieces_should_be_present() {
  let mut board = empty_board();
    for cell in 0..8 {
      board[cell] = 'p';
    }
    let result = convert_to_fen(&board);
    assert_eq!(result, "pppppppp/8/8/8/8/8/8/8 w - - 0 1");
  }

  #[test]
  fn it_should_convert_adjacent_empty_cells_together() {
    let mut board = empty_board();
    for cell in 0..2 {
      board[cell] = 'p';
    }

    for cell in 4..6 {
      board[cell] = 'p';
    }

    let result = convert_to_fen(&board);
    assert_eq!(result, "pp2pp2/8/8/8/8/8/8/8 w - - 0 1");
  }
}
