use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::Drawable;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{Point, RgbColor, Size};
use embedded_graphics::primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StyledDrawable};
use embedded_graphics::text::{Text, TextStyle};
use crate::DISPLAY_WIDTH;
use crate::input::Input;

pub struct Menu {
    /// Style
    pub style: MenuStyle,
    pub cursor_style: PrimitiveStyle<Rgb565>,

    /// Elements
    pub elements: Vec<MenuElement>,

    /// Active state
    pub cursor_index: usize,
    pub visible_elements: u8,
    pub redraw: bool,
    pub has_focus: bool,

    /// Events
    pub on_close: Option<fn()>,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            style: MenuStyle::default(),
            cursor_style: Default::default(),
            elements: Vec::new(),
            cursor_index: 0,
            visible_elements: 3,
            redraw: true,
            has_focus: true,
            on_close: None,
        }
    }
}

impl Menu {

    /// Update the active menu
    pub fn update<T: DrawTarget<Color = Rgb565>>(&mut self, display: &mut T, input: &Input) {
        // Handle input updates on the menu
        self.handle_input(input);

        // Draw the menu
        if self.redraw {
            self.draw(display);
        }

        // Handle Submenu
        self.handle_submenu(display, input);
    }

    fn handle_input(&mut self, input: &Input) {
        // Check for focus
        if !self.has_focus {
            return;
        }
        // If a button is tapped, we go the direction
        if input.right.click() {
            self.next_item();
        } else if input.left.click() {
            self.previous_item();
        } else if input.right.long_press() {
            // On a long right press, we select
            self.select_item();
        } else if input.left.long_press() {
            // On a long left press, we close the menu
            self.close_menu();
        }
    }

    fn handle_submenu<T: DrawTarget<Color = Rgb565>>(&mut self, display: &mut T, input: &Input) {
        if let Some(active_element) = self.elements.get_mut(self.cursor_index) {
            if let Some(submenu) = active_element.submenu.as_mut(){
                submenu.update(display, input);
                let active_submenu = active_element.takes_focus && submenu.has_focus;
                if active_submenu && self.has_focus {
                    self.has_focus = false;
                } else if !active_submenu && !self.has_focus {
                    self.has_focus = true;
                    self.redraw = true;
                }
            }
        }
    }

    /// Draw the current menu
    fn draw<T: DrawTarget<Color = Rgb565>>(&mut self, display: &mut T) {
        // Draw the frame & background
        Rectangle::new(self.style.point, self.style.size).draw_styled(&self.style.style, display);

        // Draw container text
        if let Some(content) = self.style.text.as_ref() {
            let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
            Text::new(content, self.style.point + self.style.text_offset, style).draw(display);
        }

        if self.elements.len() == 0 {
            self.redraw = false;
            return;
        }

        let mut element_position = self.style.point.clone();
        element_position += self.style.padding;

        //
        // Dynamic scrolling list way

        // Need to adjust the position of the drawn elements, to align with a cursor in the middle.
        //let available_area = self.style.size - self.style.padding;
        //let center = self.style.point + self.style.padding + (available_area/2);
        let edge_count = self.visible_elements / 2;

        for i in 0..self.visible_elements {
            // Determine our active item based on the cursor and current index
            let mut real_index = self.cursor_index as i32 + (i as i32 - edge_count as i32);
            if real_index < 0 {
                real_index += self.elements.len() as i32;
            }
            if real_index >= self.elements.len() as i32 {
                real_index -= self.elements.len() as i32;
            }
            let active_index = real_index as usize;
            // Get the element from the array
            if let Some(element) = self.elements.get(active_index){
                element.draw(display, element_position);

                // If this is the cursor then draw the cursor on it
                if active_index == self.cursor_index {
                    Rectangle::new(element_position, element.style.size).draw_styled(&self.cursor_style, display);
                }

                // Update the element position based on iteration
                match self.style.direction {
                    MenuDirection::Vertical => element_position.y += (element.style.size.height + self.style.element_space.height) as i32,
                    MenuDirection::Horizontal => {element_position.x += (element.style.size.width + self.style.element_space.width) as i32}
                }
            }
        }


        //
        // Static list way:
        //
        // Draw the visible elements
        // for element in self.elements.iter() {
        //     element.draw(display, element_position);
        //
        //     match self.style.direction {
        //         MenuDirection::Vertical => element_position.y += (element.style.size.height + self.style.element_space.height) as i32,
        //         MenuDirection::Horizontal => {element_position.x += (element.style.size.width + self.style.element_space.width) as i32}
        //     }
        // }

        // Draw the cursor

        // Mark the menu as clean
        self.redraw = false;
    }

    /// Select the next item
    fn next_item(&mut self) {
        self.cursor_index += 1;
        // If the cursor is past the element list, wrap back around
        if self.cursor_index >= self.elements.len() {
            self.cursor_index = 0;
        }
        // Mark menu as dirty
        self.redraw = true;
    }

    /// Select previous item
    fn previous_item(&mut self) {
        // If the cursor is past the element list, wrap back around
        if self.cursor_index == 0 {
            self.cursor_index = self.elements.len() - 1;
        } else {
            self.cursor_index -= 1;
        }
        // Mark menu as dirty
        self.redraw = true;
    }

    /// Select the current item under the cursor
    fn select_item(&mut self) {
        if let Some(item) = self.elements.get_mut(self.cursor_index) {
            item.select();
        }
    }

    /// Handle closing the menu
    fn close_menu(&mut self) {
        if let Some(exit) = self.on_close {
            exit();
        }
        self.has_focus = false;
    }
}

/// Item within a menu
pub struct MenuElement {
    /// Style
    pub style: MenuStyle,

    /// Triggered event
    pub trigger: Option<fn()>,
    pub on_close: Option<fn()>,

    /// Submenu content
    pub takes_focus: bool,
    pub submenu: Option<Menu>,
}

impl Default for MenuElement {
    fn default() -> Self {
        Self {
            style: Default::default(),
            trigger: None,
            on_close: None,
            takes_focus: false,
            submenu: None,
        }
    }
}


impl MenuElement {
    /// Draw this item
    pub fn draw<T: DrawTarget<Color = Rgb565>>(&self, display: &mut T, point: Point) {
        // Draw the frame & background
        Rectangle::new(point + self.style.point, self.style.size).draw_styled(&self.style.style, display);

        // Draw container text
        if let Some(content) = self.style.text.as_ref() {
            let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
            Text::new(content, point + self.style.point + self.style.text_offset, style).draw(display);
        }
    }

    /// Select this item
    pub fn select(&mut self) {
        if let Some(trigger) = self.trigger {
            trigger();
        }
        if let Some(submenu) = self.submenu.as_mut() {
            submenu.redraw = true;
            submenu.has_focus = true;
        }
    }
}

pub struct MenuStyle {
    pub point: Point,
    pub size: Size,
    pub style: PrimitiveStyle<Rgb565>,
    pub text: Option<String>,
    pub text_offset: Point,
    pub padding: Size,
    pub element_space: Size,
    pub element_size: Size,
    pub direction: MenuDirection,
}

impl Default for MenuStyle {
    fn default() -> Self {
        Self {
            point: Point::zero(),
            size: Size::new(40, 40),
            style: Default::default(),
            text: None,
            text_offset: Point::new(0, 10),
            padding: Default::default(),
            element_space: Default::default(),
            element_size: Default::default(),
            direction: MenuDirection::Vertical,
        }
    }
}

pub enum MenuDirection {
    Vertical,
    Horizontal,
}

