use std::borrow::Cow;

use wgpu::util::DeviceExt;
use winit::{event::WindowEvent, window::Window};

use crate::cell::Cell;

pub struct Game {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    cells: Vec<Cell>,
    cell_size: u32,
    num_cells_x: u32,
    num_cells_y: u32,

    pv_mat: glam::Mat4,
    pv_mat_bind_group: wgpu::BindGroup,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    state_buffer: wgpu::Buffer,
    model_mats_buffer: wgpu::Buffer,

    render_pipeline: wgpu::RenderPipeline,
}

impl Game {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                power_preference: wgpu::PowerPreference::HighPerformance,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    label: None,
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let num_cells_x = 80;
        let (num_cells_y, cell_size) = Self::calculate_cells(num_cells_x, &size);
        let mut count = 0;
        let cells = (0..num_cells_y)
            .into_iter()
            .flat_map(|y| {
                count += 1;
                (0..num_cells_x).into_iter().map(move |x| {
                    count += 1;
                    Cell {
                        position: glam::vec2((x * cell_size) as f32, (y * cell_size) as f32),
                        state: (count % 2) == 1,
                    }
                })
            })
            .collect::<Vec<_>>();

        let model_matricies_data = cells
            .iter()
            .map(|cell| cell.model_matrix(cell_size as f32))
            .collect::<Vec<_>>();
        let model_mats_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&model_matricies_data),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let state_data = cells
            .iter()
            .map(|cell| cell.state as u32 as f32)
            .collect::<Vec<_>>();
        let state_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&state_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let pv_mat = glam::Mat4::orthographic_rh(
            0.0,
            size.width as f32,
            size.height as f32,
            0.0,
            0.0,
            100.0,
        );
        let pv_mat_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&(pv_mat.to_cols_array_2d())),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let pv_mat_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                }],
            });
        let pv_mat_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pv_mat_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: pv_mat_buffer.as_entire_binding(),
            }],
        });

        let vertex_data: Vec<f32> = vec![0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0];
        let index_data: Vec<u16> = vec![0, 1, 2, 2, 1, 3];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&index_data),
            usage: wgpu::BufferUsages::INDEX,
        });

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&pv_mat_bind_group_layout],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Cell::vertex_desc(), Cell::matrix_desc(), Cell::state_desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[config.format.into()],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,

            cells,
            num_cells_x,
            num_cells_y,
            cell_size,

            pv_mat,
            pv_mat_bind_group,

            vertex_buffer,
            index_buffer,
            state_buffer,
            model_mats_buffer,

            render_pipeline,
        }
    }

    pub fn input(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Resized(physical_size) => {
                self.size = *physical_size;
                self.resize();
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.size = **new_inner_size;
                self.resize();
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.pv_mat_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.model_mats_buffer.slice(..));
            render_pass.set_vertex_buffer(2, self.state_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..6, 0, 0..self.cells.len() as u32);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn resize(&mut self) {
        self.config.width = self.size.width;
        self.config.height = self.size.height;
        self.surface.configure(&self.device, &self.config);
    }

    fn calculate_cells(num_cells_x: u32, size: &winit::dpi::PhysicalSize<u32>) -> (u32, u32) {
        let cell_size = size.width as f32 / num_cells_x as f32;
        let num_cells_y = size.height as f32 / cell_size;
        (num_cells_y.ceil() as u32, cell_size.ceil() as u32)
    }
}
