
#[tokio::main]
async fn main() {
    env_logger::init();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        flags: wgpu::InstanceFlags::default(),
        backend_options: Default::default(),
        display: None,
        memory_budget_thresholds: Default::default(),
    });

    println!("-- GPUs found on this machine --");

    let adapters = instance
        .enumerate_adapters(wgpu::Backends::all())
        .await;

    for adapter in adapters {
        let info = adapter.get_info();

        println!("Name:\t\t{}", info.name);
        println!("Backend:\t\t{}", info.backend);
        println!("Type:\t\t{:?}", info.device_type);
        println!("Driver:\t\t{} {}", info.driver, info.driver_info);

        let limits = adapter.limits();

        println!("Max texture size:\t\t{}", limits.max_texture_dimension_2d);
        println!("Max buffer size:\t\t{}", limits.max_buffer_size);
        println!("Max bind groups:\t\t{}", limits.max_bind_groups);
        println!("Max Vertex attribs:\t\t{}", limits.max_vertex_attributes);
        println!("Max compute group:\t\t{}", limits.max_compute_workgroup_size_x);

        println!();
    }
}
