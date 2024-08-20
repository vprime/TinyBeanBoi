mod menu;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{RgbColor, Size};
use embedded_graphics::primitives::PrimitiveStyleBuilder;
use menu::*;
use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::input::Input;

pub struct GameUi {
    main_menu: Menu,
}

const MAIN_MENU_HEIGHT: u32 = 50;

impl GameUi {
    pub fn new() -> Self {
        Self{
            main_menu: Menu {
                style: MenuStyle {
                    point: Point::new(0, (DISPLAY_HEIGHT - MAIN_MENU_HEIGHT) as i32),
                    size: Size::new(DISPLAY_WIDTH, MAIN_MENU_HEIGHT),
                    style: PrimitiveStyleBuilder::new().stroke_color(Rgb565::RED).stroke_width(1).fill_color(Rgb565::BLACK).build(),
                    text: Some("Main".into()),
                    text_offset: Point::new( 3, 13),
                    padding: Size::new(3, 16),
                    element_space: Size::new(3, 3),
                    element_size: Size::new(32, 32),
                    direction: MenuDirection::Horizontal,
                },
                elements: [
                    MenuElement {
                        style: MenuStyle {
                            size: Size::new(32, 32),
                            style: PrimitiveStyleBuilder::new().stroke_color(Rgb565::WHITE).stroke_width(1).fill_color(Rgb565::BLACK).build(),
                            text: Some("1".into()),
                            text_offset: Point::new( 3, 13),
                            ..MenuStyle::default()
                        },
                        trigger: Some(|| { println!("First item selected!"); }),
                    },
                    MenuElement {
                        style: MenuStyle {
                            size: Size::new(32, 32),
                            style: PrimitiveStyleBuilder::new().stroke_color(Rgb565::WHITE).stroke_width(1).fill_color(Rgb565::BLACK).build(),
                            text: Some("2".into()),
                            text_offset: Point::new( 3, 13),
                            ..MenuStyle::default()
                        },
                        trigger: Some(|| { println!("Second item selected!"); }),
                    },
                    MenuElement {
                        style: MenuStyle {
                            size: Size::new(32, 32),
                            style: PrimitiveStyleBuilder::new().stroke_color(Rgb565::WHITE).stroke_width(1).fill_color(Rgb565::BLACK).build(),
                            text: Some("3".into()),
                            text_offset: Point::new( 3, 13),
                            ..MenuStyle::default()
                        },
                        trigger: Some(|| { println!("Third item selected!"); }),
                    },
                    MenuElement {
                        style: MenuStyle {
                            size: Size::new(32, 32),
                            style: PrimitiveStyleBuilder::new().stroke_color(Rgb565::WHITE).stroke_width(1).fill_color(Rgb565::BLACK).build(),
                            text: Some("4".into()),
                            text_offset: Point::new( 3, 13),
                            ..MenuStyle::default()
                        },
                        trigger: Some(|| { println!("Fourth item selected!"); }),
                    },
                ].into(),
                ..Menu::default()
            }
        }
    }

    pub fn update<T: DrawTarget<Color = Rgb565>>(&mut self, display: &mut T, input: &Input) {
        // Update the main menu
        self.main_menu.update(display, input);

    }
}