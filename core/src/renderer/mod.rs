use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use wgpu;
use winit::window::Window;

use crate::text::TextRenderer;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Globals {
    screen_width: f32,
    screen_height: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct RectUniform {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
    border_radius: f32,
}

#[derive(Clone)]
pub struct Rect {
    pub id: u64,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
    pub node_type: String,
    pub on_click_id: String,
    pub on_change_text_id: String,
    pub text_content: String,
    pub font_size: f32,
    pub text_color: [f32; 4],
    pub scroll_offset_x: f32,
    pub scroll_offset_y: f32,
    pub clip_rect: Option<[f32; 4]>,
    pub image_source: Option<String>,
    pub hover_color: Option<[f32; 4]>,
    pub active_color: Option<[f32; 4]>,
    pub hover_text_color: Option<[f32; 4]>,
    pub active_text_color: Option<[f32; 4]>,
    pub disabled: bool,
    pub border_radius: f32,
}

struct LoadedImage {
    #[allow(dead_code)]
    texture_view: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
}

pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface<'static>,
    pipeline: wgpu::RenderPipeline,
    texture_pipeline: wgpu::RenderPipeline,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    texture_sampler: wgpu::Sampler,
    globals_buffer: wgpu::Buffer,
    rect_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    #[allow(dead_code)]
    window: Arc<Window>,
    text_renderer: TextRenderer,
    image_cache: Mutex<HashMap<String, LoadedImage>>,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Axo Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .unwrap();

        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Axo Shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let texture_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Axo Textured Shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("textured.wgsl"))),
        });

        let globals_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Globals Buffer"),
            size: std::mem::size_of::<Globals>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let rect_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Rect Buffer"),
            size: std::mem::size_of::<RectUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Axo Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::all(),
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Axo Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &globals_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &rect_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Axo Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Axo Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // Textured pipeline
        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Axo Texture Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::all(),
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Axo Texture Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let texture_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Axo Texture Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let texture_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Axo Texture Render Pipeline"),
            layout: Some(&texture_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &texture_shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &texture_shader,
                entry_point: "fs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        std::mem::forget(instance);

        Renderer {
            device, queue, config, surface, pipeline,
            texture_pipeline, texture_bind_group_layout, texture_sampler,
            globals_buffer, rect_buffer, bind_group, window,
            text_renderer: TextRenderer::new(),
            image_cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn viewport_size(&self) -> (f32, f32) {
        (self.config.width as f32, self.config.height as f32)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width.max(1);
        self.config.height = height.max(1);
        self.surface.configure(&self.device, &self.config);
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_rect_internal(&self, rpass: &mut wgpu::RenderPass, x: f32, y: f32, w: f32, h: f32, color: [f32; 4], br: f32) {
        let ru = RectUniform { x, y, w, h, r: color[0], g: color[1], b: color[2], a: color[3], border_radius: br };
        self.queue.write_buffer(&self.rect_buffer, 0, bytemuck::bytes_of(&ru));
        rpass.draw(0..6, 0..1);
    }

    fn ensure_image_loaded(&self, path: &str) {
        let mut cache = self.image_cache.lock().unwrap();
        if cache.contains_key(path) {
            return;
        }

        let img = match image::open(path) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("[Image] Failed to load '{}': {}", path, e);
                return;
            }
        };
        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();

        let texture_size = wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        };

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&format!("Image: {}", path)),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * w),
                rows_per_image: Some(h),
            },
            texture_size,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("ImageBG: {}", path)),
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &self.globals_buffer, offset: 0, size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &self.rect_buffer, offset: 0, size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&self.texture_sampler),
                },
            ],
        });

        let loaded = LoadedImage { texture_view, bind_group };
        cache.insert(path.to_string(), loaded);
    }

    pub fn render(&mut self, rects: &[Rect], hovered_id: Option<u64>, active_id: Option<u64>) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Axo Encoder"),
        });

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Axo Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.1, b: 0.2, a: 1.0 }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        let globals = Globals {
            screen_width: self.config.width as f32,
            screen_height: self.config.height as f32,
        };
        self.queue.write_buffer(&self.globals_buffer, 0, bytemuck::bytes_of(&globals));

        for rect in rects {
            let rx = rect.x - rect.scroll_offset_x;
            let ry = rect.y - rect.scroll_offset_y;

            // Resolve hover/active colors
            let is_active = active_id == Some(rect.id);
            let is_hovered = hovered_id == Some(rect.id);

            let bg_color = if rect.disabled {
                [rect.color[0] * 0.5, rect.color[1] * 0.5, rect.color[2] * 0.5, rect.color[3] * 0.6]
            } else if is_active {
                rect.active_color.unwrap_or(rect.color)
            } else if is_hovered {
                rect.hover_color.unwrap_or(rect.color)
            } else {
                rect.color
            };

            let text_color = if rect.disabled {
                [rect.text_color[0] * 0.5, rect.text_color[1] * 0.5, rect.text_color[2] * 0.5, rect.text_color[3] * 0.6]
            } else if is_active {
                rect.active_text_color.unwrap_or(rect.text_color)
            } else if is_hovered {
                rect.hover_text_color.unwrap_or(rect.text_color)
            } else {
                rect.text_color
            };

            // Check clipping
            if let Some([cx, cy, cw, ch]) = rect.clip_rect {
                if rx + rect.w < cx || rx > cx + cw || ry + rect.h < cy || ry > cy + ch {
                    continue;
                }
                let clip_x = rx.max(cx);
                let clip_y = ry.max(cy);
                let clip_w = (rx + rect.w).min(cx + cw) - clip_x;
                let clip_h = (ry + rect.h).min(cy + ch) - clip_y;
                if clip_w <= 0.0 || clip_h <= 0.0 {
                    continue;
                }

                // Draw background clipped
                if rect.node_type == "Image" && rect.image_source.is_some() {
                    if let Some(ref src) = rect.image_source {
                        self.ensure_image_loaded(src);
                    }
                    self.render_image(&mut rpass, rect, rx, ry);
                } else {
                    rpass.set_pipeline(&self.pipeline);
                    rpass.set_bind_group(0, &self.bind_group, &[]);
                    self.draw_rect_internal(&mut rpass, clip_x, clip_y, clip_w, clip_h, bg_color, rect.border_radius);
                }

                // Draw text clipped
                if !rect.text_content.is_empty() {
                    let glyphs = self.text_renderer.layout(
                        &rect.text_content, rect.font_size, rect.w,
                    );
                    for g in &glyphs {
                        let gx = rx + g.x;
                        let gy = ry + g.y;
                        if gx + g.w < clip_x || gx > clip_x + clip_w
                            || gy + g.h < clip_y || gy > clip_y + clip_h
                        {
                            continue;
                        }
                        let tgx = gx.max(clip_x);
                        let tgy = gy.max(clip_y);
                        let tgw = (gx + g.w).min(clip_x + clip_w) - tgx;
                        let tgh = (gy + g.h).min(clip_y + clip_h) - tgy;
                        if tgw > 0.0 && tgh > 0.0 {
                            rpass.set_pipeline(&self.pipeline);
                            rpass.set_bind_group(0, &self.bind_group, &[]);
                            self.draw_rect_internal(&mut rpass, tgx, tgy, tgw, tgh, text_color, 0.0);
                        }
                    }
                }
            } else {
                // No clipping — draw normally with scroll offset
                if rect.node_type == "Image" && rect.image_source.is_some() {
                    if let Some(ref src) = rect.image_source {
                        self.ensure_image_loaded(src);
                    }
                    self.render_image(&mut rpass, rect, rx, ry);
                } else {
                    rpass.set_pipeline(&self.pipeline);
                    rpass.set_bind_group(0, &self.bind_group, &[]);
                    self.draw_rect_internal(&mut rpass, rx, ry, rect.w, rect.h, bg_color, rect.border_radius);
                }

                if !rect.text_content.is_empty() {
                    let glyphs = self.text_renderer.layout(
                        &rect.text_content, rect.font_size, rect.w,
                    );
                    for g in &glyphs {
                        rpass.set_pipeline(&self.pipeline);
                        rpass.set_bind_group(0, &self.bind_group, &[]);
                        self.draw_rect_internal(&mut rpass, rx + g.x, ry + g.y, g.w, g.h, text_color, 0.0);
                    }
                }
            }
        }

        drop(rpass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn render_image(&self, rpass: &mut wgpu::RenderPass, rect: &Rect, rx: f32, ry: f32) {
        if let Some(ref source) = rect.image_source {
            let cache = self.image_cache.lock().unwrap();
            if let Some(img) = cache.get(source) {
                rpass.set_pipeline(&self.texture_pipeline);
                rpass.set_bind_group(0, &img.bind_group, &[]);
                let ru = RectUniform { x: rx, y: ry, w: rect.w, h: rect.h, r: 1.0, g: 1.0, b: 1.0, a: 1.0, border_radius: 0.0 };
                self.queue.write_buffer(&self.rect_buffer, 0, bytemuck::bytes_of(&ru));
                rpass.draw(0..6, 0..1);
            }
        }
    }
}
