use graphics::Drawable;
use viering_chess::*;
use ggez::*;

pub const SQUARE_SIZE: f32 = 100.;
pub const SIDEBAR_WIDTH: f32 = 400.;
pub const PADDING: f32 = 40.0;

pub const SCREEN_HEIGHT: f32 = SQUARE_SIZE * 8. + PADDING * 2.;
pub const SCREEN_WIDTH: f32 = SQUARE_SIZE * 8. + PADDING * 2. + SIDEBAR_WIDTH;


pub fn draw_image(rect: graphics::Rect, path: String, ctx: &mut Context, canvas: &mut graphics::Canvas) {
    let image = graphics::Image::from_path(ctx, path).expect("Error reading image");
    let dim = image.dimensions(ctx).unwrap_or(graphics::Rect::new(0.0,0.0,rect.w,rect.h));
    let dest = glam::Vec2{x: SQUARE_SIZE * (rect.x) as f32 + PADDING, y: SQUARE_SIZE * (rect.y) as f32 + PADDING};
    let scale = glam::Vec2{x: (SQUARE_SIZE / dim.w), y: (SQUARE_SIZE / dim.h)};

    canvas.draw(&image, graphics::DrawParam::default().dest(dest).scale(scale));
}

pub fn draw_square(rect: graphics::Rect, ctx: &mut Context, canvas: &mut graphics::Canvas, color: graphics::Color) {
    let square = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        rect,
        color,
    )
    .expect("error creating square");
    canvas.draw(&square, graphics::DrawParam::default());
}

pub fn draw_circle(rect: graphics::Rect, ctx: &mut Context, canvas: &mut graphics::Canvas, color: graphics::Color) {
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


pub fn draw_highlight(rect: graphics::Rect, ctx: &mut Context, canvas: &mut graphics::Canvas, color: graphics::Color) {
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
pub fn piece_to_path(piece: Option<Piece>) -> String {
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
pub fn write_text(input: String, tx: f32, ty: f32, canvas: &mut graphics::Canvas) {
    let mut text = graphics::Text::new(input);
    let pos = glam::Vec2{x: tx, y: ty};

    text.set_scale(graphics::PxScale::from(30.0));

    text.draw(canvas, graphics::DrawParam::default().dest(pos));
}


pub fn write_info(name_1_string: String, name_2_string: String, turn: Color, game_state: GameState, canvas: &mut graphics::Canvas) {
    let name_1 = name_1_string.as_str();
    let name_2 = name_2_string.as_str();
    let mut white_text = name_1.to_string();
    let mut black_text = name_2.to_string();

    if turn == Color::White { white_text =  format!("{}{}", name_1, " *"); }
    if turn == Color::Black { black_text = format!("{}{}", name_2, " *"); }
    
    if let GameState::Checkmate(Color::White) = game_state { black_text = format!("{}{}", name_2, " Won"); white_text = format!("{}{}", name_1, " -"); }
    if let GameState::Checkmate(Color::Black) = game_state { white_text = format!("{}{}", name_1, " Won"); black_text = format!("{}{}", name_2, " -");}

    if let GameState::Draw = game_state {
        white_text = format!("{}{}", name_1, " -");
        black_text = format!("{}{}", name_2, " -");
    }
    if let GameState::Check(Color::White) = game_state { white_text = format!("{}{}", name_1, " in check *"); }
    if let GameState::Check(Color::Black) = game_state { black_text = format!("{}{}", name_2, " in check *"); }

    write_text(
        white_text.to_string(), 
        PADDING * 2. + SQUARE_SIZE * 8.,
        PADDING * 1.5 + SQUARE_SIZE * 4.,
        canvas
    );
    write_text(
        black_text.to_string(), 
        PADDING * 2. + SQUARE_SIZE * 8.,
        PADDING * 0.5 + SQUARE_SIZE * 4.,
        canvas
    );

    // Restart
    write_text(
        "Restart Game".to_string(), 
        PADDING * 2. + SQUARE_SIZE * 8.,
        -PADDING * 0.5 + SQUARE_SIZE * 8.,
        canvas
    );
}