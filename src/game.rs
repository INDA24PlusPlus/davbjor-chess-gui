use crate::draw::*;

use serde::{Deserialize, Serialize};
use rmp_serde;
use viering_chess::*;
use ggez::*;

use chess_networking;
use std::net::{TcpListener, TcpStream};

#[derive(Debug, PartialEq, Clone)]
enum ConnectionState {
    ClientStart,
    ServerStart,
    ClientMove,
    ServerMove,
    ClientAck,
    ServerAck
}


struct Connection {
    player_turn: bool,
    expected: ConnectionState,
    stream: Option<TcpStream>,
}

impl Connection {
    fn new() -> Self {
        Connection {
            player_turn: true,
            expected: ConnectionState::ClientStart,
            stream: None,
        }
    }
    fn next(&mut self) {
        match self.expected {
            ConnectionState::ClientStart => self.expected = ConnectionState::ClientMove,
            ConnectionState::ServerStart => self.expected = ConnectionState::ServerMove,
            ConnectionState::ClientMove => self.expected = ConnectionState::ClientAck,
            ConnectionState::ServerMove => self.expected = ConnectionState::ServerAck,
            ConnectionState::ClientAck => self.expected = ConnectionState::ClientMove,
            ConnectionState::ServerAck => self.expected = ConnectionState::ServerMove
        }
    }
    fn listen(&mut self) -> Result<(), String> {
        // Start server
        let listener = TcpListener::bind("127.0.0.1:5000").expect("Error listening on port");
        
        // accept connections and process them serially
        let (stream, _addr) = listener.accept().expect("Error accepting connection");
        //stream.set_nonblocking(true);

        self.stream = Some(stream);

        Ok(())
    }
    fn connect(&mut self) -> Result<(), String> {
        let mut stream = TcpStream::connect("127.0.0.1:5000").expect("Error connecting");
        self.stream = Some(stream);
        Ok(())
    }
    fn write_start(&mut self, start: chess_networking::Start) {
        if self.stream.is_some() {
            start.serialize(&mut rmp_serde::Serializer::new(self.stream.as_mut().expect("No stream to serialize"))).expect("Couldn't serialize");
            println!("Sent packet");
            return
        }
        print!("No stream");
    }
    fn write_move(&mut self, send_move: chess_networking::Move) {
        if self.stream.is_some() {
            send_move.serialize(&mut rmp_serde::Serializer::new(self.stream.as_mut().expect("No stream to serialize"))).expect("Couldn't serialize");
            println!("Sent packet");
            return;
        }
        print!("No stream");
    }
    fn read_start(&mut self) -> Option<chess_networking::Start> {
        if self.stream.is_none() { return None; }
        let mut stream = self.stream.as_mut().expect("Wierd error unwrapping stream");

        let mut des  = rmp_serde::Deserializer::new(&mut stream);
        let packet: chess_networking::Start = chess_networking::Start::deserialize(&mut des).expect("Could'nt read");

        println!("Recieved: {}", packet.name.as_deref().unwrap_or("No-name"));

        Some(packet)
    }
    fn read_move(&mut self) -> Option<chess_networking::Move> {
        if self.stream.is_none() { return None; }
        let mut stream = self.stream.as_mut().expect("Wierd error unwrapping stream");
        let _ = stream.set_nonblocking(true);

        let mut des  = rmp_serde::Deserializer::new(&mut stream);
        let packet: chess_networking::Move;
        match chess_networking::Move::deserialize(&mut des) {
            Ok(p) => packet = p,
            Err(e) => {
                return None;
            }   
        }
        println!("Recieved: {} {}", packet.from.0, packet.from.1);

        Some(packet)
    }
    fn write_ack(&mut self, send_ack: chess_networking::Ack) {
        if self.stream.is_some() {
            send_ack.serialize(&mut rmp_serde::Serializer::new(self.stream.as_mut().expect("No stream to serialize"))).expect("Couldn't serialize");
            println!("Sent Ack");
            return;
        }
        print!("No stream");
    }
    fn read_ack(&mut self) -> Option<chess_networking::Ack> {
        if self.stream.is_none() { return None; }
        let mut stream = self.stream.as_mut().expect("Wierd error unwrapping stream");
        let _ = stream.set_nonblocking(true);

        let mut des  = rmp_serde::Deserializer::new(&mut stream);
        let packet: chess_networking::Ack;
        match chess_networking::Ack::deserialize(&mut des) {
            Ok(p) => packet = p,
            Err(_e) => {
                return None;
            }   
        }

        println!("Recieved Ack: {}", packet.ok);

        Some(packet)
    }
}
/*
struct Server {
    player_turn: bool,
    expected: ConnectionState,
    stream: Option<TcpStream>,
}

impl Server {
    fn new() -> Self {
        Server {
            player_turn: true,
            expected: ConnectionState::ClientStart,
            stream: None,
        }
    }
    fn next(&mut self) {
        if self.player_turn { self.expected = ConnectionState::ClientAck; }
        else                { self.expected = ConnectionState::ClientMove; }
    }
    fn listen(&mut self) -> Result<(), String> {
        // Start server
        let listener = TcpListener::bind("127.0.0.1:5000").expect("Error listening on port");
        
        // accept connections and process them serially
        let (stream, _addr) = listener.accept().expect("Error accepting connection");
        //stream.set_nonblocking(true);

        self.stream = Some(stream);

        Ok(())
    }
    fn connect(&mut self) -> Result<(), String> {
        let mut stream = TcpStream::connect("127.0.0.1:5000").expect("Error connecting");
        self.stream = Some(stream);
        Ok(())
    }
    fn write_start(&mut self, start: chess_networking::Start) {
        if self.stream.is_some() {
            start.serialize(&mut rmp_serde::Serializer::new(self.stream.as_mut().expect("No stream to serialize"))).expect("Couldn't serialize");
            println!("Sen't packet");
        }
        else {
            print!("No stream");
        }
    }
    fn write_move(&mut self, send_move: chess_networking::Move) {
        if self.stream.is_some() {
            send_move.serialize(&mut rmp_serde::Serializer::new(self.stream.as_mut().expect("No stream to serialize"))).expect("Couldn't serialize");
            println!("Sen't packet");
        }
        else {
            print!("No stream");
        }
    }
    fn read_start(&mut self) -> Option<chess_networking::Start> {
        if self.stream.is_none() { return None; }
        let mut stream = self.stream.as_mut().expect("Wierd error unwrapping stream");

        let mut des  = rmp_serde::Deserializer::new(&mut stream);
        let packet: chess_networking::Start = chess_networking::Start::deserialize(&mut des).expect("Could'nt read");

        println!("Recieved: {}", packet.name.as_deref().unwrap_or("No-name"));

        Some(packet)
    }
    fn read_move(&mut self) -> Option<chess_networking::Move> {
        if self.stream.is_none() { return None; }
        let mut stream = self.stream.as_mut().expect("Wierd error unwrapping stream");
        let _ = stream.set_nonblocking(true);

        let mut des  = rmp_serde::Deserializer::new(&mut stream);
        let packet: chess_networking::Move;
        match chess_networking::Move::deserialize(&mut des) {
            Ok(p) => packet = p,
            Err(e) => {
                return None;
            /*if e.kind() == std::io::ErrorKind::WouldBlock {
                return None;
            }*/ 
            }   

        }
        println!("Recieved: {} {}", packet.from.0, packet.from.1);

        Some(packet)
    }
    fn write_ack(&mut self, send_ack: chess_networking::Ack) {
        if self.stream.is_some() {
            send_ack.serialize(&mut rmp_serde::Serializer::new(self.stream.as_mut().expect("No stream to serialize"))).expect("Couldn't serialize");
            println!("Sen't Acc");
        }
        else {
            print!("No stream");
        }
    }
    fn read_ack(&mut self) -> Option<chess_networking::Ack> {
        if self.stream.is_none() { return None; }
        let mut stream = self.stream.as_mut().expect("Wierd error unwrapping stream");
        let _ = stream.set_nonblocking(true);

        let mut des  = rmp_serde::Deserializer::new(&mut stream);
        let packet: chess_networking::Ack;
        match chess_networking::Ack::deserialize(&mut des) {
            Ok(p) => packet = p,
            Err(_e) => {
                return None;
            }   

        }

        println!("Recieved Ack: {}", packet.ok);

        Some(packet)
    }
}


struct Client {
    client_turn: bool,
    expected: ConnectionState,
    stream: Option<TcpStream>,
}

impl Client {
    fn new() -> Self {
        Client {
            client_turn: true,
            expected: ConnectionState::ClientStart,
            stream: None,
        }
    }
    fn next(&mut self) {
        if self.client_turn { self.expected = ConnectionState::ClientAck; }
        else                { self.expected = ConnectionState::ClientMove; }
    }
    fn connect(&mut self) -> Result<(), String> {
        let mut stream = TcpStream::connect("127.0.0.1:5000").expect("Error connecting");
        self.stream = Some(stream);
        Ok(())
    }
    fn write_start(&mut self, start: chess_networking::Start) {
        if self.stream.is_some() {
            start.serialize(&mut rmp_serde::Serializer::new(self.stream.as_mut().expect("No stream to serialize"))).expect("Couldn't serialize");
            println!("Sen't packet");
        }
        else {
            print!("No stream");
        }
    }
    fn write_move(&mut self, send_move: chess_networking::Move) {
        if self.stream.is_some() {
            send_move.serialize(&mut rmp_serde::Serializer::new(self.stream.as_mut().expect("No stream to serialize"))).expect("Couldn't serialize");
            println!("Sen't packet");
        }
        else {
            print!("No stream");
        }
    }
    fn read_start(&mut self) -> Option<chess_networking::Start> {
        if self.stream.is_none() { return None; }
        let mut stream = self.stream.as_mut().expect("Wierd error unwrapping stream");

        let mut des  = rmp_serde::Deserializer::new(&mut stream);
        let packet: chess_networking::Start = chess_networking::Start::deserialize(&mut des).expect("Could'nt read");

        println!("Recieved: {}", packet.name.as_deref().unwrap_or("No-name"));

        Some(packet)
    }
    fn read_move(&mut self) -> Option<chess_networking::Move> {
        if self.stream.is_none() { return None; }
        let mut stream = self.stream.as_mut().expect("Wierd error unwrapping stream");
        let _ = stream.set_nonblocking(true);

        let mut des  = rmp_serde::Deserializer::new(&mut stream);
        let packet: chess_networking::Move;
        match chess_networking::Move::deserialize(&mut des) {
            Ok(p) => packet = p,
            Err(_e) => {
                return None;
            /*if e.kind() == std::io::ErrorKind::WouldBlock {
                return None;
            }*/ 
            }   

        }

        println!("Recieved: {} {}", packet.from.0, packet.from.1);

        Some(packet)
    }
    fn write_ack(&mut self, send_ack: chess_networking::Ack) {
        if self.stream.is_some() {
            send_ack.serialize(&mut rmp_serde::Serializer::new(self.stream.as_mut().expect("No stream to serialize"))).expect("Couldn't serialize");
            println!("Sen't Acc");
        }
        else {
            print!("No stream");
        }
    }
    fn read_ack(&mut self) -> Option<chess_networking::Ack> {
        if self.stream.is_none() { return None; }
        let mut stream = self.stream.as_mut().expect("Wierd error unwrapping stream");
        let _ = stream.set_nonblocking(true);

        let mut des  = rmp_serde::Deserializer::new(&mut stream);
        let packet: chess_networking::Ack;
        match chess_networking::Ack::deserialize(&mut des) {
            Ok(p) => packet = p,
            Err(_e) => {
                return None;
            }   

        }

        println!("Recieved Ack: {}", packet.ok);

        Some(packet)
    }
}
*/

#[derive(Debug, PartialEq, Clone)]
pub enum Mode {
    Server,
    Client,
    Singleplayer
}

fn start_server(name_1:&mut String, name_2: &mut String) -> Connection {
    // Start a client
    let mut server = Connection {
        player_turn: true,
        expected: ConnectionState::ClientStart,
        stream: None
    };
    //let mut server = Server::new();
    // Connect to the server
    let _ = server.listen();

    // Read start parameters
    let data = server.read_start();
    server.next();
    // Store opponents name
    *name_2 = "White".to_string();
    *name_2 = data.expect("No data").name.unwrap_or("White".to_string());
    

    // Pass start parameters
    let start = chess_networking::Start {
        is_white: false,
        name: Some("Bjork".to_string()),
        fen: None,
        time: None,
        inc: None
    };
    // Store the name
    *name_1 = start.name.as_deref().unwrap_or("Black").to_string();
    server.write_start(start);

    server
}

fn start_client(name_1:&mut String, name_2:&mut String) -> Connection {
    // Start a client
    let mut client = Connection {
        player_turn: false,
        expected: ConnectionState::ServerStart,
        stream: None
    };
    // Connect to the server
    let _ = client.connect();
    

    // Pass start parameters
    let start = chess_networking::Start {
        is_white: true,
        name: Some("David".to_string()),
        fen: None,
        time: None,
        inc: None
    };
    // Store the name
    *name_2 = start.name.as_deref().unwrap_or("White").to_string();
    client.write_start(start);


    // Read start parameters
    let data = client.read_start();
    client.next();
    
    // Store opponents name
    *name_1 = "Black".to_string();
    *name_1 = data.expect("No data").name.unwrap_or("Black".to_string());


    client
}

#[derive(Clone)]
pub struct MainState {
    game: Game,
    board: Vec<graphics::Rect>,
    pieces: Vec<Option<Piece>>,
    moves: Vec<Position>,
    board_moves: Vec<bool>,
    from: Position,
    pub mode: Mode,
    pub name_1: String,
    pub name_2: String
}


impl MainState {
    pub fn new() -> Self {
        MainState {
            game: Game::new(),
            board: vec![],
            pieces: vec![None; 64],
            moves: vec![],
            board_moves: vec![false; 64],
            from: Position::new(0,0),
            mode: Mode::Singleplayer,
            name_1: "White".to_string(), 
            name_2: "Black".to_string()
        }
    }
    pub fn init(&mut self) {
        self.game = Game::new();
        self.board = vec![];
        self.pieces = vec![None; 64];
        self.moves = vec![];
        self.board_moves = vec![false; 64];
        self.from = Position::new(0,0);
        self.name_1 = "White".to_string(); 
        self.name_2 = "Black".to_string();
        
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

    pub fn run(&mut self) {
        self.init();

        let mut server_option = None;
        let mut client_option = None;
        match self.mode {
            Mode::Server => {
                server_option = Some(start_server(&mut self.name_1, &mut self.name_2));
            },
            Mode::Client => {
                client_option = Some(start_client(&mut self.name_1, &mut self.name_2));
            }
            _ => {

            }
        }

        let c = conf::Conf::new();
        let (ctx, event_loop) = ContextBuilder::new("davbjor-chess-gui", "davidbjorklund")
            .default_conf(c)
            .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
            .build()
            .unwrap();

        let state = State {
            game: self.game.clone(),
            board: self.board.clone(),
            pieces: self.pieces.clone(),
            moves: self.moves.clone(),
            board_moves: self.board_moves.clone(),
            from: self.from.clone(),
            mode: self.mode.clone(),
            name_1: self.name_1.clone(),
            name_2: self.name_2.clone(),
            server: server_option,
            client: client_option
        };

        event::run(ctx, event_loop, state);
    }
    
}

pub struct State {
    game: Game,
    board: Vec<graphics::Rect>,
    pieces: Vec<Option<Piece>>,
    moves: Vec<Position>,
    board_moves: Vec<bool>,
    from: Position,
    pub mode: Mode,
    pub name_1: String,
    pub name_2: String,
    server: Option<Connection>,
    client: Option<Connection>,
    
}

impl State {
    pub fn new() -> Self {
        State {
            game: Game::new(),
            board: vec![],
            pieces: vec![None; 64],
            moves: vec![],
            board_moves: vec![false; 64],
            from: Position::new(0,0),
            mode: Mode::Singleplayer,
            name_1: "White".to_string(), 
            name_2: "Black".to_string(),
            server: None,
            client: None
        }
    }
    fn init(&mut self) {
        let mut mainstate = MainState::new();
        mainstate.init();


        self.game = mainstate.game.clone();
        self.board = mainstate.board.clone();
        self.pieces = mainstate.pieces.clone();
        self.moves = mainstate.moves.clone();
        self.board_moves = mainstate.board_moves.clone();
        self.from = mainstate.from.clone();
        self.mode = mainstate.mode.clone();
        self.name_1 = mainstate.name_1.clone();
        self.name_2 = mainstate.name_2.clone();
    }
}


impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.mode == Mode::Client {
            // READ SERVER MOVE
            if self.client.as_mut().expect("No client to check turn of").expected == ConnectionState::ServerMove {
                let x = self.client.as_mut().expect("Couldn't unpack client").read_move();
                let mut is_ok = false;
                if x.is_some() {
                    let y = x.unwrap();
                    

                    let res = self.game.make_move(Position::new(y.from.0, y.from.1), Position::new(y.to.0, y.to.1));
                    if res == MoveResult::Allowed { is_ok = true; }
                    self.client.as_mut().expect("No client to check turn of").player_turn = true;

                    let send_ack = chess_networking::Ack { ok: is_ok, end_state: None };
                    self.client.as_mut().expect("No client to check turn of").write_ack(send_ack);
                }
            }

            // READ SERVER ACK
            if self.client.as_mut().expect("No client to check turn of").expected == ConnectionState::ServerAck {
                let x = self.client.as_mut().expect("Couldn't unpack client").read_ack();
                if x.is_some() {
                    let y = x.unwrap();

                    if !y.ok {
                        // Server did not accept servers move
                        println!("Server did not accept servers move");
                    }
                    //self.server.as_mut().expect("No server to check turn of").player_turn = false;
                    self.client.as_mut().expect("No client to check turn of").next();
                }
            }
        }

        if self.mode == Mode::Server {
            // READ CLIENT MOVE
            if self.server.as_mut().expect("No server to check turn of").expected == ConnectionState::ClientMove {
                let x = self.server.as_mut().expect("Couldn't unpack server").read_move();
                let mut is_ok = false;
                if x.is_some() {
                    let y = x.unwrap();
                    
                    let res = self.game.make_move(Position::new(y.from.0, y.from.1), Position::new(y.to.0, y.to.1));
                    if res == MoveResult::Allowed { is_ok = true; }
                    self.server.as_mut().expect("No server to check turn of").player_turn = true;
                    self.server.as_mut().expect("No server to check next of").next();

                    let send_ack = chess_networking::Ack { ok: is_ok, end_state: None };
                    self.server.as_mut().expect("No client to check turn of").write_ack(send_ack);
                }
            }

            // READ CLIENT ACK
            if self.server.as_mut().expect("No server to check turn of").expected == ConnectionState::ClientAck {
                let x = self.server.as_mut().expect("Couldn't unpack server").read_ack();
                if x.is_some() {
                    let y = x.unwrap();

                    if !y.ok {
                        // Client did not accept servers move
                        println!("Client did not accept servers move");
                    }
                    //self.server.as_mut().expect("No server to check turn of").player_turn = false;
                    self.server.as_mut().expect("No server to check next of").next();
                }
            }
        }

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
                    let piece: Piece = self.pieces[i].unwrap_or(Piece {color: Color::White, piece_type: PieceType::Pawn});

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
        write_info(self.name_1.clone(), self.name_2.clone(), self.game.turn, self.game.game_state, &mut canvas);


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

        // Pressed on the chessboard
        let sq_x: u8 = ((x - PADDING) / SQUARE_SIZE) as u8;
        let sq_y: u8 = 7 - ((y - PADDING) / SQUARE_SIZE) as u8;
        
        // Handle promotion
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

        if self.mode == Mode::Server {
            if self.server.is_none() || self.server.as_mut().expect("No server").player_turn == false {
                return Ok(());
            }
        }
        
        if self.mode == Mode::Client {
            if self.client.is_none() || self.client.as_mut().expect("No client").player_turn == false {
                return Ok(());
            }
        }

        // When possible moves are shown
        if !self.moves.is_empty() {
            // Move piece to a highlighted square
            if self.board_moves[(sq_y*8 + sq_x) as usize] {
                let send_move = chess_networking::Move {
                    from: (self.from.x, self.from.y),
                    to: (sq_x, sq_y),
                    promotion: None,
                    forfeit: false,
                    offer_draw: false
                };

                match self.mode {
                    Mode::Server => {  
                        // Try To Make Move
                        let result: MoveResult = self.game.make_move(
                            Position::new(self.from.x, self.from.y),
                            Position::new(sq_x, sq_y)
                        ); 

                        if MoveResult::Allowed == result {
                            // Send move to client;

                            self.server.as_mut().expect("No server to send moves to").write_move(send_move);
                            self.server.as_mut().expect("No server to send next to").next();
                            self.server.as_mut().expect("No server to check turn of").player_turn = false;
                        }

                    },
                    Mode::Client => {  
                        // Try To Make Move
                        let result: MoveResult = self.game.clone().make_move(
                            Position::new(self.from.x, self.from.y),
                            Position::new(sq_x, sq_y)
                        );

                        if MoveResult::Allowed == result {
                            // Send move to server
                            // Check if move is allowed
                            self.client.as_mut().expect("No server to send moves to").write_move(send_move);

                            // Do move locally aswell
                            let _: MoveResult = self.game.make_move(
                                Position::new(self.from.x, self.from.y),
                                Position::new(sq_x, sq_y)
                            );
                            self.client.as_mut().expect("No server to check turn of").player_turn = false;
                            self.client.as_mut().expect("No server to send moves to").next();
                        }

                    },
                    Mode::Singleplayer => {    
                        // Make Move
                        let _result: MoveResult = self.game.make_move(
                            Position::new(self.from.x, self.from.y),
                            Position::new(sq_x, sq_y)
                        ); 
                    },
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
