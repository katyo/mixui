use winit::{
    event::{
        Event, WindowEvent, ElementState,
        KeyboardInput as KeyboardInputEvent,
        MouseScrollDelta, Touch as TouchEvent,
    },
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};

use glutin::{
    ContextBuilder,
    ContextWrapper,
    ContextError,
    NotCurrent,
    PossiblyCurrent,
    GlRequest, Api,
    //GlProfile,
    PixelFormat,
    dpi::PhysicalSize,
};

use sgl::{GL, HasContext, Context as GlContext};

use super::{Key, EventHandler, ViewConfig, AppConfig};

struct View {
    dot_ratio: f64,
    view_size: PhysicalSize<u32>,

    gl_context: Option<ContextWrapper<PossiblyCurrent, Window>>,
    pixel_format: PixelFormat,
    gl_size: (i32, i32),
    gl: GlContext,
}

impl View {
    fn try_init(gl_context: ContextWrapper<NotCurrent, Window>) -> Result<Self, (ContextWrapper<NotCurrent, Window>, ContextError)> {
        let gl_context = unsafe { gl_context.make_current()? };

        let gl = GlContext::from_loader_function(
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

        let (dot_ratio, view_size, gl_size) = {
            let window = gl_context.window();
            let dot_ratio = window.scale_factor();
            let view_size = window.inner_size();
            let gl_size = (view_size.width as i32, view_size.height as i32);
            (dot_ratio, view_size, gl_size)
        };

        println!("View scale: {} size: {:?}", dot_ratio, gl_size);

        let gl_context = Some(gl_context);

        Ok(Self {
            dot_ratio,
            view_size,

            gl_context,
            pixel_format,
            gl_size,
            gl,
        })
    }

    fn teardown(mut self) -> ContextWrapper<NotCurrent, Window> {
        unsafe { self.gl_context.take().unwrap().treat_as_not_current() }
    }

    fn resize(&mut self) {
        self.gl_size = (self.view_size.width as i32,
                        self.view_size.height as i32);
        self.gl_context.as_ref().unwrap().resize(self.view_size);
        unsafe { self.gl.viewport(0, 0, self.gl_size.0, self.gl_size.1); }
    }

    fn request_redraw(&self) {
        self.gl_context.as_ref().unwrap().window().request_redraw();
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

    fn redraw<H>(&mut self, handler: &mut H)
    where
        H: EventHandler<Context = GlContext>
    {
        handler.redraw(&self.gl);
        self.gl_context.as_ref().unwrap().swap_buffers().unwrap();
    }

    fn handle<H: EventHandler<Context = GlContext>>(&mut self, event: WindowEvent, control_flow: &mut ControlFlow, handler: &mut H) {
        use self::WindowEvent::*;
        match event {
            CloseRequested => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit;
            },
            Resized(inner_size) => {
                self.view_size = inner_size;
                self.resize();
                handler.reconf(self.view_config(), &self.gl);
            },
            ScaleFactorChanged { scale_factor, new_inner_size } => {
                self.view_size = *new_inner_size;
                self.dot_ratio = scale_factor;
                self.resize();
                handler.reconf(self.view_config(), &self.gl);
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
    }
}

pub struct Platform {
    event_loop: EventLoop<()>,
    gl_context: ContextWrapper<NotCurrent, Window>,
}

impl Platform {
    pub fn new(config: AppConfig) -> Self {
        let event_loop = EventLoop::new();

        let window_builder = WindowBuilder::new()
            .with_title(config.title)
            .with_visible(true)
            .with_decorations(true)
            .with_resizable(true)
            .with_window_icon(config.icon);

        let gl_context = ContextBuilder::new()
            .with_gl(/*GlRequest::GlThenGles {
                opengl_version: (2, 0),
                opengles_version: (2, 0)
            }*/GlRequest::Specific(Api::OpenGlEs, (2, 0))
                     /*GlRequest::Latest*/)
            .with_vsync(true)
            //.with_gl_profile(GlProfile::Core)
            //.with_pixel_format(24, 8)
            //.with_stencil_buffer(8)
            //.with_depth_buffer(0)
            //.with_double_buffer(Some(true))
            //.with_multisampling(0)
            .build_windowed(window_builder, &event_loop)
            .unwrap();

        Self { event_loop, gl_context }
    }

    pub fn run<H>(self, mut handler: H)
    where
        H: EventHandler<Context = GlContext> + 'static,
    {
        let Platform { event_loop, gl_context } = self;
        let mut gl_context = Some(gl_context);
        let mut view: Option<View> = None;

        #[cfg(not(target_os = "android"))]
        match View::try_init(gl_context.take().unwrap()) {
            Ok(v) => {
                view = v.into();
            },
            Err((c, e)) => {
                println!("View init error: {}", e);
                gl_context = c.into();
            },
        }

        event_loop.run(move |event, _, control_flow| {
            use self::Event::*;
            match event {
                RedrawRequested(_window_id) => {
                    // Redraw the application.
                    //
                    // It's preferrable to render in this event rather than in EventsCleared, since
                    // rendering in here allows the program to gracefully handle redraws requested
                    // by the OS.
                    if let Some(view) = &mut view {
                        view.redraw(&mut handler);
                    }
                },
                RedrawEventsCleared => {
                    if let Some(view) = &mut view {
                        view.request_redraw();
                    }
                },
                LoopDestroyed => {
                    handler.destroy();
                },
                Suspended => {
                    handler.suspend();
                    if view.is_some() {
                        gl_context = view.take().unwrap().teardown().into();
                    }
                },
                Resumed => {
                    if view.is_none() {
                        match View::try_init(gl_context.take().unwrap()) {
                            Ok(v) => {
                                view = v.into();
                                handler.resume();
                            },
                            Err((c, e)) => {
                                println!("View init error: {}", e);
                                gl_context = c.into();
                            },
                        }
                    }
                },
                WindowEvent { event, .. } => {
                    if let Some(view) = &mut view {
                        view.handle(event, control_flow, &mut handler);
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
        });
    }
}
