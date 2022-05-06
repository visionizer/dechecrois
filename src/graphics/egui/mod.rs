use std::sync::Arc;

use eframe::{run_native, NativeOptions};
use egui::CentralPanel;
use spin::Mutex;

use crate::{
    chess::{
        board::{Board, RoChessBoard},
        board_builder::BoardBuilder,
        Color, Move,
    },
    player::Player,
};

use super::{GraphicsProvider, GraphicsProviderImpl};

pub struct EGuiGraphicsProvider<'a> {
    title: String,
    my_move: bool,
    my_color: Color,
    board: Option<RoChessBoard<'a>>,
}

impl<'a> EGuiGraphicsProvider<'a> {
    pub fn new() -> Self {
        Self {
            title: "ChEngine".to_owned(),
            my_move: false,
            my_color: Color::Black,
            board: None,
        }
    }
}

impl<'a> eframe::App for EGuiGraphicsProvider<'a> {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, World!");
        });
    }
}

impl<'a> Player<'a> for EGuiGraphicsProvider<'a> {
    fn init(&mut self, board: RoChessBoard<'a>, color: Color) {
        println!("Initializing EGui");
        self.board = Some(board);

        let options = NativeOptions::default();
        run_native(
            "ChEngine",
            options,
            Box::new(|_| Box::new(EGuiGraphicsProvider::new())),
        )
    }

    fn apply_move(&mut self, moved: &crate::chess::Move) {
        println!("Got move {}", moved.fmt(self.board.as_ref().unwrap()));
    }

    fn send_move(&mut self) -> crate::chess::Move {
        Move::RequestDraw
    }

    fn name(&self) -> &str {
        "<User>"
    }
}

impl<'a> GraphicsProviderImpl<'a> for EGuiGraphicsProvider<'a> {
    fn refresh(&mut self) {}
}

pub fn create_graphics_provider<'a>() -> GraphicsProvider<'a> {
    Arc::new(Mutex::new(EGuiGraphicsProvider::new()))
}
