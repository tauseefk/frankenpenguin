use shapes::Rectangle;
use wasm_bindgen::prelude::*;

pub struct AnimationController {
    pub rectangles: Vec<Rectangle>,
    pub canvas_width: f64,
    pub canvas_height: f64,
}

impl AnimationController {
    pub fn new(
        num_rectangles: usize,
        canvas_width: f64,
        canvas_height: f64,
    ) -> Result<AnimationController, JsValue> {
        // Generate rectangles
        let rectangles = Rectangle::generate(num_rectangles, canvas_width, canvas_height);

        Ok(AnimationController {
            rectangles,
            canvas_width,
            canvas_height,
        })
    }

    pub fn update(&mut self) {
        // Update rectangle positions
        for rect in &mut self.rectangles {
            rect.update(self.canvas_width, self.canvas_height);
        }
    }

    pub fn rectangles(&self) -> &[Rectangle] {
        &self.rectangles
    }

    pub fn set_canvas_size(&mut self, width: f64, height: f64) {
        self.canvas_width = width;
        self.canvas_height = height;
    }
}

// Trait that renderers should implement
pub trait Renderer {
    fn update(&mut self);
    fn render(&self);
}
