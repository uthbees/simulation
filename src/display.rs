use std::iter::once;

use wgpu::{Device, PresentMode, Queue, RenderPipeline, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub struct Display<'a> {
    window: &'a Window,
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    render_pipeline: RenderPipeline,
}

impl<'a> Display<'a> {
    // TODO: Break this function up.
    #[expect(clippy::too_many_lines)]
    pub async fn new(window: &'a Window) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let enabled_backends = wgpu::Backends::PRIMARY;
        #[cfg(target_arch = "wasm32")]
        let enabled_backends = wgpu::Backends::GL;

        let wgpu_instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: enabled_backends,
            ..wgpu::InstanceDescriptor::default()
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
            width: if win_size.width > 0 {
                win_size.width
            } else {
                1
            },
            height: if win_size.height > 0 {
                win_size.height
            } else {
                1
            },
            present_mode: PresentMode::Fifo,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        // Initialize the surface. Doing this in the constructor is necessary for WASM and decreases the startup time for desktop.
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render pipeline layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vert_main",
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "frag_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Self {
            window,
            surface,
            device,
            queue,
            config,
            render_pipeline,
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

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.draw(0..3, 0..1);

        // We have to explicitly end the render pass by dropping it before calling encoder.finish().
        drop(render_pass);

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
        let _ = window.request_inner_size(PhysicalSize::new(2048, 1024));
    }

    window
}

/// Converts a color value from the standard srgb format (eg (0, 255, 127)) to the linear rgb format that wgpu expects.
/// See [the Wikipedia article on sRGB](https://en.wikipedia.org/wiki/SRGB#From_sRGB_to_CIE_XYZ).
#[must_use]
fn get_linear_rgb(standard_rgb: [i32; 3]) -> [f64; 3] {
    fn get_linear_color(standard_color: i32) -> f64 {
        let normalized_standard_color = f64::from(standard_color) / 255.0;

        if normalized_standard_color <= 0.04045 {
            return normalized_standard_color / 12.92;
        }

        ((normalized_standard_color + 0.055) / 1.055).powf(2.4)
    }

    [
        get_linear_color(standard_rgb[0]),
        get_linear_color(standard_rgb[1]),
        get_linear_color(standard_rgb[2]),
    ]
}
