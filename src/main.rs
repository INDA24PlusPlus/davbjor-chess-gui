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
    moves: Vec<Position>,
    from: Position,
    white_king: Position,
    black_king: Position,
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

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //event::process_event(ctx, event);
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for i in 0..64 {
            let mut move_here = false;
            for p in &self.moves {
                // Look if position is a possible move
                if i / 8 == (p.y as usize) && i % 8 == (p.x as usize) {
                    // Red Square
                    draw_square(self.board[i], ctx, &mut canvas, graphics::Color::new(1.0, 0.0, 0.0, 1.0));

                    move_here = true;
                }
            }
            // White King is in check
            if self.game.game_state == GameState::Check(Color::White) {
                if i / 8 == (self.white_king.y as usize) && i % 8 == (self.white_king.x as usize) {
                    // Red Square
                    draw_square(self.board[i], ctx, &mut canvas, graphics::Color::new(1.0, 0.0, 0.0, 1.0));

                    move_here = true;
                }
            }
            // Black King is in check
            if self.game.game_state == GameState::Check(Color::Black) {
                if i / 8 == (self.black_king.y as usize) && i % 8 == (self.black_king.x as usize) {
                    // Red Square
                    draw_square(self.board[i], ctx, &mut canvas, graphics::Color::new(1.0, 0.0, 0.0, 1.0));

                    move_here = true;
                }
            }

            // Drew red square
            if move_here { continue; }


            if i / 8 % 2 == 0 && i % 2 == 0 || i / 8 % 2 == 1 && i % 2 == 1{
                // White Square
                draw_square(self.board[i], ctx, &mut canvas, graphics::Color::new(1.0, 1.0, 1.0, 1.0));
                continue;
            }
            // Black Square
            draw_square(self.board[i], ctx, &mut canvas, graphics::Color::new(0.0, 0.0, 0.0, 1.0));
        }

        for i in 0..64 {
            if self.game.squares[i].is_some() {
                let mut path = String::new();
                match self.game.squares[i].unwrap() {
                    Piece {color: Color::White, piece_type: PieceType::Pawn} => path = "/white_pawn_60.png".to_string(),
                    Piece {color: Color::White, piece_type: PieceType::Knight} => path = "/white_knight_60.png".to_string(),
                    Piece {color: Color::White, piece_type: PieceType::Bishop} => path = "/white_bishop_60.png".to_string(),
                    Piece {color: Color::White, piece_type: PieceType::Rook} => path = "/white_rook_60.png".to_string(),
                    Piece {color: Color::White, piece_type: PieceType::Queen} => path = "/white_queen_60.png".to_string(),
                    Piece {color: Color::White, piece_type: PieceType::King} => path = "/white_king_60.png".to_string(),
                    Piece {color: Color::Black, piece_type: PieceType::Pawn} => path = "/black_pawn_60.png".to_string(),
                    Piece {color: Color::Black, piece_type: PieceType::Knight} => path = "/black_knight_60.png".to_string(),
                    Piece {color: Color::Black, piece_type: PieceType::Bishop} => path = "/black_bishop_60.png".to_string(),
                    Piece {color: Color::Black, piece_type: PieceType::Rook} => path = "/black_rook_60.png".to_string(),
                    Piece {color: Color::Black, piece_type: PieceType::Queen} => path = "/black_queen_60.png".to_string(),
                    Piece {color: Color::Black, piece_type: PieceType::King} => path = "/black_king_60.png".to_string(),
                    _ => (),
                }
                if path == "" { continue; }
                let image = graphics::Image::from_path(ctx, path)?;
                let dim = image.dimensions(ctx).unwrap_or(graphics::Rect::new(0.0,0.0,30.0,30.0));
                let dest = glam::Vec2{x: SQUARE_SIZE * (i % 8) as f32, y: SQUARE_SIZE * (i / 8) as f32};
                let scale = glam::Vec2{x: (SQUARE_SIZE / dim.w), y: (SQUARE_SIZE / dim.h)};

                canvas.draw(&image, graphics::DrawParam::default().dest(dest).scale(scale));
            }
        }

        canvas.finish(ctx)?;

        Ok(())
    }


    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), GameError> {
        let sq_x: u8 = (x / SQUARE_SIZE) as u8;
        let sq_y: u8 = (y / SQUARE_SIZE) as u8;

        match self.game.game_state {
            GameState::AwaitingPromotion(p) => {
                let _result: MoveResult = self.game.promote(PieceType::Queen); 
                return Ok(())
            }
            _ => (),
        } 

        // Get moves of piece
        if self.moves.is_empty(){
            println!("{} {}", sq_x, sq_y);

            let piece = self.game.get_square(Position::new(sq_x, 7-sq_y));
            // If piece doesnt exist or is of wrong color, dont show the moves of the piece
            if piece.is_none() || piece.unwrap().color != self.game.turn {
                return Ok(());
            }

            self.moves = self.game.get_possible_moves(Position::new(sq_x, 7-sq_y));
            self.from = Position::new(sq_x, 7-sq_y);
            return Ok(());
        }
        // Move piece to a highlighted square
        for p in &self.moves {
            if p.x == sq_x && p.y == 7-sq_y {
                // Make Move
                let result: MoveResult = self.game.make_move(Position::new(self.from.x, self.from.y), Position::new(sq_x, 7-sq_y)); 
                match result {
                    MoveResult::Allowed => println!("Allowed!"),
                    MoveResult::Disallowed => println!("Disallowed!"),
                }
                match self.game.game_state {
                    GameState::Normal => println!("Normal!"),
                    GameState::AwaitingPromotion(_p) => println!("Awaiting promotion!"),
                    _ => ()
                }
                self.moves = vec![];

                for i in 0..64 {
                    let piece_option = self.game.squares[i];
                    if piece_option.is_some() {
                        match piece_option.unwrap() {
                            Piece {color: Color::White, piece_type: PieceType::King} => self.white_king = Position::new((i%8) as u8, (7-i/8) as u8),
                            Piece {color: Color::Black, piece_type: PieceType::King} => self.black_king = Position::new((i%8) as u8, (7-i/8) as u8),
                            _ => ()
                        }
                    }
                }

                return Ok(());
            }
        }
        // Clear moves of piece
        self.moves = vec![];
        Ok(())
    }
}

fn main() {
    let mut game = Game::new();
    //let moves: Vec<Position> = game.get_possible_moves(Position::new(1, 3));

    //let result: MoveResult = game.make_move(Position::new(1, 1), Position::new(1, 3)); 

    /*
    loop {
        if game.turn == Color::White {
            println!("Whites Turn: ");
        }
        else {
            println!("Blacks Turn: ");
        }

        if game.game_state == GameState::Draw || game.game_state == GameState::Checkmate(Color::White) || game.game_state == GameState::Checkmate(Color::Black) {
            break;
        }
    }*/
    
    let mut state = State {
        game: game,
        board: vec![],
        moves: vec![],
        from: Position::new(0,0),
        white_king: Position::new(0,0),
        black_king: Position::new(0,0),
    };

    for i in 0..64 {
        state.board.push(graphics::Rect::new(
            SQUARE_SIZE * (i % 8) as f32,
            SQUARE_SIZE * (7 - (i / 8)) as f32,
            SQUARE_SIZE,
            SQUARE_SIZE
        ));
    }

    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("davbjor-chess-gui", "davidbjorklund")
        .default_conf(c)
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
    
}
