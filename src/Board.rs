use array2d::Array2D;
use rand;
use rand::Rng;



use graphics::types::Color;
use graphics::{Context, Graphics};

use piston::input::{GenericEvent, UpdateArgs};



const NEIGHBOR_SPACES: [i32; 3] = [-1, 0, 1];

 lazy_static! {
     static ref NEIGHBOR_SPACES_CROSS: Vec<(i32, i32)> = {
                let mut product: Vec<(i32, i32)> = vec!();


                for up in NEIGHBOR_SPACES.iter() {
                    for left in NEIGHBOR_SPACES.iter() {

                        if (*left == 0) && (*up == 0)  {
                            continue;
                        }

                        product.push((*up, *left));
                    }
                }
                return product;

    };
}
#[derive(Debug, Clone)]
pub struct Board {
    array: Array2D<Cell>
}

impl Board {
    pub fn new() -> Board {
        let mut rng = rand::thread_rng();

        let mut array = Array2D::filled_with(Cell::new(0, 0), 100, 100);
        for col in 0..array.column_len() {
            for row in 0..array.row_len() {

                let random = rng.gen::<bool>();
                let mut el = array.get_mut(row, col).unwrap();
                el.on = random;
                el.x = row;
                el.y = col;
            }
        }

        return Board { array };
    }
    pub fn next(&self) -> Board {
        let old_board = self;
        let mut nex = self.clone();
        let neighbor_spaces: Vec<i32> = vec![-1, 0, 1];
        for col in 0..old_board.array.column_len() {
            for row in 0..old_board.array.row_len() {
                let mut tally = 0;
                for up in neighbor_spaces.iter() {
                    for left in neighbor_spaces.iter() {
                        if *up == (0 as i32)  && *left == (0 as i32) {
                            continue;
                        }
                        let neighbor_col = ((col as i32) + *up) as usize;
                        let neighbor_row = ((row as i32) + *left) as usize;
                        let neighbor: Option<&Cell> = old_board.array.get(neighbor_row,
                                                                         neighbor_col);

                        let neighbor_value: usize = match neighbor {
                            Some(Cell { on: true, x: _, y: _ }) => 1,
                            Some(Cell { on: false, x: _, y: _ }) => 0,
                            None => 0,
                        };

                        tally += neighbor_value;
                    }
                }
                let old_cell = self.array.get(col, row).unwrap();
                let pair: (bool, usize) = (old_cell.on, tally);

                let on = match pair {
                    (true, 2 ) => true,
                    (true, 3 ) => true,
                    (false, 3) => true,
                    _ => false,
                };

                let cell = nex.array.get_mut(col, row).unwrap();
                cell.on = on;
            }
        }
        return nex;
    }

    pub fn print(&self) -> () {
        let n = self;
        for row in n.array.rows_iter() {
            for el in row {
                print!(" ");
                if el.on {
                    print!("X");
                } else {
                    print!("_");
                }
            }
            print!("\n");
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct Cell {
    on: bool,
    x: usize,
    y: usize
}
impl Cell {
    fn new(x: usize, y: usize) -> Cell {
        return Cell{on: false, x, y};
    }
}


/// Handles events for Sudoku game.
pub struct GameboardController {
    /// Stores the gameboard state.
    pub gameboard: Board,
    dt: f64
}

impl GameboardController {
    /// Creates a new gameboard controller.
    pub fn new(gameboard: Board) -> GameboardController {
        GameboardController {
            gameboard: gameboard,
            dt: 0.0
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {

    }
    pub fn update(&mut self, args: &UpdateArgs) {
// Rotate 2 radians per second.
        self.dt += args.dt;
        if self.dt > 0.01 {
            self.gameboard = self.gameboard.next();
            self.dt -= 0.01;
        }
    }

}


/// Stores gameboard view settings.
pub struct GameboardViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of cell along horizontal and vertical edge.
    pub size: f64,
    /// Background color.
    pub background_color: Color,
    pub live_color: Color,
    pub dead_color: Color,
}

impl GameboardViewSettings {
    /// Creates new gameboard view settings.
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            position: [10.0; 2],
            size: 10.0,
            background_color: [0.8, 0.8, 1.0, 1.0],
            live_color: [1.0, 1.0, 1.0, 1.0],
            dead_color: [0.8, 0.8, 1.0, 1.0],
        }
    }
}

/// Stores visual information about a gameboard.
pub struct GameboardView {
    /// Stores gameboard view settings.
    pub settings: GameboardViewSettings,
}

impl GameboardView {
    /// Creates a new gameboard view.
    pub fn new(settings: GameboardViewSettings) -> GameboardView {
        GameboardView {
            settings,
        }
    }

    fn draw_cell<G: Graphics>(c: &Context, g: &mut G, settings: &GameboardViewSettings, x: usize, y:
    usize, color: Color) {
        use graphics::{Line, Rectangle};

        let cell_size = settings.size;
        let pos = [x as f64 * cell_size, y as f64 * cell_size];
        let cell_rect = [
            settings.position[0] + pos[0], settings.position[1] + pos[1],
            cell_size, cell_size
        ];
        Rectangle::new(color)
            .draw(cell_rect, &c.draw_state, c.transform, g);

    }
    fn draw_board<G: Graphics>(c: &Context,
                               g: &mut G,
                               board: &Board,
                               settings: &GameboardViewSettings) {

        for row in board.array.rows_iter() {
            for el in row {
                if el.on {
                    GameboardView::draw_cell(c, g, settings, el.x, el.y, settings.dead_color);
                } else {
                    GameboardView::draw_cell(c, g, settings, el.x, el.y, settings.live_color);
                }
            }
        }
    }
    /// Draw gameboard.
    pub fn draw<G: Graphics>(&self, controller: &GameboardController, c: &Context, g: &mut G) {
        use graphics::{Line, Rectangle};

        let ref settings = self.settings;
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size,
        ];

// Draw board background.
        Rectangle::new(settings.background_color)
            .draw(board_rect, &c.draw_state, c.transform, g);

        GameboardView::draw_board(c, g, &controller.gameboard, settings);




    }
}
