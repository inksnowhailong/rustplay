// 定义粒子数据结构
struct Particle {
    position: vec2<f32>,
    velocity: vec2<f32>,
}

// 定义存储缓冲区
@group(0) @binding(0)
var<storage, read_write> particles: array<Particle>;

// 定义常量
const GRAVITY: vec2<f32> = vec2<f32>(0.0, -9.81);
const DELTA_TIME: f32 = 0.016; // 约60fps
const BOUNDS: vec2<f32> = vec2<f32>(800.0, 600.0); // 画布大小
const PARTICLE_RADIUS: f32 = 2.0;

// 计算着色器入口点
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= arrayLength(&particles)) {
        return;
    }

    var particle = particles[index];

    // 更新速度（应用重力）
    particle.velocity = particle.velocity + GRAVITY * DELTA_TIME;

    // 更新位置
    particle.position = particle.position + particle.velocity * DELTA_TIME;

    // 边界检查
    if (particle.position.x < PARTICLE_RADIUS) {
        particle.position.x = PARTICLE_RADIUS;
        particle.velocity.x = -particle.velocity.x * 0.8; // 反弹，损失一些能量
    }
    if (particle.position.x > BOUNDS.x - PARTICLE_RADIUS) {
        particle.position.x = BOUNDS.x - PARTICLE_RADIUS;
        particle.velocity.x = -particle.velocity.x * 0.8;
    }
    if (particle.position.y < PARTICLE_RADIUS) {
        particle.position.y = PARTICLE_RADIUS;
        particle.velocity.y = -particle.velocity.y * 0.8;
    }
    if (particle.position.y > BOUNDS.y - PARTICLE_RADIUS) {
        particle.position.y = BOUNDS.y - PARTICLE_RADIUS;
        particle.velocity.y = -particle.velocity.y * 0.8;
    }

    // 存储更新后的粒子数据
    particles[index] = particle;
}
