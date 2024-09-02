use std::iter::once;

use wgpu::{Device, PresentMode, Queue, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub struct Display<'a> {
    window: &'a Window,
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
}

impl<'a> Display<'a> {
    pub async fn new(window: &'a Window) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let enabled_backends = wgpu::Backends::PRIMARY;
        #[cfg(target_arch = "wasm32")]
        let enabled_backends = wgpu::Backends::GL;

        let wgpu_instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: enabled_backends,
            ..Default::default()
        });

        let surface = wgpu_instance
            .create_surface(window)
            .expect("Failed to initialize renderer: couldn't create surface");

        let adapter = wgpu_instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to initialize renderer: couldn't get adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if we're building for the web, we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    memory_hints: wgpu::MemoryHints::default(),
                    label: None,
                },
                None,
            )
            .await
            .expect("Failed to initialize renderer: couldn't connect to device");

        let win_size = window.inner_size();

        let capabilities = surface.get_capabilities(&adapter);
        let surface_format = capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: win_size.width,
            height: win_size.height,
            present_mode: PresentMode::Fifo,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Self {
            window,
            surface,
            device,
            queue,
            config,
        }
    }

    /// Returns a reference to the Window object.
    ///
    /// We have to do things this way instead of just using the object directly because the Surface
    /// object needs to be next to the borrowed Window object in the struct (not owned!).
    pub fn window(&self) -> &Window {
        self.window
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.configure_surface();
        }
    }

    pub fn configure_surface(&self) {
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        // submit will accept anything that implements IntoIter
        self.queue.submit(once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub fn create_window(event_loop: &EventLoop<()>) -> Window {
    let window = WindowBuilder::new()
        .build(event_loop)
        .expect("Failed to create window");

    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::LogicalSize;
        use winit::platform::web::WindowExtWebSys;

        web_sys::window()
            .and_then(|js_window| js_window.document())
            .and_then(|document| {
                let destination = document.get_element_by_id("canvas-holder")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                destination.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document");

        // winit doesn't allow sizing with CSS, so we have to set the size manually when on web.
        // Note that this sets the size of the canvas on web, not the window itself.
        let _ = window.request_inner_size(LogicalSize::new(1000, 500));
    }

    window
}