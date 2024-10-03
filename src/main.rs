mod game;
mod draw;

use crate::game::{MainState, Mode};

use std::env;

/*
TODO:

Set path for resources to be in src/, not in target/debug/

Handle promotion better

Make it prettier

*/

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut settings = "".to_string();

    if args.len() >= 2 {
        settings = args[1].clone();
    }

    let mut state = MainState::new();

    match settings.as_str() {
        "server" => {
            state.mode = Mode::Server;
        },
        "client" => {
            state.mode = Mode::Client;
        },
        _ => {
            state.mode = Mode::Singleplayer;
        }
    }
    state.run();
    
    /*
    let game = Game::new();
    //game.load_fen("4n1bk/7p/8/8/8/2B5/8/4K3 b - - 0 1");
    //game.load_fen("8/2p2b2/1pk5/3Q4/2P5/8/8/3K4 b - - 0 1");
    
    //
    
    // Err(ref e) i e.kind() == std:io::ErrorKind::WouldBlock => msg_received = false
    
    let server = false;
    
    if server {
        // Start server
        let listener = TcpListener::bind("127.0.0.1:5000")?;
        
        // accept connections and process them serially
        let (mut stream, _addr) = listener.accept()?;
        //stream.set_nonblocking(true);
        
        
        let mut des  = rmp_serde::Deserializer::new(&mut stream);
        let packet: chess_networking::Start = chess_networking::Start::deserialize(&mut des).expect("Could'nt read");

        println!("Recieved: {}", packet.name.unwrap_or("No-name".to_string()));
        }
        
    else {
        let mut stream = TcpStream::connect("127.0.0.1:5000")?;
        //stream.set_nonblocking(true);
        
        let start = chess_networking::Start {
            is_white: true,
            name: Some("David".to_string()),
            fen: None,
            time: None,
            inc: None
        };
        /*
        let mut data = Vec::new();
        let mut buf: [u8; 4096] = [0; 4096];
        loop {
            let n = stream.read(&mut buf)?;
            if n == 0 {
                break;
            }
            data.extend_from_slice(&buf[..n]);
        }*/

        start.serialize(&mut rmp_serde::Serializer::new(&stream)).expect("Couldn't serialize");
        println!("Sen't packet");

        //start.serialize(rmp_serde::Serializer::new());
        //let buf = vec![start.try_into().unwrap_or(0)].as_slice();
        //stream.write(&buf)?;

    }
    */
    /*
    let mut state = State {
        game: game,
        board: vec![],
        pieces: vec![None; 64],
        moves: vec![],
        board_moves: vec![false; 64],
        from: Position::new(0,0)
    };

    state.init();
    //state.game.load_fen("4n1bk/7p/8/8/8/2B5/8/4K3 b - - 0 1");

    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("davbjor-chess-gui", "davidbjorklund")
        .default_conf(c)
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);*/

    Ok(())
}
