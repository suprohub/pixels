#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::sync::Arc;

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: i16 = 64;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

#[derive(Default)]
pub struct App<'win> {
    window: Option<Arc<Window>>,
    world: World,
    pixels: Option<Pixels<'win>>
}

impl<'win> ApplicationHandler for App<'win> {
    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: WindowEvent,
        ) {
        match event {
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = self.pixels.as_mut() {
                    self.world.draw(pixels.frame_mut());
                    if let Err(err) = pixels.render() {
                        log_error("pixels.render", err);
                        event_loop.exit();
                        return;
                    }
                }
            },
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::Resized(size) => {
                if let Some(pixels) = self.pixels.as_mut() {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        log_error("pixels.resize_surface", err);
                        event_loop.exit();
                        return;
                    }
                }
            },
            _ => ()
        }
    }

    fn device_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            device_id: winit::event::DeviceId,
            event: winit::event::DeviceEvent,
        ) {
        
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        
    }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Ok(window) = event_loop.create_window(Window::default_attributes()) {
            let window = Arc::new(window);
            self.window = Some(window.clone());
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window.clone());
            if let Ok(pixels) = Pixels::new(WIDTH, HEIGHT, surface_texture) {
                self.pixels = Some(pixels)
            }
        }
    }

}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
    Ok(())
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let inside_the_box = x >= self.box_x
                && x < self.box_x + BOX_SIZE
                && y >= self.box_y
                && y < self.box_y + BOX_SIZE;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}


impl Default for World {
    fn default() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
        }
    }
}