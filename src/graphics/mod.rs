mod egui;
use spin::Mutex;
use std::sync::Arc;

use crate::{
    chess::{board::Board, Color, Move},
    player::Player,
};

pub use self::egui::create_graphics_provider;

pub type GraphicsProvider<'a> = Arc<Mutex<(dyn GraphicsProviderImpl<'a> + Send + Sync + 'a)>>;

pub trait GraphicsProviderImpl<'a>: Player<'a> {
    fn refresh(&'a mut self);
}
