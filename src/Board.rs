use array2d::Array2D;
use rand;
use rand::Rng;



use graphics::types::Color;
use graphics::{Context, Graphics};

use piston::input::{GenericEvent, UpdateArgs};



#[derive(Debug, Clone)]
pub struct Board {
    array: Array2D<Cell>
}

impl Board {
    pub fn new() -> Board {
        let mut rng = rand::thread_rng();

        let mut array = Array2D::filled_with(Cell::new(0, 0), 9, 9);
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
        let mut n = self.clone();
        let neighbor_spaces: Vec<i32> = vec![-1, 0, 1];
        for col in 0..self.array.column_len() {
            for row in 0..self.array.row_len() {
                let mut tally = 0;
                for up in neighbor_spaces.iter() {
                    for left in neighbor_spaces.iter() {
                        if *up == (0 as i32)  && *left == (0 as i32) {
                            continue;
                        }
                        let neighbor_col = ((col as i32) + *up) as usize;
                        let neighbor_row = ((row as i32) + *left) as usize;
                        let neighbor: Option<&Cell> = self.array.get(neighbor_row, neighbor_col);

                        let neighbor_value: i32 = match neighbor {
                            Some(Cell { on: true, x: _, y: _ }) => 1,
                            Some(Cell { on: false, x: _, y: _ }) => 0,
                            None => 0,
                        };

                        tally += neighbor_value;
                    }
                }
                let cell = n.array.get_mut(col, row).unwrap();
                let on = if cell.on {

                    match tally {

                        2..=3 => true,
                        _ => false
                    }

                } else {
                    match tally {
                        3 => true,
                        _ => false
                    }
                };
                cell.on = on;
            }
        }
        return n;
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
        if self.dt > 0.5 {

            self.gameboard = self.gameboard.next();
            self.dt = 0.0;
        }
    }

}


/// Stores gameboard view settings.
pub struct GameboardViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of gameboard along horizontal and vertical edge.
    pub size: f64,
    /// Background color.
    pub background_color: Color,
    /// Border color.
    pub border_color: Color,
    /// Edge color around the whole board.
    pub board_edge_color: Color,
    /// Edge color between the 3x3 sections.
    pub section_edge_color: Color,
    /// Edge color between cells.
    pub cell_edge_color: Color,
    /// Edge radius around the whole board.
    pub board_edge_radius: f64,
    /// Edge radius between the 3x3 sections.
    pub section_edge_radius: f64,
    /// Edge radius between cells.
    pub cell_edge_radius: f64,
    pub live_background_color: Color,
}

impl GameboardViewSettings {
    /// Creates new gameboard view settings.
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            position: [10.0; 2],
            size: 400.0,
            background_color: [0.8, 0.8, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
            live_background_color: [1.0, 1.0, 1.0, 1.0],
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
    usize) {
        use graphics::{Line, Rectangle};

        let cell_size = settings.size / 9.0;
        let pos = [x as f64 * cell_size, y as f64 * cell_size];
        let cell_rect = [
            settings.position[0] + pos[0], settings.position[1] + pos[1],
            cell_size, cell_size
        ];
        Rectangle::new(settings.live_background_color)
            .draw(cell_rect, &c.draw_state, c.transform, g);

    }
    fn draw_board<G: Graphics>(c: &Context, g: &mut G, board: &Board,
                               settings: &GameboardViewSettings) {

        for row in board.array.rows_iter() {
            for el in row {
                if el.on {
                    GameboardView::draw_cell(c, g, settings, el.x, el.y);
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



        // Declare the format for cell and section lines.
        let cell_edge = Line::new(settings.cell_edge_color, settings.cell_edge_radius);
        let section_edge = Line::new(settings.section_edge_color, settings.section_edge_radius);

        // Generate and draw the lines for the Sudoku Grid.
        for i in 0..9 {
            let x = settings.position[0] + i as f64 / 9.0 * settings.size;
            let y = settings.position[1] + i as f64 / 9.0 * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            let hline = [settings.position[0], y, x2, y];

            // Draw Section Lines instead of Cell Lines
            if (i % 3) == 0 {
                section_edge.draw(vline, &c.draw_state, c.transform, g);
                section_edge.draw(hline, &c.draw_state, c.transform, g);
            }
            // Draw the regular cell Lines
            else {
                cell_edge.draw(vline, &c.draw_state, c.transform, g);
                cell_edge.draw(hline, &c.draw_state, c.transform, g);
            }
        }

        // Draw board edge.
        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius)
            .draw(board_rect, &c.draw_state, c.transform, g);

    }
}
