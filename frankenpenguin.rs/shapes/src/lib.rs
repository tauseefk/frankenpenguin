#[derive(Clone)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub dx: f64,
    pub dy: f64,
    pub height: f64,
    color: Color,
}

impl Rectangle {
    pub fn random(canvas_width: f64, canvas_height: f64) -> Self {
        let width = js_sys::Math::random() * 50.0 + 10.0;
        let height = js_sys::Math::random() * 50.0 + 10.0;
        let x = js_sys::Math::random() * (canvas_width - width);
        let y = js_sys::Math::random() * (canvas_height - height);
        let dx = (js_sys::Math::random() - 0.5) * 4.0; // Random velocity between -2 and 2
        let dy = (js_sys::Math::random() - 0.5) * 4.0; // Random velocity between -2 and 2
        let color = Color::random();

        Self {
            x,
            y,
            width,
            height,
            dx,
            dy,
            color,
        }
    }

    pub fn update(&mut self, canvas_width: f64, canvas_height: f64) {
        self.x += self.dx;
        self.y += self.dy;

        if self.x <= 0.0 || self.x + self.width >= canvas_width {
            self.dx = -self.dx;
        }
        if self.y <= 0.0 || self.y + self.height >= canvas_height {
            self.dy = -self.dy;
        }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn generate(count: usize, canvas_width: f64, canvas_height: f64) -> Vec<Rectangle> {
        (0..count)
            .map(|_| Self::random(canvas_width, canvas_height))
            .collect()
    }
}

#[derive(Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
    hex_string: String,
}

impl Color {
    pub fn random() -> Self {
        let r = js_sys::Math::random();
        let g = js_sys::Math::random();
        let b = js_sys::Math::random();
        let a = 1.0;

        let r_byte = (r * 255.0) as u8;
        let g_byte = (g * 255.0) as u8;
        let b_byte = (b * 255.0) as u8;
        let hex_string = format!("#{:02x}{:02x}{:02x}", r_byte, g_byte, b_byte);

        Self {
            r,
            g,
            b,
            a,
            hex_string,
        }
    }

    pub fn to_hex(&self) -> String {
        self.hex_string.clone()
    }
}
