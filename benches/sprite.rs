use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vulpo::backend::sprite::Sprites;

fn generate_sprites(n: usize) -> Sprites {
    let mut sprites = Sprites::new();
    for i in 0..n {
        sprites.add(
            0,
            ultraviolet::Vec2::new(0.0, 0.0),
            ultraviolet::Vec2::new(128.0, 128.0),
            ultraviolet::Vec2::new(128.0 * i as f32, 128.0 * i as f32),
            i as f32,
            ultraviolet::Vec2::new(i as f32, i as f32),
            1.0,
            [1.0, 1.0, 1.0, 1.0],
            ultraviolet::Vec2::new(0.0, 0.0),
        );
    }

    sprites
}

fn vertex_generation_benches(c: &mut Criterion) {
    let sprites = generate_sprites(200_000);

    c.bench_function("two_hundred_thousand_sprites", |b| {
        b.iter(|| black_box(sprites.vertices_indices(128, 128)))
    });
}

criterion_group!(sprite_vertex_generation, vertex_generation_benches);
criterion_main!(sprite_vertex_generation);
