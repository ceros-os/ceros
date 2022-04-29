// Basic library for interacting with the vex v5 display.


use alloc::{vec::Vec, boxed::Box};

use crate::{runtime::mutex::Mutex, println};

use super::util::get_display;

const BRAIN_SCREEN_WIDTH: i32 = 480;
const BRAIN_SCREEN_HEIGHT: i32 = 240;


/// A touch event
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum TouchEvent {
    Release,
    Press,
    AutoPress,
}


/// Trait that defines objects that can be displayed
pub trait DisplayElement {
    /// Draws the shape, assuming the display is already locked
    fn draw(&self);

    /// Returns true if the given point intersects the shape
    fn intersects(&self, x: i32, y: i32) -> bool;

    /// Runs when a touch event happens over this element
    fn touch(&mut self, event: TouchEvent, x: i32, y: i32);
}

/// A shape that can be drawn
pub enum Shape {
    Rectangle {x1: i32, y1: i32, x2: i32, y2: i32, color: u32, fill: bool},
    Circle {cx: i32, cy: i32, r: i32, color: u32, fill: bool}
}


impl Shape {

    // Sets the color of a shape
    pub fn set_color(&mut self, new_color: u32) {
        match self {
            Shape::Rectangle {x1, y1, x2, y2, color, fill} => {
                *color = new_color;
            },
            Shape::Circle {cx, cy, r, color, fill} => {
                *color = new_color;
            }
        }
    }

    // Sets the fill of a shape
    pub fn set_fill(&mut self, new_fill: bool) {
        match self {
            Shape::Rectangle {x1, y1, x2, y2, color, fill} => {
                *fill = new_fill;
            },
            Shape::Circle {cx, cy, r, color, fill} => {
                *fill = new_fill;
            }
        }
    }
}

/// A drawable element
pub struct Element {
    pub shapes: Vec<Shape>,
}


impl DisplayElement for Element {

    /// Draws the shape
    fn draw(&self) {
        for shape in &self.shapes {
            match *shape {
                Shape::Rectangle { x1, y1, x2, y2, color, fill} => {
                    // Draw it using the v5 api
                    if fill {
                        unsafe {
                            vexv5rt::vexDisplayForegroundColor(color);
                            vexv5rt::vexDisplayRectFill(x1, y1, x2, y2);
                        }
                    } else {
                        unsafe {
                            vexv5rt::vexDisplayForegroundColor(color);
                            vexv5rt::vexDisplayRectDraw(x1, y1, x2, y2);
                        }
                    }
                },
                Shape::Circle { cx, cy, r, color, fill} => {
                    // Draw it using the v5 api
                    if fill {
                        unsafe {
                            vexv5rt::vexDisplayForegroundColor(color);
                            vexv5rt::vexDisplayCircleFill(cx, cy, r);
                        }
                    } else {
                        unsafe {
                            vexv5rt::vexDisplayForegroundColor(color);
                            vexv5rt::vexDisplayCircleDraw(cx, cy, r);
                        }
                    }
                }
            };
        }
    }

    fn intersects(&self, x: i32, y: i32) -> bool {
        for shape in &self.shapes {
            if match *shape {
                Shape::Rectangle { x1, y1, x2, y2 , ..}  => {
                    (x2 > x && x > x1) && (y2 > y && y > y1)
                },
                Shape::Circle { cx, cy, r , ..} => {
                    ((x-cx)*(x-cx) + (y-cy)*(y-cy)) <= r*r
                }
            } {
                return true;
            }
        }
        false
    }

    fn touch(&mut self, event: TouchEvent, x: i32, y: i32) {
        match event {
            TouchEvent::Press => {
                self.shapes[1].set_color(0xff00ff);
            },
            TouchEvent::Release => {
                self.shapes[1].set_color(0xffffff);
            },
            _ => {}
        }
    }
}

/// A Structure for interacting with the v5 brain display
pub struct Display {
    elements: Mutex<Vec<Box<dyn DisplayElement>>>,
    draw_lock: Mutex<()>
}

impl Display {

    /// Add a component to the display
    pub fn add(&mut self, element: Box<dyn DisplayElement>) {
        // Lock the mutex
        let mut list = self.elements.acquire();

        // Add the elements
        list.push(element);
    }

    /// Creates a new display object
    pub fn new() -> Display {
        Display {
            elements: Mutex::new(Vec::new()),
            draw_lock: Mutex::new(())
        }
    }

    /// Initializes the display, adding it to the global singleton
    pub fn init(&self) {
        unsafe {
            // Set the global runtime
            super::DISPLAY = self as *const Display;

            // Setup the touch callback
            vexv5rt::vexTouchUserCallbackSet(Some(touch_callback));
        }
    }

    /// Clears the screen
    pub fn clear_screen(&self) {
        
        // Clear the screen
        unsafe {
            vexv5rt::vexDisplayErase();
        }
        
    }

    /// Clears all elements
    pub fn clear_elements(&self) {
        // Lock the elements
        let mut elements = self.elements.acquire();

        // Clear the elements
        elements.clear();
    }

    /// Clears the screen and all elements
    pub fn clear(&self) {
        self.clear_elements();
        self.clear_screen();
    }

    

    /// Draws a frame of the display
    pub fn draw(&self) {

        // Acquire a lock on the elements
        let elements = self.elements.acquire();

        // Acquire a lock on drawing
        let _mtx = self.draw_lock.acquire();

        // Iterate over elements, drawing each
        for element in elements.iter() {
            // Draw the element
            element.draw();
        }

        unsafe {
            vexv5rt::vexDisplayRender(true, false);
        }
    }


    /// Should be called when a touch event is recieved
    pub fn on_touch(&self, event: TouchEvent, x: i32, y: i32) {

        let mut elements = self.elements.acquire();

        // Find which element it intersects with
        for element in elements.iter_mut() {
            // If the element intersects, call it's touch function and then break
            if element.intersects(x, y) {
                element.touch(event, x, y);
                break;
            } 
        }
    }
    
}



/// The global touch callback. This will call the on_touch event on display.
unsafe extern "C" fn touch_callback(event: u32, x: i32, y: i32) {
    // Get the display
    let disp = get_display();

    // Run the touch callback
    disp.on_touch(match event {
        0 => TouchEvent::Release,
        1 => TouchEvent::Press,
        2 => TouchEvent::AutoPress,
        _ => {
            return;
        }
    }, x, y)
}