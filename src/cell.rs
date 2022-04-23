pub struct Cell {
    pub position: glam::Vec2,
    pub state: bool,
}

impl Cell {
    pub fn model_matrix(&self, size: f32) -> [[f32; 4]; 4] {
        glam::Mat4::from_scale_rotation_translation(
            glam::vec3(size, size, 1.0),
            glam::Quat::IDENTITY,
            glam::vec3(self.position.x, self.position.y, 0.0),
        )
        .to_cols_array_2d()
    }

    const MATRIX_ATTRIBS: [wgpu::VertexAttribute; 4] =
        wgpu::vertex_attr_array![2 => Float32x4, 3 => Float32x4, 4 => Float32x4, 5 => Float32x4];

    pub fn matrix_desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::MATRIX_ATTRIBS,
        }
    }

    const VERTEX_ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x2];

    pub fn vertex_desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::VERTEX_ATTRIBS,
        }
    }

    const STATE_ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![1 => Uint32];

    pub fn state_desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<f32>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::STATE_ATTRIBS,
        }
    }
}
