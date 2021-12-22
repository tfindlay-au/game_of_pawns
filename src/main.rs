use std::env;
mod printer;
mod board;
mod pieces;

fn main() {
  let pieces = pieces::generate_set();
  let board = board::place_all(pieces);

  let args: Vec<String> = env::args().collect();
  match &args.get(1).map(|v| &v[..]) {
    Some("--grid") => printer::print_8x8_grid(&board),
    _ => printer::print_in_fen(&board),
  }
}