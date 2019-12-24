use std::{
    time::Duration,
    thread::sleep
};

use winit::{
    event::{
        Event, WindowEvent, ElementState,
        KeyboardInput as KeyboardInputEvent,
        MouseScrollDelta, Touch as TouchEvent
    },
    event_loop::{ControlFlow, EventLoop},
    window::{Icon, WindowBuilder, Window},
};

use glutin::{
    ContextBuilder,
    ContextWrapper,
    PossiblyCurrent,
    GlRequest, Api,
    GlProfile,
    PixelFormat,
    dpi::PhysicalSize,
};

use crate::sgl::{GL, HasContext, Context as GlContext};

use super::{Key, EventHandler, ViewConfig};

pub struct Platform {
    event_loop: EventLoop<()>,
    state: State,
}

struct State {
    dot_ratio: f64,
    view_size: PhysicalSize,

    gl_context: Option<ContextWrapper<PossiblyCurrent, Window>>,
    pixel_format: PixelFormat,
    gl_size: (i32, i32),
    gl: GlContext,

    visible: bool,
}

impl State {
    fn new(event_loop: &EventLoop<()>) -> Self {
        let icon = Icon::from_rgba(
            include_bytes!("../icon.rgba")
                .as_ref().into(),
            64, 64
        ).ok();

        let window_builder = WindowBuilder::new()
            .with_title("Pianino")
            .with_visible(true)
            .with_decorations(true)
            .with_resizable(true)
            .with_window_icon(icon);

        let gl_context = ContextBuilder::new()
            .with_gl(/*GlRequest::GlThenGles {
                opengl_version: (2, 0),
                opengles_version: (2, 0)
            }*/GlRequest::Specific(Api::OpenGlEs, (2, 0)))
            //.with_vsync(true)
            //.with_gl_profile(GlProfile::Core)
            //.with_pixel_format(24, 8)
            //.with_stencil_buffer(8)
            //.with_depth_buffer(0)
            //.with_double_buffer(Some(true))
            //.with_multisampling(0)
            .build_windowed(window_builder, &event_loop)
            .unwrap();

        let (dot_ratio, view_size, gl_size) = {
            let window = gl_context.window();
            let dot_ratio = window.hidpi_factor();
            let view_size = window.inner_size().to_physical(dot_ratio);
            let gl_size = (view_size.width as i32, view_size.height as i32);
            (dot_ratio, view_size, gl_size)
        };

        let gl_context = unsafe { gl_context.make_current().unwrap() };

        let gl = glow::Context::from_loader_function(
            |proc_name| gl_context.get_proc_address(proc_name)
        );

        let gl_api = match gl_context.get_api() {
            Api::OpenGl => "OpenGL",
            Api::OpenGlEs => "OpenGLES",
            Api::WebGl => "WebGL",
        };

        let gl_version = unsafe { (
            gl.get_parameter_i32(GL::MAJOR_VERSION),
            gl.get_parameter_i32(GL::MINOR_VERSION),
        ) };

        println!("API: {} {}.{}", gl_api, gl_version.0, gl_version.1);

        let gl_vendor = unsafe { gl.get_parameter_string(GL::VENDOR) };
        let gl_renderer = unsafe { gl.get_parameter_string(GL::RENDERER) };
        let gl_version = unsafe { gl.get_parameter_string(GL::VERSION) };

        println!("API VENDOR: {}", gl_vendor);
        println!("API RENDERER: {}", gl_renderer);
        println!("API VERSION: {}", gl_version);

        let pixel_format = gl_context.get_pixel_format();

        println!("Pixel format: {:?}", pixel_format);

        unsafe {
            gl.clear_color(0.5, 0.5, 0.5, 1.0);
            gl.clear_stencil(0);
            gl.stencil_mask(0xffffffff);
        }

        let gl_context = Some(gl_context);

        Self {
            dot_ratio,
            view_size,

            gl_context,
            pixel_format,
            gl_size,
            gl,

            visible: true,
        }
    }

    fn resize(&mut self) {
        self.gl_size = (self.view_size.width as i32,
                        self.view_size.height as i32);
        self.gl_context.as_ref().unwrap().resize(self.view_size);
        unsafe { self.gl.viewport(0, 0, self.gl_size.0, self.gl_size.1); }
    }

    fn view_config(&self) -> ViewConfig {
        let pixel_format = &self.pixel_format;

        ViewConfig::new(
            pixel_format.color_bits,
            pixel_format.alpha_bits,
            pixel_format.stencil_bits,
            pixel_format.depth_bits,
            pixel_format.srgb,
            pixel_format.multisampling
                .map(|samples| samples as u8)
                .unwrap_or(1),
            self.view_size.width as f32,
            self.view_size.height as f32,
            self.dot_ratio as f32,
        )
    }

    fn handle<H: EventHandler<Context = GlContext>>(&mut self, event: Event<()>, control_flow: &mut ControlFlow, handler: &mut H) {
        match event {
            Event::EventsCleared => {
                // Application update code.

                // Queue a RedrawRequested event.
                if self.visible {
                    self.gl_context.as_ref().unwrap().window().request_redraw();
                }
            },
            Event::LoopDestroyed => {
                handler.destroy();
            },
            Event::Suspended => {
                handler.suspend();
                self.visible = false;
            },
            Event::Resumed => {
                self.visible = true;
                self.gl_context = self.gl_context.take().map(|gl_context| unsafe { gl_context.treat_as_not_current().make_current().unwrap() });
                handler.resume();
            },
            Event::WindowEvent { event, .. } => {
                use self::WindowEvent::*;
                match event {
                    RedrawRequested => {
                        // Redraw the application.
                        //
                        // It's preferrable to render in this event rather than in EventsCleared, since
                        // rendering in here allows the program to gracefully handle redraws requested
                        // by the OS.

                        if self.visible {
                            handler.redraw(&self.gl);
                            self.gl_context.as_ref().unwrap().swap_buffers().unwrap();
                        }
                    },
                    CloseRequested => {
                        println!("The close button was pressed; stopping");
                        *control_flow = ControlFlow::Exit;
                    },
                    Resized(inner_size) => {
                        if self.visible && inner_size.width > 0.0 && inner_size.height > 0.0 {
                            self.view_size = inner_size.to_physical(self.dot_ratio);
                            //sleep(Duration::from_millis(250));
                            self.resize();
                            handler.reconf(self.view_config(), &self.gl);
                        }
                    },
                    HiDpiFactorChanged(dot_ratio) => {
                        if self.visible {
                            let size = self.gl_context.as_ref().unwrap().window().inner_size();
                            self.view_size = size.to_physical(dot_ratio);
                            self.dot_ratio = dot_ratio;
                            //sleep(Duration::from_millis(250));
                            self.resize();
                            handler.reconf(self.view_config(), &self.gl);
                        }
                    },
                    ReceivedCharacter(uchar) => {
                        handler.input(uchar);
                    },
                    KeyboardInput { input: KeyboardInputEvent { state, virtual_keycode, .. }, .. } => {
                        if let Some(keycode) = virtual_keycode {
                            if keycode == Key::Escape {
                                println!("The close button was pressed; stopping");
                                *control_flow = ControlFlow::Exit;
                            } else {
                                handler.key(keycode, state == ElementState::Pressed);
                            }
                        }
                    },
                    CursorMoved { position, .. } => {
                        handler.pointer(position.x as f32, position.y as f32, self.dot_ratio as f32);
                    },
                    MouseInput { button, state, .. } => {
                        handler.button(button, state == ElementState::Pressed);
                    },
                    MouseWheel { delta, .. } => {
                        use self::MouseScrollDelta::*;
                        let (dx, dy, dot) = match delta {
                            LineDelta(dx, dy) => (dx, dy, false),
                            PixelDelta(pos) => (pos.x as f32, pos.y as f32, true),
                        };
                        handler.scroll(dx, dy, dot);
                    },
                    Touch (TouchEvent { location, phase, .. }) => {
                        handler.touch(location.x as f32, location.y as f32, phase);
                    },
                    CursorEntered { .. } => {
                        handler.hover(true);
                    },
                    CursorLeft { .. } => {
                        handler.hover(false);
                    },
                    Focused(state) => {
                        handler.focus(state);
                    },
                    HoveredFile(path) => {
                        handler.file_over(path);
                    },
                    HoveredFileCancelled => {
                        handler.file_out();
                    },
                    DroppedFile(path) => {
                        handler.file_drop(path);
                    },
                    _ => (),
                }
            },
            // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
            // dispatched any events. This is ideal for games and similar applications.
            //_ => *control_flow = ControlFlow::Poll,
            // ControlFlow::Wait pauses the event loop if no events are available to process.
            // This is ideal for non-game applications that only update in response to user
            // input, and uses significantly less power/CPU time than ControlFlow::Poll.
            _ => *control_flow = ControlFlow::Wait,
        }
    }
}

impl Platform {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let state = State::new(&event_loop);

        Self {
            event_loop,
            state,
        }
    }

    pub fn run<H: EventHandler<Context = GlContext> + 'static>(self, mut handler: H) {
        let Platform { event_loop, mut state } = self;

        state.resize();
        handler.reconf(state.view_config(), &state.gl);

        event_loop.run(move |event, _, control_flow| {
            state.handle(event, control_flow, &mut handler);
        });
    }
}
