use graphics::Drawable;
use viering_chess::*;
use ggez::*;

/*
TODO:

Set path for resources to be in src/, not in target/debug/

Handle promotion better

Make it prettier

*/

const SQUARE_SIZE: f32 = 100.;
const SIDEBAR_WIDTH: f32 = 400.;
const PADDING: f32 = 40.0;

const SCREEN_HEIGHT: f32 = SQUARE_SIZE * 8. + PADDING * 2.;
const SCREEN_WIDTH: f32 = SQUARE_SIZE * 8. + PADDING * 2. + SIDEBAR_WIDTH;



struct State {
    game: Game,
    board: Vec<graphics::Rect>,
    pieces: Vec<Option<Piece>>,
    moves: Vec<Position>,
    board_moves: Vec<bool>,
    from: Position
}

fn draw_image(rect: graphics::Rect, path: String, ctx: &mut Context, canvas: &mut graphics::Canvas) {
    let image = graphics::Image::from_path(ctx, path).expect("Error reading image");
    let dim = image.dimensions(ctx).unwrap_or(graphics::Rect::new(0.0,0.0,rect.w,rect.h));
    let dest = glam::Vec2{x: SQUARE_SIZE * (rect.x) as f32 + PADDING, y: SQUARE_SIZE * (rect.y) as f32 + PADDING};
    let scale = glam::Vec2{x: (SQUARE_SIZE / dim.w), y: (SQUARE_SIZE / dim.h)};

    canvas.draw(&image, graphics::DrawParam::default().dest(dest).scale(scale));
}

fn draw_square(rect: graphics::Rect, ctx: &mut Context, canvas: &mut graphics::Canvas, color: graphics::Color) {
    let square = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        rect,
        color,
    )
    .expect("error creating square");
    canvas.draw(&square, graphics::DrawParam::default());
}

fn draw_circle(rect: graphics::Rect, ctx: &mut Context, canvas: &mut graphics::Canvas, color: graphics::Color) {
    // make our circle instance
    let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(), // Fill the circle
        mint::Point2::from([rect.x + SQUARE_SIZE/2.0, rect.y + SQUARE_SIZE/2.0]), // Center position
        SQUARE_SIZE/8.0, // radius
        0.2, // mesh tolerance, higher smoother
        color // Colouring
    )
    .expect("Error creating circle");    // draw our circle instance   

    canvas.draw(&circle, graphics::DrawParam::default());
}


fn draw_highlight(rect: graphics::Rect, ctx: &mut Context, canvas: &mut graphics::Canvas, color: graphics::Color) {
    let points = vec![
        [
            mint::Point2::from([rect.x, rect.y]),
            mint::Point2::from([rect.x + SQUARE_SIZE/4.0, rect.y]),
            mint::Point2::from([rect.x, rect.y + SQUARE_SIZE/4.0]),
            mint::Point2::from([rect.x, rect.y])
        ],
        [
            mint::Point2::from([rect.x + SQUARE_SIZE, rect.y + SQUARE_SIZE]),
            mint::Point2::from([rect.x + SQUARE_SIZE - SQUARE_SIZE/4.0, rect.y + SQUARE_SIZE]),
            mint::Point2::from([rect.x + SQUARE_SIZE, rect.y + SQUARE_SIZE - SQUARE_SIZE/4.0]),
            mint::Point2::from([rect.x + SQUARE_SIZE, rect.y + SQUARE_SIZE])
        ],
        [
            mint::Point2::from([rect.x + SQUARE_SIZE, rect.y]),
            mint::Point2::from([rect.x + SQUARE_SIZE - SQUARE_SIZE/4.0, rect.y]),
            mint::Point2::from([rect.x + SQUARE_SIZE, rect.y + SQUARE_SIZE/4.0]),
            mint::Point2::from([rect.x + SQUARE_SIZE, rect.y])
        ],
        [
            mint::Point2::from([rect.x, rect.y + SQUARE_SIZE]),
            mint::Point2::from([rect.x + SQUARE_SIZE/4.0, rect.y + SQUARE_SIZE]),
            mint::Point2::from([rect.x, rect.y + SQUARE_SIZE - SQUARE_SIZE/4.0]),
            mint::Point2::from([rect.x, rect.y + SQUARE_SIZE])
        ]
    ];
    for p in &points {

        // make our triangle instance
        let triangle = graphics::Mesh::new_polyline(
            ctx,
            graphics::DrawMode::fill(), // Fill the triangle
            p,
            color // Colouring
        )
        .expect("Error creating triangle");    // draw our triangle instance   

        canvas.draw(&triangle, graphics::DrawParam::default());

    } 
}


/*
Convert piece to path to its image

*/
fn piece_to_path(piece: Option<Piece>) -> String {
    match piece {
        Some(Piece {color: Color::White, piece_type: PieceType::Pawn})    => "/white_pawn-min.png".to_string(),
        Some(Piece {color: Color::White, piece_type: PieceType::Knight})  => "/white_knight-min.png".to_string(),
        Some(Piece {color: Color::White, piece_type: PieceType::Bishop})  => "/white_bishop-min.png".to_string(),
        Some(Piece {color: Color::White, piece_type: PieceType::Rook})    => "/white_rook-min.png".to_string(),
        Some(Piece {color: Color::White, piece_type: PieceType::Queen})   => "/white_queen-min.png".to_string(),
        Some(Piece {color: Color::White, piece_type: PieceType::King})    => "/white_king-min.png".to_string(),
        Some(Piece {color: Color::Black, piece_type: PieceType::Pawn})    => "/black_pawn-min.png".to_string(),
        Some(Piece {color: Color::Black, piece_type: PieceType::Knight})  => "/black_knight-min.png".to_string(),
        Some(Piece {color: Color::Black, piece_type: PieceType::Bishop})  => "/black_bishop-min.png".to_string(),
        Some(Piece {color: Color::Black, piece_type: PieceType::Rook})    => "/black_rook-min.png".to_string(),
        Some(Piece {color: Color::Black, piece_type: PieceType::Queen})   => "/black_queen-min.png".to_string(),
        Some(Piece {color: Color::Black, piece_type: PieceType::King})    => "/black_king-min.png".to_string(),
        None => "".to_string(),
    }
}

/*
Write Text

*/
fn write_text(input: String, tx: f32, ty: f32, canvas: &mut graphics::Canvas) {
    let mut text = graphics::Text::new(input);
    let pos = glam::Vec2{x: tx, y: ty};

    text.set_scale(graphics::PxScale::from(30.0));

    text.draw(canvas, graphics::DrawParam::default().dest(pos));
}


impl State {
    fn init(&mut self) {
        self.game = Game::new();
        self.board = vec![];
        self.pieces = vec![None; 64];
        self.moves = vec![];
        self.board_moves = vec![false; 64];
        self.from = Position::new(0,0);
        
        for i in 0..64 {
            self.board.push(graphics::Rect::new(
                SQUARE_SIZE * (i % 8) as f32 + PADDING,
                SQUARE_SIZE * (7 - (i / 8)) as f32 + PADDING,
                SQUARE_SIZE,
                SQUARE_SIZE
            ));
        }
    
        let mut i = 0;
        for y in (0..8).rev() {
            for x in 0..8 {
                self.pieces[y*8+x] = self.game.squares[i];
                i += 1;
            }
        }
    }
    
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        let mut opacity: f32 = 1.0;
        if let GameState::AwaitingPromotion(_) = self.game.game_state {
            opacity = 0.6;
        }


        for i in 0..64 {
            if (i / 8 % 2 == 0 && i % 2 == 0) || (i / 8 % 2 == 1 && i % 2 == 1){
                // Black Square
                draw_square(self.board[i], ctx, &mut canvas, graphics::Color::new(0.71, 0.53, 0.39, opacity));
            }
            else {
                // White Square
                draw_square(self.board[i], ctx, &mut canvas, graphics::Color::new(0.94, 0.85, 0.71, opacity));
            }

            if self.board_moves[i] {
                // Possible moves
                if self.pieces[i].is_none() {
                    // Empty square hit
                    draw_circle(self.board[i], ctx, &mut canvas, graphics::Color::new(0.08, 0.34, 0.12, 0.85));                    
                }
                else {
                    // Piece is hit
                    draw_highlight(self.board[i], ctx, &mut canvas, graphics::Color::new(0.08, 0.34, 0.12, 0.85));                    
                }
            }
            
            // King in check
            for color in [Color::White, Color::Black] {
                if self.game.game_state == GameState::Check(color) || self.game.game_state == GameState::Checkmate(color) {
                    let piece = self.pieces[i].unwrap_or(Piece {color: Color::White, piece_type: PieceType::Pawn});

                    if piece.color == color && piece.piece_type == PieceType::King {
                        draw_square(self.board[i], ctx, &mut canvas, graphics::Color::new(1.0, 0.0, 0.0, opacity));
                    }
                }
            }
        }

        // Highlight the "from" piece
        if self.moves.len() > 0 {
            let pos = (self.from.y * 8 + self.from.x) as usize;
            draw_square(self.board[pos], ctx, &mut canvas, graphics::Color::new(0.0, 0.5, 0.0, 0.3));
        }

        // Draw the image pieces
        for i in 0..64 {
            if self.game.squares[i].is_some() {
                let path = piece_to_path(self.game.squares[i]);
                if path == "" { continue; }

                draw_image(graphics::Rect::new((i % 8) as f32, (i / 8) as f32, 60.0, 60.0), path, ctx, &mut canvas);
            }
        }

        // Draw possible promotion options
        if let GameState::AwaitingPromotion(p) = self.game.game_state {
            let mut paths = vec![ "/white_queen-min.png", "/white_rook-min.png", "/white_bishop-min.png", "/white_knight-min.png"];
            let mut dy: i32 = -1;
            if p.y == 0 {
                // black
                paths = vec!["/black_queen-min.png", "/black_rook-min.png", "/black_bishop-min.png", "/black_knight-min.png"]; 
                dy = 1;
            }
            for i in 0..4 {
                let y = p.y as i32 +(i+1)*dy;
                let x = p.x as i32;
                draw_square(self.board[(y*8+x) as usize], ctx, &mut canvas, graphics::Color::new(0.0, 0.0, 0.0, 1.0));
                draw_image(graphics::Rect::new((p.x) as f32, (7 - y) as f32, 60.0, 60.0), paths[i as usize].to_string(), ctx, &mut canvas);
            }
        }


        /*
        Info Sidebar
        */
        
        let mut white_text = "White".to_string();
        let mut black_text = "Black".to_string();

        if self.game.turn == Color::White { white_text = "White *".to_string(); }
        if self.game.turn == Color::Black { black_text = "Black *".to_string(); }
        
        if let GameState::Checkmate(Color::White) = self.game.game_state { black_text = "Black Won".to_string(); white_text = "White -".to_string(); }
        if let GameState::Checkmate(Color::Black) = self.game.game_state { white_text = "White Won".to_string(); black_text = "Black -".to_string();}

        if let GameState::Draw = self.game.game_state {
            white_text = "White -".to_string();
            black_text = "Black -".to_string();
        }
        if let GameState::Check(Color::White) = self.game.game_state { white_text = "White in check *".to_string(); }
        if let GameState::Check(Color::Black) = self.game.game_state { black_text = "Black in check *".to_string(); }

        write_text(
            white_text, 
            PADDING * 2. + SQUARE_SIZE * 8.,
            PADDING * 1.5 + SQUARE_SIZE * 4.,
            &mut canvas
        );
        write_text(
            black_text, 
            PADDING * 2. + SQUARE_SIZE * 8.,
            PADDING * 0.5 + SQUARE_SIZE * 4.,
            &mut canvas
        );

        // Restart
        write_text(
            "Restart Game".to_string(), 
            PADDING * 2. + SQUARE_SIZE * 8.,
            -PADDING * 0.5 + SQUARE_SIZE * 8.,
            &mut canvas
        );


        canvas.finish(ctx)?;

        Ok(())
    }


    fn mouse_button_down_event(&mut self,_ctx: &mut Context,_button: event::MouseButton,x: f32,y: f32,) -> Result<(), GameError> {
        if x < PADDING || y < PADDING || x > PADDING + SQUARE_SIZE * 8. || y > PADDING + SQUARE_SIZE * 8. { 
            // Not pressed on the chessboard

            // Restart pressed
            let start_x = PADDING * 2. + SQUARE_SIZE * 8.;
            let start_y = -PADDING * 0.5 + SQUARE_SIZE * 8.;
            if x > start_x && y > start_y && x < start_x + SIDEBAR_WIDTH - PADDING && y < start_y + PADDING {
                // RESTART GAME
                self.init();
            }
            
            

            return Ok(()); 
        }

        let sq_x: u8 = ((x - PADDING) / SQUARE_SIZE) as u8;
        let sq_y: u8 = 7 - ((y - PADDING) / SQUARE_SIZE) as u8;
        
        if let GameState::AwaitingPromotion(p) = self.game.game_state {
            // whites direction
            let mut direction: i32 = -1;
            // if blacks direction
            if self.game.turn == Color::White { direction = 1; }
            
            if sq_x == p.x {
                if sq_y == (p.y as i32 + direction * 1) as u8 { let _result: MoveResult = self.game.promote(PieceType::Queen); }
                if sq_y == (p.y as i32 + direction * 2) as u8 { let _result: MoveResult = self.game.promote(PieceType::Rook); }
                if sq_y == (p.y as i32 + direction * 3) as u8 { let _result: MoveResult = self.game.promote(PieceType::Bishop); }
                if sq_y == (p.y as i32 + direction * 4) as u8 { let _result: MoveResult = self.game.promote(PieceType::Knight); }
            }
            return Ok(());
        }

        // When possible moves are shown
        if !self.moves.is_empty() {
            // Move piece to a highlighted square
            if self.board_moves[(sq_y*8 + sq_x) as usize] {
                // Make Move
                let _result: MoveResult = self.game.make_move(Position::new(self.from.x, self.from.y), Position::new(sq_x, sq_y)); 

                // Update the game board
                let mut i = 0;
                for y in (0..8).rev() {
                    for x in 0..8 {
                        self.pieces[y*8+x] = self.game.squares[i];
                        i += 1;
                    }
                }
            }

            // Clear moves of piece
            self.moves = vec![];
            self.board_moves = vec![false; 64];

            // Toggle showing moves when same piece clicked twice
            if self.from.x == sq_x && self.from.y == sq_y { return Ok(()); }
        }
        
        // piece exists and is of right color
        let piece = self.pieces[(sq_y*8 + sq_x) as usize];
        if piece.is_some() && piece.unwrap().color == self.game.turn {
            self.moves = self.game.get_possible_moves(Position::new(sq_x, sq_y));
            self.from = Position::new(sq_x, sq_y);
    
            self.board_moves = vec![false; 64];
            for p in &self.moves {
                self.board_moves[(p.y*8 + p.x) as usize] = true;
            }
        }

        Ok(())
    }
}

fn main() {
    let game = Game::new();
    //game.load_fen("4n1bk/7p/8/8/8/2B5/8/4K3 b - - 0 1");
    //game.load_fen("8/2p2b2/1pk5/3Q4/2P5/8/8/3K4 b - - 0 1");

    let mut state = State {
        game: game,
        board: vec![],
        pieces: vec![None; 64],
        moves: vec![],
        board_moves: vec![false; 64],
        from: Position::new(0,0)
    };

    state.init();

    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("davbjor-chess-gui", "davidbjorklund")
        .default_conf(c)
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
    
}
