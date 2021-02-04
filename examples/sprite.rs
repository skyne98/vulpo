use vulpo::backend::pipeline::sprite::SpritePipeline;
use vulpo::backend::resource::build_bind_group;
use vulpo::backend::resource::sampler::Sampler;
use vulpo::backend::resource::texture::Texture;
use vulpo::backend::window::Window;

fn main() {
    env_logger::init();
    let vulpo_window = Window::new(
        |device, queue| {
            // Texture
            let diffuse_bytes = include_bytes!("../assets/noise_90x90.png");
            let diffuse_texture =
                Texture::from_bytes(&device, &queue, diffuse_bytes, "skeleton.png").unwrap();
            let sampler = Sampler::pixel(&device);
            // Bind group
            let (bind_group_layout, bind_group) = build_bind_group(
                &device,
                wgpu::ShaderStage::FRAGMENT,
                vec![&diffuse_texture, &sampler],
            );

            vec![(bind_group_layout, bind_group)]
        },
        |device, texture_format| SpritePipeline::new(&device, texture_format, 90, 90),
    );
}
