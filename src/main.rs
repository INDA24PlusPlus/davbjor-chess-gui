use graphics::Drawable;
use viering_chess::*;
use ggez::*;

const SQUARE_SIZE: f32 = 100.;

const SCREEN_HEIGHT: f32 = SQUARE_SIZE * 8.;
const SCREEN_WIDTH: f32 = SQUARE_SIZE * 8.;



struct State {
    game: Game,
    board: Vec<graphics::Rect>
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
            if i / 8 % 2 == 0 && i % 2 == 0 || i / 8 % 2 == 1 && i % 2 == 1{
                let white_square = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    self.board[i],
                    graphics::Color::new(1.0, 1.0, 1.0, 1.0),
                )
                .expect("error creating ball mesh");
                //graphics::Mesh::from_data(ctx, white_square.build()).expect("White Square Error");
                canvas.draw(&white_square, graphics::DrawParam::default());
                continue;
            }
            let black_square = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                self.board[i],
                graphics::Color::new(0.0, 0.0, 0.0, 1.0),
            )
            .expect("error creating ball mesh");
            canvas.draw(&black_square, graphics::DrawParam::default());
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
