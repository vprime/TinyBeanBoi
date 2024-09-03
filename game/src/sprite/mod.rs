use embedded_graphics::geometry::Size;
use embedded_graphics::image::Image;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use tinybmp::Bmp;

pub struct Assets<'a> {
    pub sprites: SpriteAtlas<'a>,
}

pub struct SpriteAtlas<'a> {
    pub image: Bmp<'a, Rgb565>,
    pub subimage_size: Size,
}

pub struct Sprite {
    pub atlas_index: u32,
}

pub struct SpriteMap {
    pub vec: Vec<SpriteMapTile>,
}

pub struct SpriteMapTile {
    pub sprite: Sprite,
    pub point: Point,
}

pub struct Animation {
    pub vec: Vec<SpriteMap>,
}

impl<'a> SpriteAtlas<'a> {
    pub fn draw_sprite<T: DrawTarget<Color = Rgb565>>(&self, display: &mut T, position: Point, index: u32) {
        let rect = self.get_rect_from_index(index);
        let _ = Image::new(&self.image.sub_image(&rect), position).draw(display);
    }

    pub fn get_rect_from_index(&self, index: u32) -> Rectangle {
        let width = self.image.size().width / self.subimage_size.width;
        let height = self.image.size().height / self.subimage_size.height;
        let tile_y = (index as f32 / height as f32).floor() as u32;
        let tile_x = index % width;
        Rectangle::new(
            Point::new(
                (tile_x * self.subimage_size.width) as i32,
                (tile_y * self.subimage_size.height) as i32
            ),
            self.subimage_size
        )
    }
}

impl<'a> Default for Assets<'a> {
    fn default() -> Self {
        Self {
            sprites: SpriteAtlas {
                image: Bmp::from_slice(include_bytes!("../../../assets/sprites.bmp")).unwrap(),
                subimage_size: Size::new(16, 16),
            }
        }
    }
}