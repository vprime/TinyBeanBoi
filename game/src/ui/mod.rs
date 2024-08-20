pub mod menu;

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
        let element_style = PrimitiveStyleBuilder::new().stroke_color(Rgb565::WHITE).stroke_width(1).fill_color(Rgb565::BLACK).build();
        let submenu_style = PrimitiveStyleBuilder::new().stroke_color(Rgb565::RED).stroke_width(1).fill_color(Rgb565::BLACK).build();
        let submenu_point = Point::new(0, (DISPLAY_HEIGHT - (MAIN_MENU_HEIGHT * 2)) as i32);
        let submenu_size = Size::new(DISPLAY_WIDTH - 15, MAIN_MENU_HEIGHT * 2);
        let vert_element_size = Size::new(DISPLAY_WIDTH - 15 - 6, 16);
        let horiz_element_size = Size::new(32, 32);
        let text_offset = Point::new( 3, 13);
        let padding = Size::new(3, 16);
        let element_space = Size::new(3, 3);
        let cursor_style = PrimitiveStyleBuilder::new().stroke_color(Rgb565::YELLOW).stroke_width(3).build();


        let menu_one = Menu {
            style: MenuStyle {
                point: submenu_point,
                size: submenu_size,
                style: submenu_style,
                text: Some("Menu One".into()),
                text_offset,
                padding,
                element_space,
                element_size: vert_element_size,
                direction: MenuDirection::Vertical,
            },
            redraw: false,
            has_focus: false,
            cursor_style,
            elements: [
                MenuElement {
                    style: MenuStyle {
                        size: vert_element_size,
                        style: element_style,
                        text_offset,
                        text: Some("First and only in menu one".into()),
                        ..MenuStyle::default()
                    },
                    trigger: Some(|| { println!("First item selected!"); }),
                    on_close: None,
                    takes_focus: false,
                    submenu: None,
                },

            ].into(),
            ..Menu::default()
        };
        let menu_two = Menu {
            redraw: false,
            has_focus: false,
            style: MenuStyle {
                point: submenu_point,
                size: submenu_size,
                style: submenu_style,
                text: Some("Menu Two".into()),
                text_offset,
                padding,
                element_space,
                element_size: vert_element_size,
                direction: MenuDirection::Vertical,
            },
            cursor_style,
            elements: [
                MenuElement {
                    style: MenuStyle {
                        size: vert_element_size,
                        style: element_style,
                        text_offset,
                        text: Some("First in menu two".into()),
                        ..MenuStyle::default()
                    },
                    trigger: Some(|| { println!("First item selected!"); }),
                    on_close: None,
                    takes_focus: false,
                    submenu: None,
                },
                MenuElement {
                    style: MenuStyle {
                        size: vert_element_size,
                        style: element_style,
                        text_offset,
                        text: Some("Second in menu two".into()),
                        ..MenuStyle::default()
                    },
                    trigger: Some(|| { println!("First item selected!"); }),
                    on_close: None,
                    takes_focus: false,
                    submenu: None,
                },

            ].into(),
            ..Menu::default()
        };
        let menu_three = Menu {
            redraw: false,
            has_focus: false,
            style: MenuStyle {
                point: submenu_point,
                size: submenu_size,
                style: submenu_style,
                text: Some("Menu Three".into()),
                text_offset,
                padding,
                element_space,
                element_size: vert_element_size,
                direction: MenuDirection::Vertical,
            },
            cursor_style,
            elements: [
                MenuElement {
                    style: MenuStyle {
                        size: vert_element_size,
                        style: element_style,
                        text_offset,
                        text: Some("You're beautiful".into()),
                        ..MenuStyle::default()
                    },
                    trigger: Some(|| { println!("First item selected!"); }),
                    on_close: None,
                    takes_focus: false,
                    submenu: None,
                },
                MenuElement {
                    style: MenuStyle {
                        size: vert_element_size,
                        style: element_style,
                        text_offset,
                        text: Some("You're really beautiful".into()),
                        ..MenuStyle::default()
                    },
                    trigger: Some(|| { println!("First item selected!"); }),
                    on_close: None,
                    takes_focus: false,
                    submenu: None,
                },
                MenuElement {
                    style: MenuStyle {
                        size: vert_element_size,
                        style: element_style,
                        text_offset,
                        text: Some("You're really really beautiful".into()),
                        ..MenuStyle::default()
                    },
                    trigger: Some(|| { println!("First item selected!"); }),
                    on_close: None,
                    takes_focus: false,
                    submenu: None,
                },
                MenuElement {
                    style: MenuStyle {
                        size: vert_element_size,
                        style: element_style,
                        text_offset,
                        text: Some("You're really really really beautiful".into()),
                        ..MenuStyle::default()
                    },
                    trigger: Some(|| { println!("First item selected!"); }),
                    on_close: None,
                    takes_focus: false,
                    submenu: None,
                },



            ].into(),
            ..Menu::default()
        };
        let menu_four = Menu {
            redraw: false,
            has_focus: false,
            style: MenuStyle {
                point: submenu_point,
                size: submenu_size,
                style: submenu_style,
                text: Some("Menu Four".into()),
                text_offset,
                padding,
                element_space,
                element_size: vert_element_size,
                direction: MenuDirection::Vertical,
            },
            cursor_style,
            elements: [
                MenuElement {
                    style: MenuStyle {
                        size: vert_element_size,
                        style: element_style,
                        text_offset,
                        text: Some("I want to have your babies".into()),
                        ..MenuStyle::default()
                    },
                    trigger: Some(|| { println!("First item selected!"); }),
                    on_close: None,
                    takes_focus: false,
                    submenu: None,
                },
                MenuElement {
                    style: MenuStyle {
                        size: vert_element_size,
                        style: element_style,
                        text_offset,
                        text: Some("But only figureatively".into()),
                        ..MenuStyle::default()
                    },
                    trigger: Some(|| { println!("First item selected!"); }),
                    on_close: None,
                    takes_focus: false,
                    submenu: None,
                },
                MenuElement {
                    style: MenuStyle {
                        size: vert_element_size,
                        style: element_style,
                        text_offset,
                        text: Some("Not literally".into()),
                        ..MenuStyle::default()
                    },
                    trigger: Some(|| { println!("First item selected!"); }),
                    on_close: None,
                    takes_focus: false,
                    submenu: None,
                },
                MenuElement {
                    style: MenuStyle {
                        size: vert_element_size,
                        style: element_style,
                        text_offset,
                        text: Some("I'm sorry".into()),
                        ..MenuStyle::default()
                    },
                    trigger: Some(|| { println!("First item selected!"); }),
                    on_close: None,
                    takes_focus: false,
                    submenu: None,
                },
                MenuElement {
                    style: MenuStyle {
                        size: vert_element_size,
                        style: element_style,
                        text_offset,
                        text: Some("I just don't think I'm ready for children yet.".into()),
                        ..MenuStyle::default()
                    },
                    trigger: Some(|| { println!("First item selected!"); }),
                    on_close: None,
                    takes_focus: false,
                    submenu: None,
                },

            ].into(),
            ..Menu::default()
        };

        Self{
            main_menu: Menu {
                cursor_style,
                style: MenuStyle {
                    point: Point::new(0, (DISPLAY_HEIGHT - MAIN_MENU_HEIGHT) as i32),
                    size: Size::new(DISPLAY_WIDTH, MAIN_MENU_HEIGHT),
                    style: PrimitiveStyleBuilder::new().stroke_color(Rgb565::RED).stroke_width(1).fill_color(Rgb565::BLACK).build(),
                    text: Some("Main".into()),
                    text_offset,
                    padding,
                    element_space,
                    element_size: horiz_element_size,
                    direction: MenuDirection::Horizontal,
                },
                elements: [
                    MenuElement {
                        style: MenuStyle {
                            size: horiz_element_size,
                            style: element_style,
                            text: Some("1".into()),
                            text_offset,
                            ..MenuStyle::default()
                        },
                        trigger: Some(|| { println!("First item selected!"); }),
                        on_close: None,
                        takes_focus: true,
                        submenu: Some(menu_one),
                    },
                    MenuElement {
                        style: MenuStyle {
                            size: horiz_element_size,
                            style: element_style,
                            text: Some("2".into()),
                            text_offset,
                            ..MenuStyle::default()
                        },
                        trigger: Some(|| { println!("Second item selected!"); }),
                        on_close: None,
                        takes_focus: true,
                        submenu: Some(menu_two),
                    },
                    MenuElement {
                        style: MenuStyle {
                            size: horiz_element_size,
                            style: element_style,
                            text: Some("3".into()),
                            text_offset,
                            ..MenuStyle::default()
                        },
                        trigger: Some(|| { println!("Third item selected!"); }),
                        on_close: None,
                        takes_focus: true,
                        submenu: Some(menu_three),
                    },
                    MenuElement {
                        style: MenuStyle {
                            size: horiz_element_size,
                            style: element_style,
                            text: Some("4".into()),
                            text_offset,
                            ..MenuStyle::default()
                        },
                        trigger: Some(|| { println!("Fourth item selected!"); }),
                        on_close: None,
                        takes_focus: true,
                        submenu: Some(menu_four),
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