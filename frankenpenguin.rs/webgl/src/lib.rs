use animation::{AnimationController, Renderer};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlProgram, WebGlShader};

// Vertex shader program - simplified to match TypeScript
const VERTEX_SHADER: &str = r#"
    attribute vec2 a_position;
    attribute vec4 a_color;
    varying vec4 v_color;
    void main() {
        gl_Position = vec4(a_position, 0.0, 1.0);
        v_color = a_color;
    }
"#;

// Fragment shader program - simplified to match TypeScript
const FRAGMENT_SHADER: &str = r#"
    precision mediump float;
    varying vec4 v_color;
    void main() {
        gl_FragColor = v_color;
    }
"#;

struct WebGlRenderer {
    gl: GL,
    _program: WebGlProgram,
    animation_controller: AnimationController,
    position_buffer: web_sys::WebGlBuffer,
    color_buffer: web_sys::WebGlBuffer,
    // Pre-allocated arrays to avoid repeated allocation
    positions: Vec<f32>,
    colors: Vec<f32>,
    // Cached attribute locations to avoid repeated lookups
    position_location: u32,
    color_location: u32,
}

impl WebGlRenderer {
    fn new(canvas: HtmlCanvasElement, num_rectangles: usize) -> Result<Self, JsValue> {
        // Initialize WebGL context
        let gl = canvas
            .get_context("webgl2")?
            .ok_or("Failed to get WebGL2 context")?
            .dyn_into::<GL>()?;

        // Create shader program
        let program = compile_program(&gl, VERTEX_SHADER, FRAGMENT_SHADER)?;
        gl.use_program(Some(&program));

        // Create buffers
        let position_buffer = gl
            .create_buffer()
            .ok_or("Failed to create position buffer")?;
        let color_buffer = gl.create_buffer().ok_or("Failed to create color buffer")?;

        // Pre-allocate arrays: 6 vertices per rectangle, 2 components per vertex
        let positions = vec![0.0; num_rectangles * 12];
        // Pre-allocate colors: 6 vertices per rectangle, 4 components per color
        let colors = vec![0.0; num_rectangles * 24];

        // Create animation controller
        let animation_controller = AnimationController::new(
            num_rectangles,
            canvas.width() as f64,
            canvas.height() as f64,
        )?;

        // Cache attribute locations
        let position_location = gl.get_attrib_location(&program, "a_position") as u32;
        let color_location = gl.get_attrib_location(&program, "a_color") as u32;

        Ok(WebGlRenderer {
            gl,
            _program: program,
            animation_controller,
            position_buffer,
            color_buffer,
            positions,
            colors,
            position_location,
            color_location,
        })
    }

    fn clear_canvas(&self) {
        let gl = &self.gl;
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT);
    }

    fn update_position_buffer(&self) {
        let gl = &self.gl;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.position_buffer));
        unsafe {
            // Use view to create a view of the Rust data without copying
            let positions_view = js_sys::Float32Array::view(&self.positions);
            gl.buffer_data_with_array_buffer_view(
                GL::ARRAY_BUFFER,
                &positions_view,
                GL::DYNAMIC_DRAW,
            );
        }
    }

    fn update_color_buffer(&self) {
        let gl = &self.gl;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.color_buffer));
        unsafe {
            // Use view to create a view of the Rust data without copying
            let colors_view = js_sys::Float32Array::view(&self.colors);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &colors_view, GL::DYNAMIC_DRAW);
        }
    }

    fn setup_attributes(&self) {
        let gl = &self.gl;

        // Enable and bind position attribute
        gl.enable_vertex_attrib_array(self.position_location);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.position_buffer));
        gl.vertex_attrib_pointer_with_i32(self.position_location, 2, GL::FLOAT, false, 0, 0);

        // Enable and bind color attribute
        gl.enable_vertex_attrib_array(self.color_location);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.color_buffer));
        gl.vertex_attrib_pointer_with_i32(self.color_location, 4, GL::FLOAT, false, 0, 0);
    }

    fn draw_rectangles(&self) {
        let gl = &self.gl;
        let rectangles = self.animation_controller.rectangles();
        gl.draw_arrays(GL::TRIANGLES, 0, (rectangles.len() * 6) as i32);
    }
}

impl Renderer for WebGlRenderer {
    fn update(&mut self) {
        self.animation_controller.update();

        let rectangles = self.animation_controller.rectangles();
        let canvas_width = self.animation_controller.canvas_width;
        let canvas_height = self.animation_controller.canvas_height;

        // Pre-calculate scale factors to avoid repeated divisions
        let scale_x = 2.0 / canvas_width as f32;
        let scale_y = 2.0 / canvas_height as f32;

        // Update position buffer - convert to clip space like TypeScript
        for (i, rect) in rectangles.iter().enumerate() {
            let base_index = i * 12;

            // Convert from pixel coordinates to clip space (-1 to 1) like TypeScript
            let x1 = rect.x as f32 * scale_x - 1.0;
            let y1 = -(rect.y as f32 * scale_y - 1.0);
            let x2 = (rect.x as f32 + rect.width as f32) * scale_x - 1.0;
            let y2 = -((rect.y as f32 + rect.height as f32) * scale_y - 1.0);

            // First triangle
            self.positions[base_index] = x1;
            self.positions[base_index + 1] = y1;
            self.positions[base_index + 2] = x2;
            self.positions[base_index + 3] = y1;
            self.positions[base_index + 4] = x1;
            self.positions[base_index + 5] = y2;

            // Second triangle
            self.positions[base_index + 6] = x1;
            self.positions[base_index + 7] = y2;
            self.positions[base_index + 8] = x2;
            self.positions[base_index + 9] = y1;
            self.positions[base_index + 10] = x2;
            self.positions[base_index + 11] = y2;
        }

        // Update color buffer - set same color for all vertices of each rectangle
        for (i, rect) in rectangles.iter().enumerate() {
            let base_index = i * 24;
            let color = rect.color();
            let r = color.r as f32;
            let g = color.g as f32;
            let b = color.b as f32;
            let a = color.a as f32;

            // Set the same color for all 6 vertices of the rectangle
            for j in 0..6 {
                let vertex_index = base_index + j * 4;
                self.colors[vertex_index] = r;
                self.colors[vertex_index + 1] = g;
                self.colors[vertex_index + 2] = b;
                self.colors[vertex_index + 3] = a;
            }
        }
    }

    fn render(&self) {
        self.clear_canvas();
        self.update_position_buffer();
        self.update_color_buffer();
        self.setup_attributes();
        self.draw_rectangles();
    }
}

fn get_num_rectangles_from_url() -> Result<usize, JsValue> {
    let window = web_sys::window().ok_or("No window found")?;
    let location = window.location();
    let search = location
        .search()
        .map_err(|_| "Failed to get URL search params")?;

    // Parse URL parameters
    let url_params = web_sys::UrlSearchParams::new_with_str(&search)
        .map_err(|_| "Failed to parse URL search params")?;

    let rectangles_param = url_params
        .get("rectangles")
        .ok_or("Missing required URL parameter: rectangles")?;

    let num_rectangles = rectangles_param
        .parse::<usize>()
        .map_err(|_| "Invalid rectangles parameter: must be a positive integer")?;

    if num_rectangles == 0 {
        return Err("Invalid rectangles parameter: must be greater than 0".into());
    }

    Ok(num_rectangles)
}

fn compile_program(
    gl: &GL,
    vertex_source: &str,
    fragment_source: &str,
) -> Result<WebGlProgram, String> {
    let vertex_shader = compile_shader(gl, GL::VERTEX_SHADER, vertex_source)?;
    let fragment_shader = compile_shader(gl, GL::FRAGMENT_SHADER, fragment_source)?;

    let program = gl
        .create_program()
        .ok_or("Unable to create shader program")?;
    gl.attach_shader(&program, &vertex_shader);
    gl.attach_shader(&program, &fragment_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or("Unable to create shader")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

#[wasm_bindgen]
pub struct Frankenpenguin {
    renderer: WebGlRenderer,
}

#[wasm_bindgen]
impl Frankenpenguin {
    pub fn new() -> Result<Self, JsValue> {
        console_error_panic_hook::set_once();

        // Get number of rectangles from URL parameter
        let num_rectangles = get_num_rectangles_from_url()?;

        let window = web_sys::window().ok_or("No window found")?;
        let document = window.document().ok_or("No document found")?;

        // Get or create the canvas element
        let canvas: HtmlCanvasElement = {
            let canvas = document.create_element("canvas")?;
            canvas.set_id("canvas");

            // Set canvas size to window size
            let window_width = window.inner_width()?.as_f64().unwrap_or(800.0) as u32;
            let window_height = window.inner_height()?.as_f64().unwrap_or(600.0) as u32;

            let canvas_style = format!(
                "display: block; width: {}px; height: {}px; background: #000;",
                window_width, window_height
            );
            canvas.set_attribute("style", &canvas_style)?;

            // Set the canvas dimensions
            let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;
            canvas.set_width(window_width);
            canvas.set_height(window_height);

            // Add canvas to document body
            let body = document.body().ok_or("No body found")?;
            body.append_child(&canvas)?;

            canvas
        };

        let renderer = WebGlRenderer::new(canvas, num_rectangles)?;

        Ok(Frankenpenguin { renderer })
    }

    pub fn tick(&mut self) {
        self.renderer.update();
        self.renderer.render();
    }
}
