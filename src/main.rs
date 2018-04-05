#![allow(non_snake_case)]

#[derive(Debug)]
struct Character {
  row: i32,
  col: i32,
}

impl Character {
  fn should_move(&mut self, peach: &Character) -> bool {
    self.row != peach.row || self.col != peach.col
  }

  fn move_himself(&mut self, peach: &Character) -> String {
    let mut response = String::new();

    if self.col != peach.col {
      response.push_str(&self.move_horizontal(peach));
    }

    if self.row != peach.row {
      response.push_str(&self.move_vertical(peach));
    }

    response
  }

  fn move_horizontal(&mut self, peach: &Character) -> String {
    let mut response = String::new();

    if self.col > peach.col {
      self.col -= 1;
      response.push_str("LEFT\n");
    }
    else {
      self.col += 1;
      response.push_str("RIGHT\n");
    }

    response
  }

  fn move_vertical(&mut self, peach: &Character) -> String {
    let mut response = String::new();

    if self.row > peach.row {
      self.row -= 1;
      response.push_str("UP\n");
    }
    else {
      self.row += 1;
      response.push_str("DOWN\n");
    }

    response
  }
}

fn show_board(board: &[&str]){
  println!("\n");
  for x in board { println!("{}", x) }
}

fn discover_character(board: &[&str], character: char) -> Character {
  let mut row: i32 = -1;
  let mut col: i32 = -1;

  for (i, item) in board.iter().enumerate() {
    match item.chars().position(|c| c == character) {
      Some(x) => {
        col = x as i32;
        row = i as i32;

        break;
      },
      None  => {}
    }
  }

  Character {
    row: row,
    col: col
  }
}

fn play(board: &[&str]) -> String {
  let mut mario   = discover_character(&board, 'm');
  let peach       = discover_character(&board, 'p');

  //println!("\n");
  //show_board(&board);
  let mut response  = String::new();

  while mario.should_move(&peach) {
    response.push_str(&mario.move_himself(&peach));
  }

  let len = response.len();
  response.truncate(len - 1);

  response
}

fn build_board(board: &str) -> Vec<&str> {
  let parts = board.split("\n").collect::<Vec<&str>>();

  parts
}

fn displayPathtoPrincess(_n: i32, raw_board: &str) -> String {
  let board: Vec<&str> = build_board(raw_board);

  play(&board)
}

fn main() {
  let n: i32  = 3;

  let boards = [
    "---\nm-p\n---",
    "---\np-m\n---",
    "m--\n---\np--",
    "--p\n---\n--m",
    "---\n-m-\np--",
    "p--\n-m-\n---",
    "p--\n---\n--m",
    "--m\n---\np--",
  ];

  for board in &boards {
    show_board(&build_board(board));

    println!("{}", displayPathtoPrincess(n, board));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn right_right() {
    assert_eq!("RIGHT\nRIGHT", displayPathtoPrincess(1, "---\nm-p\n---"));
  }

  #[test]
  fn left_left() {
    assert_eq!("LEFT\nLEFT", displayPathtoPrincess(1, "---\np-m\n---"));
  }

  #[test]
  fn down_down() {
    assert_eq!("DOWN\nDOWN", displayPathtoPrincess(1, "m--\n---\np--"));
  }

  #[test]
  fn up_up() {
    assert_eq!("UP\nUP", displayPathtoPrincess(1, "--p\n---\n--m"));
  }

  #[test]
  fn left_down() {
    assert_eq!("LEFT\nDOWN", displayPathtoPrincess(1, "---\n-m-\np--"));
  }

  #[test]
  fn left_up() {
    assert_eq!("LEFT\nUP", displayPathtoPrincess(1, "p--\n-m-\n---"));
  }

  #[test]
  fn left_up_left_up() {
    assert_eq!("LEFT\nUP\nLEFT\nUP", displayPathtoPrincess(1, "p--\n---\n--m"));
  }

  #[test]
  fn left_down_left_down() {
    assert_eq!("LEFT\nDOWN\nLEFT\nDOWN", displayPathtoPrincess(1, "--m\n---\np--"));
  }
}
