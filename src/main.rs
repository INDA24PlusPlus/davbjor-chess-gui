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

const SCREEN_HEIGHT: f32 = SQUARE_SIZE * 8.;
const SCREEN_WIDTH: f32 = SQUARE_SIZE * 8.;



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
    let dest = glam::Vec2{x: SQUARE_SIZE * (rect.x) as f32, y: SQUARE_SIZE * (rect.y) as f32};
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

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //event::process_event(ctx, event);
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
            if i / 8 % 2 == 0 && i % 2 == 0 || i / 8 % 2 == 1 && i % 2 == 1{
                // White Square
                draw_square(self.board[i], ctx, &mut canvas, graphics::Color::new(1.0, 1.0, 1.0, opacity));
            }
            else {
                // Black Square
                draw_square(self.board[i], ctx, &mut canvas, graphics::Color::new(0.0, 0.0, 0.0, opacity));
            }

            if self.board_moves[i] {
                // Possible moves
                if self.pieces[i].is_none() {
                    draw_circle(self.board[i], ctx, &mut canvas, graphics::Color::new(0.0, 1.0, 0.0, opacity));                    
                }
                else {
                    // Piece is hit
                    draw_highlight(self.board[i], ctx, &mut canvas, graphics::Color::new(0.0, 1.0, 0.0, opacity));                    
                }
            }
            
            // King in check
            for color in [Color::White, Color::Black] {
                if self.game.game_state == GameState::Check(color) {
                    let piece = self.pieces[i].unwrap_or(Piece {color: Color::White, piece_type: PieceType::Pawn});

                    if piece.color == color && piece.piece_type == PieceType::King {
                        draw_square(self.board[i], ctx, &mut canvas, graphics::Color::new(1.0, 0.0, 0.0, opacity));
                    }
                }
            }

            
        }

        for i in 0..64 {
            if self.game.squares[i].is_some() {
                let mut path = String::new();
                match self.game.squares[i].unwrap() {
                    Piece {color: Color::White, piece_type: PieceType::Pawn}    => path = "/white_pawn_60.png".to_string(),
                    Piece {color: Color::White, piece_type: PieceType::Knight}  => path = "/white_knight_60.png".to_string(),
                    Piece {color: Color::White, piece_type: PieceType::Bishop}  => path = "/white_bishop_60.png".to_string(),
                    Piece {color: Color::White, piece_type: PieceType::Rook}    => path = "/white_rook_60.png".to_string(),
                    Piece {color: Color::White, piece_type: PieceType::Queen}   => path = "/white_queen_60.png".to_string(),
                    Piece {color: Color::White, piece_type: PieceType::King}    => path = "/white_king_60.png".to_string(),
                    Piece {color: Color::Black, piece_type: PieceType::Pawn}    => path = "/black_pawn_60.png".to_string(),
                    Piece {color: Color::Black, piece_type: PieceType::Knight}  => path = "/black_knight_60.png".to_string(),
                    Piece {color: Color::Black, piece_type: PieceType::Bishop}  => path = "/black_bishop_60.png".to_string(),
                    Piece {color: Color::Black, piece_type: PieceType::Rook}    => path = "/black_rook_60.png".to_string(),
                    Piece {color: Color::Black, piece_type: PieceType::Queen}   => path = "/black_queen_60.png".to_string(),
                    Piece {color: Color::Black, piece_type: PieceType::King}    => path = "/black_king_60.png".to_string(),
                    _ => (),
                }
                if path == "" { continue; }
                draw_image(graphics::Rect::new((i % 8) as f32, (i / 8) as f32, 30.0, 30.0), path, ctx, &mut canvas);
            }
        }

        // Colors are reversed on promotion
        // Test for white
        if self.game.turn == Color::Black {
            match self.game.game_state {
                GameState::AwaitingPromotion(p) => {
                    // Draw piece one step below promotion
                    // Draw white queen
                    draw_square(self.board[((p.y-1)*8+p.x) as usize], ctx, &mut canvas, graphics::Color::new(0.0, 0.0, 0.0, 1.0));
                    draw_image(graphics::Rect::new((p.x) as f32, (7 - (p.y - 1)) as f32, 30.0, 30.0), "/white_queen_60.png".to_string(), ctx, &mut canvas);
                    
                    // Draw white rook
                    draw_square(self.board[((p.y-2)*8+p.x) as usize], ctx, &mut canvas, graphics::Color::new(0.0, 0.0, 0.0, 1.0));
                    draw_image(graphics::Rect::new((p.x) as f32, (7 - (p.y - 2)) as f32, 30.0, 30.0), "/white_rook_60.png".to_string(), ctx, &mut canvas);

                    // Draw white bishop
                    draw_square(self.board[((p.y-3)*8+p.x) as usize], ctx, &mut canvas, graphics::Color::new(0.0, 0.0, 0.0, 1.0));
                    draw_image(graphics::Rect::new((p.x) as f32, (7 - (p.y - 3)) as f32, 30.0, 30.0), "/white_bishop_60.png".to_string(), ctx, &mut canvas);
                    
                    // Draw white knight
                    draw_square(self.board[((p.y-4)*8+p.x) as usize], ctx, &mut canvas, graphics::Color::new(0.0, 0.0, 0.0, 1.0));
                    draw_image(graphics::Rect::new((p.x) as f32, (7 - (p.y - 4)) as f32, 30.0, 30.0), "/white_knight_60.png".to_string(), ctx, &mut canvas);
                }
                _ => (),
            }
        }

        // Colors are reversed on promotion
        // Test for black
        if self.game.turn == Color::White {
            match self.game.game_state {
                GameState::AwaitingPromotion(p) => {
                    // Draw piece one step below promotion
                    // Draw black queen
                    draw_square(self.board[((p.y+1)*8+p.x) as usize], ctx, &mut canvas, graphics::Color::new(0.0, 0.0, 0.0, 1.0));
                    draw_image(graphics::Rect::new((p.x) as f32, (7 - (p.y + 1)) as f32, 30.0, 30.0), "/black_queen_60.png".to_string(), ctx, &mut canvas);
                    
                    // Draw black rook
                    draw_square(self.board[((p.y+2)*8+p.x) as usize], ctx, &mut canvas, graphics::Color::new(0.0, 0.0, 0.0, 1.0));
                    draw_image(graphics::Rect::new((p.x) as f32, (7 - (p.y + 2)) as f32, 30.0, 30.0), "/black_rook_60.png".to_string(), ctx, &mut canvas);

                    // Draw black bishop
                    draw_square(self.board[((p.y+3)*8+p.x) as usize], ctx, &mut canvas, graphics::Color::new(0.0, 0.0, 0.0, 1.0));
                    draw_image(graphics::Rect::new((p.x) as f32, (7 - (p.y + 3)) as f32, 30.0, 30.0), "/black_bishop_60.png".to_string(), ctx, &mut canvas);
                    
                    // Draw black knight
                    draw_square(self.board[((p.y+4)*8+p.x) as usize], ctx, &mut canvas, graphics::Color::new(0.0, 0.0, 0.0, 1.0));
                    draw_image(graphics::Rect::new((p.x) as f32, (7 - (p.y + 4)) as f32, 30.0, 30.0), "/black_knight_60.png".to_string(), ctx, &mut canvas);
                }
                _ => (),
            }
        }

        canvas.finish(ctx)?;

        Ok(())
    }


    fn mouse_button_down_event(&mut self,_ctx: &mut Context,_button: event::MouseButton,x: f32,y: f32,) -> Result<(), GameError> {
        let sq_x: u8 = (x / SQUARE_SIZE) as u8;
        let sq_y: u8 = 7 - (y / SQUARE_SIZE) as u8;

        match self.game.game_state {
            GameState::AwaitingPromotion(p) => {
                // Colors are reversed during promotion
                // Check for white
                if sq_x == p.x && self.game.turn != Color::White {
                    if sq_y == p.y - 1 { let _result: MoveResult = self.game.promote(PieceType::Queen); }
                    if sq_y == p.y - 2 { let _result: MoveResult = self.game.promote(PieceType::Rook); }
                    if sq_y == p.y - 3 { let _result: MoveResult = self.game.promote(PieceType::Bishop); }
                    if sq_y == p.y - 4 { let _result: MoveResult = self.game.promote(PieceType::Knight); }
                }
                // Check for black
                if sq_x == p.x && self.game.turn != Color::Black {
                    if sq_y == p.y + 1 { let _result: MoveResult = self.game.promote(PieceType::Queen); }
                    if sq_y == p.y + 2 { let _result: MoveResult = self.game.promote(PieceType::Rook); }
                    if sq_y == p.y + 3 { let _result: MoveResult = self.game.promote(PieceType::Bishop); }
                    if sq_y == p.y + 4 { let _result: MoveResult = self.game.promote(PieceType::Knight); }
                }
                return Ok(());
            }
            _ => (),
        }

        if !self.moves.is_empty() {
            // Move piece to a highlighted square
            if self.board_moves[(sq_y*8 + sq_x) as usize] {
                // Make Move
                let result: MoveResult = self.game.make_move(Position::new(self.from.x, self.from.y), Position::new(sq_x, sq_y)); 
                
                match result {
                    MoveResult::Allowed => println!("Allowed!"),
                    MoveResult::Disallowed => println!("Disallowed!"),
                }
                match self.game.game_state {
                    GameState::Normal => println!("Normal!"),
                    GameState::AwaitingPromotion(_p) => println!("Awaiting promotion!"),
                    _ => ()
                }

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

        // Get moves of piece
        if self.moves.is_empty(){
            // If piece doesnt exist or is of wrong color, dont show the moves of the piece
            let piece = self.game.get_square(Position::new(sq_x, sq_y));
            if piece.is_none() || piece.unwrap().color != self.game.turn {
                return Ok(());
            }

            self.moves = self.game.get_possible_moves(Position::new(sq_x, sq_y));
            self.from = Position::new(sq_x, sq_y);

            self.board_moves = vec![false; 64];
            for p in &self.moves {
                self.board_moves[(p.y*8 + p.x) as usize] = true;
            }

            return Ok(());
        }
        
        Ok(())
    }
}

fn main() {
    let game = Game::new();

    let mut state = State {
        game: game,
        board: vec![],
        pieces: vec![None; 64],
        moves: vec![],
        board_moves: vec![false; 64],
        from: Position::new(0,0)
    };

    for i in 0..64 {
        state.board.push(graphics::Rect::new(
            SQUARE_SIZE * (i % 8) as f32,
            SQUARE_SIZE * (7 - (i / 8)) as f32,
            SQUARE_SIZE,
            SQUARE_SIZE
        ));
    }

    let mut i = 0;
    for y in (0..8).rev() {
        for x in 0..8 {
            state.pieces[y*8+x] = state.game.squares[i];
            i += 1;
        }
    }

    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("davbjor-chess-gui", "davidbjorklund")
        .default_conf(c)
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
    
}
