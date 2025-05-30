use wasm_bindgen::prelude::*;
use wgpu::util::DeviceExt;

#[wasm_bindgen]
pub struct ParticleSimulator {
    device: wgpu::Device,
    queue: wgpu::Queue,
    compute_pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
    particle_buffer: wgpu::Buffer,
    num_particles: u32,
}

#[wasm_bindgen]
impl ParticleSimulator {
    #[wasm_bindgen(constructor)]
    pub async fn new(num_particles: u32) -> Result<ParticleSimulator, JsValue> {
        // 初始化 GPU
        let instance = wgpu::Instance::default();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .ok_or_else(|| JsValue::from_str("Failed to find an appropriate adapter"))?;

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor::default(),
            None
        ).await.map_err(|e| JsValue::from_str(&format!("Failed to request device: {:?}", e)))?;

        // 创建粒子数据缓冲区
        let initial_data = vec![0.0f32; (num_particles * 4) as usize]; // 每个粒子4个float（x, y, vx, vy）
        let particle_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Particle Buffer"),
            contents: bytemuck::cast_slice(&initial_data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        });

        // 加载 compute shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/compute.wgsl").into()),
        });

        // 创建 compute pipeline
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: None,
            module: &shader,
            entry_point: "main",
        });

        // 创建 bind group
        let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: particle_buffer.as_entire_binding(),
            }],
            label: Some("Bind Group"),
        });

        Ok(ParticleSimulator {
            device,
            queue,
            compute_pipeline,
            bind_group,
            particle_buffer,
            num_particles,
        })
    }

    pub fn update(&self) {
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Compute Encoder"),
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.bind_group, &[]);
            compute_pass.dispatch_workgroups((self.num_particles + 63) / 64, 1, 1);
        }

        self.queue.submit(Some(encoder.finish()));
    }

    pub fn get_particle_buffer(&self) -> js_sys::Uint8Array {
        // 这里需要实现从 GPU 读取数据到 CPU 的逻辑
        // 出于简化目的，此处省略具体实现
        js_sys::Uint8Array::new_with_length((self.num_particles * 4 * 4) as u32)
    }
}
