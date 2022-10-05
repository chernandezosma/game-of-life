use std::io::{stdout, Stdout, Write};
use crossterm::{execute, ExecutableCommand, QueueableCommand, style::{
    self,
    Stylize,
    Color,
    Print
}, cursor, terminal, Result, queue};

const EMPTY_CHAR:char = ' ';
const CELL_CHAR:char = '♦';
const BORDER_CHAR:char = '·';

const DEAD:bool = false;
const ALIVE:bool = !DEAD;

const MAX_ROWS:usize = 25;
const MAX_COLS:usize = 80;
const LOGOS_HEIGHT:usize = 6;

const ROW_POSITIONS:[i8;2] = [-1, 1];
const COL_POSITIONS:[i8;2] = [-1, 1];

const UPPER_LOGO:[&str;LOGOS_HEIGHT] = [
    " ██████╗ ██████╗ ███╗   ██╗██╗    ██╗ █████╗ ██╗   ██╗███████╗",
    "██╔════╝██╔═══██╗████╗  ██║██║    ██║██╔══██╗╚██╗ ██╔╝██╔════╝",
    "██║     ██║   ██║██╔██╗ ██║██║ █╗ ██║███████║ ╚████╔╝ ███████╗",
    "██║     ██║   ██║██║╚██╗██║██║███╗██║██╔══██║  ╚██╔╝  ╚════██║",
    "╚██████╗╚██████╔╝██║ ╚████║╚███╔███╔╝██║  ██║   ██║   ███████║",
    " ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝ ╚══╝╚══╝ ╚═╝  ╚═╝   ╚═╝   ╚══════╝"];

const LOWER_LOGO:[&str;LOGOS_HEIGHT] = [
" ██████╗  █████╗ ███╗   ███╗███████╗     ██████╗ ███████╗    ██╗     ██╗███████╗███████╗",
"██╔════╝ ██╔══██╗████╗ ████║██╔════╝    ██╔═══██╗██╔════╝    ██║     ██║██╔════╝██╔════╝",
"██║  ███╗███████║██╔████╔██║█████╗      ██║   ██║█████╗      ██║     ██║█████╗  █████╗",
"██║   ██║██╔══██║██║╚██╔╝██║██╔══╝      ██║   ██║██╔══╝      ██║     ██║██╔══╝  ██╔══╝",
"╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗    ╚██████╔╝██║         ███████╗██║██║     ███████╗",
" ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝     ╚═════╝ ╚═╝         ╚══════╝╚═╝╚═╝     ╚══════╝"
];


// extracted from https://conwaylife.com/ref/lexicon/lex_s.htm#soup
const SOUPS_CONFIGURATION:[&str; MAX_ROWS] = [
"                                                                               ",
"                                                                               ",
"                                                                               ",
"                                                                               ",
"             ♦ ♦ ♦ ♦   ♦ ♦  ♦ ♦ ♦  ♦    ♦  ♦ ♦ ♦  ♦ ♦   ♦ ♦ ♦ ♦                ",
"              ♦  ♦  ♦ ♦  ♦              ♦  ♦ ♦  ♦  ♦                           ",
"               ♦ ♦ ♦   ♦  ♦  ♦        ♦  ♦  ♦   ♦ ♦ ♦                          ",
"             ♦  ♦ ♦  ♦ ♦ ♦  ♦   ♦      ♦   ♦  ♦ ♦ ♦  ♦ ♦  ♦                    ",
"              ♦ ♦ ♦ ♦  ♦    ♦ ♦  ♦ ♦ ♦ ♦ ♦  ♦ ♦    ♦  ♦ ♦ ♦ ♦                  ",
"                  ♦ ♦    ♦ ♦  ♦  ♦  ♦  ♦ ♦    ♦ ♦                              ",
"               ♦ ♦ ♦    ♦ ♦    ♦    ♦    ♦ ♦    ♦ ♦ ♦                          ",
"             ♦   ♦   ♦  ♦ ♦    ♦ ♦  ♦ ♦    ♦ ♦  ♦   ♦   ♦                      ",
"             ♦ ♦  ♦   ♦    ♦          ♦    ♦   ♦  ♦ ♦                          ",
"             ♦  ♦  ♦    ♦ ♦ ♦ ♦   ♦ ♦ ♦   ♦ ♦ ♦ ♦    ♦  ♦  ♦                   ",
"             ♦  ♦ ♦ ♦  ♦ ♦   ♦ ♦    ♦    ♦ ♦   ♦ ♦  ♦ ♦ ♦  ♦                   ",
"               ♦      ♦ ♦    ♦    ♦    ♦ ♦      ♦                              ",
"             ♦ ♦ ♦ ♦ ♦  ♦  ♦ ♦ ♦   ♦    ♦   ♦ ♦ ♦  ♦  ♦ ♦ ♦ ♦ ♦                ",
"              ♦     ♦     ♦   ♦ ♦ ♦   ♦     ♦     ♦                            ",
"              ♦ ♦  ♦    ♦ ♦ ♦ ♦ ♦ ♦ ♦ ♦ ♦ ♦ ♦ ♦ ♦ ♦ ♦    ♦  ♦ ♦                ",
"             ♦ ♦ ♦ ♦  ♦ ♦ ♦       ♦  ♦       ♦ ♦ ♦  ♦ ♦ ♦ ♦                    ",
"                                                                               ",
"                                                                               ",
"                                                                               ",
"                                                                               ",
"                                                                               "
];


struct Life {
    world: [[char; MAX_COLS]; MAX_ROWS],
    iterations: u64,
    stdout: Stdout
}

impl Life {
    fn board() -> Self {
        Self {
            world: [[EMPTY_CHAR; MAX_COLS]; MAX_ROWS],
            iterations: 0,
            stdout: stdout()
        }
    }

    fn show_logo(&mut self, logo:[&str;6], row_offset: u16) {

        for row in 0..LOGOS_HEIGHT {
            let real_row: u16 = row_offset + (row as u16);
            queue!(self.stdout,
                cursor::MoveTo(0, real_row),
                style::PrintStyledContent(logo[row as usize].yellow())
            );
        }
    }

    fn clean_bord(&mut self) {
        for row in 0..MAX_ROWS {
            for col in 0..MAX_COLS {
                self.world[row][col] = CELL_CHAR;
            }
        }
    }

    fn fill_bord(&mut self, character: char) {
        for row in 0..MAX_ROWS {
            for col in 0..MAX_COLS {
                self.world[row][col] = character;
            }
        }
    }

    fn load_pattern (&mut self, pattern:[&str; MAX_ROWS]) {
        for row in 0..MAX_ROWS {
            let line = pattern[row].to_string();
            println!("{}", line);
            for col in 0..MAX_COLS {
                let character = line.as_bytes()[col];

                println!("({},{}) => {}", row, col, character as char);

                // self.world[row][col] = pattern[row].to_string().as_bytes()[col] as char;
            }
        }
    }

    fn get_position(&self, row:usize, col:usize) -> Option<char> {
        return Some(self.world[row][col]);
    }

    fn increase_iteration(&mut self) {
        self.iterations = self.iterations + 1;
    }

    fn show_iteration(&self) -> u64 {
        return self.iterations;
    }

    fn print_bord(&mut self) {

        self.stdout.execute(terminal::Clear(terminal::ClearType::All)).expect("Oops!! Somethins went wrong!!");
        self.show_logo(UPPER_LOGO, 0);
        for row in 0..MAX_ROWS {
            for col in 0..MAX_COLS {
                let cell:char = self.get_position(row, col).unwrap();
                let real_row = row + LOGOS_HEIGHT;
                if (col == 0 || col == MAX_COLS - 1) || (row == 0 || row == MAX_ROWS - 1) {
                    queue!(
                        self.stdout,
                        cursor::MoveTo(col as u16, real_row as u16),
                        style::PrintStyledContent( BORDER_CHAR.white())
                    );
                } else {
                    queue!(
                        self.stdout,
                        cursor::MoveTo(col as u16, real_row as u16),
                        style::PrintStyledContent(cell.magenta())
                    );
                }
            }
        }
        self.show_logo(LOWER_LOGO, (MAX_ROWS + LOGOS_HEIGHT) as u16);

        // We need to print a carry return in order to avoid an % at the end of print area
        queue!(
            self.stdout,
            cursor::MoveTo(MAX_COLS as u16, (MAX_ROWS + (LOGOS_HEIGHT * 2)) as u16 as u16),
            style::PrintStyledContent("\n".magenta())
        );
        self.stdout.flush().expect("Oops!! Somethins went wrong!!");
    }

    fn is_alive(&self, row:u16, col:u16) -> bool {
        return self.world[usize::from(row)][usize::from(col)] == CELL_CHAR;
    }

    fn is_outside_world (&self, row:u16, col:u16) {
        if (usize::from(col) == 0 ||
            usize::from(usize::from(col)) == MAX_COLS - 1) ||
            (usize::from(row) == 0 || usize::from(row) == MAX_ROWS - 1) {

        }
    }

    fn cell_state(&self, row:u16, col:u16, state:bool) {
        self.world[usize::from(row)][usize::from(col)] == if state {CELL_CHAR} else {EMPTY_CHAR};
    }

    fn check_neighbors(&self, row:u16, col:u16) -> u8 {

        for row_position in 0..ROW_POSITIONS.len() {
            for col_position in 0..COL_POSITIONS.len() {
                if self.is_alive(row + row_position as u16, col + col_position as u16) {

                }
                println!("{}{}", row + ROW_POSITIONS[row_position] as u16, col + COL_POSITIONS[col_position] as u16);
            }
        }

        return 0;
    }

    fn calc_cell_state(&self, row:u16, col:u16) {
        let mut neighbors:u8 = 0;

        if (col as i16) - 1 < 0 {
            let col = MAX_COLS;
        }

        if (row as i16) - 1 < 0 {
            let row = MAX_ROWS;
        }

        if (col as i16) + 1 > MAX_COLS.try_into().unwrap() {
            let col = 0;
        }

        if (row as i16) + 1 > MAX_ROWS.try_into().unwrap() {
            let row = 0;
        }

        neighbors = self.check_neighbors(row, col);
        if self.is_alive(row, col) {
            match neighbors {
                cell_neighbors if cell_neighbors < 2 && cell_neighbors > 3 => self.cell_state(row, col, DEAD),
                2|3 => self.cell_state(row, col, ALIVE),
                _ => {}
            }
        } else {
            match neighbors {
                3 => self.cell_state(row, col, ALIVE),
                _ => {}
            }
        }
    }

    fn run(&self) {
        loop {
            for row in 0..MAX_ROWS {
                for col in 0..MAX_COLS {
                    self.calc_cell_state(row as u16, col as u16);
                }
            }
        }
    }
}

fn main() {

    let mut life_board = Life::board();

    life_board.load_pattern(SOUPS_CONFIGURATION);
    // life_board.print_bord();

    /*
    life_board.print_bord();
    life_board.run();
     */
}