#![feature(collections, core, hash)]

extern crate image;

pub use packer::{Packer, GrowingPacker};
pub use shelf_packer::ShelfPacker;
pub use guillotine_packer::GuillotinePacker;
pub use maxrect_packer::MaxrectPacker;
pub use skyline_packer::SkylinePacker;
pub use rect::Rect;
pub use buffer2d::{Buffer2d, ResizeBuffer};

mod packer;
mod shelf_packer;
mod guillotine_packer;
mod maxrect_packer;
mod skyline_packer;
mod rect;
mod buffer2d;
