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

  fn move_himself(&mut self, peach: &Character) {
    if self.col != peach.col {
      self.move_horizontal(peach);
    }
    if self.row != peach.row {
      self.move_vertical(peach);
    }
  }

  fn move_horizontal(&mut self, peach: &Character) {
    if self.col > peach.col {
      self.col -= 1;
      println!("LEFT");
    }
    else {
      self.col += 1;
      println!("RIGHT");
    }
  }

  fn move_vertical(&mut self, peach: &Character) {
    if self.row > peach.row {
      self.row -= 1;
      println!("UP");
    }
    else {
      self.row += 1;
      println!("DOWN");
    }
  }
}

fn show_board(board: &[&str]){
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

fn play(board: &[&str]){
  let mut mario   = discover_character(&board, 'm');
  let peach    = discover_character(&board, 'p');

  println!("\n");
  show_board(&board);

  while mario.should_move(&peach) {
    mario.move_himself(&peach);
  }
}

fn build_board(board: &str) -> Vec<&str> {
  let parts = board.split("\n").collect::<Vec<&str>>();

  parts
}

fn displayPathtoPrincess(_n: i32, raw_board: &str) {
  let board: Vec<&str> = build_board(raw_board);

  play(&board);
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
    displayPathtoPrincess(n, board);
  }
}
