use crate::player::Player;
use crossbeam::channel::{bounded, Receiver, Sender, unbounded};
use rand::Rng;
use spin::RwLock;

use super::board::{ChessBoard, RoChessBoard};
use super::{board::Board, Color, Move};
use crate::{engine::ChessEngine, graphics::GraphicsProvider};
use std::sync::{Mutex, Arc};
use std::thread::spawn;

pub struct Game<'a> {
    board: ChessBoard,
    provider: GraphicsProvider<'a>,
    engine: Arc<RwLock<ChessEngine<'a>>>,
    sender: Sender<Move>,
    receiver: Receiver<Move>,
}

impl<'a> Game<'a> {
    pub fn new(board: ChessBoard, provider: GraphicsProvider<'a>) -> Self {
        let (sender, receiver) = unbounded::<Move>();
        Self {
            board,
            provider,
            sender,
            receiver,
            engine: Arc::new(RwLock::new(ChessEngine::new())),
        }
    }

    pub fn start<'b>(mut self) {
        let player_is_white = rand::thread_rng().gen::<bool>();
        let color = if player_is_white {
            Color::White
        } else {
            Color::Black
        };
        let engine_color = -color;

        // Lifetime tricks start here
        let (engine_sender, engine_receiver) = bounded::<(Arc<RwLock<ChessEngine<'a>>>, RoChessBoard)>(1);
        engine_sender.send((self.engine, self.board.read())).unwrap();

        let (provider_sender, provider_receiver) = bounded::<(GraphicsProvider<'a>, RoChessBoard)>(1);
        provider_sender.send((self.provider, self.board.read())).unwrap();

        spawn(move || {
            println!("Engine thread running...");
            let (engine, board) = engine_receiver.recv().unwrap();
            
        });

        // -- And end here

        Self::game_handler();
    }

    pub fn game_handler() {}
}