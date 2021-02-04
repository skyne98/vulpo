pub struct Sprites {
    texture_index: Vec<u32>,
    source_position: Vec<ultraviolet::Vec2>,
    source_size: Vec<ultraviolet::Vec2>,
    position: Vec<ultraviolet::Vec2>,
    angle: Vec<f32>,
    scale: Vec<ultraviolet::Vec2>,
    depth: Vec<f32>,
    color: Vec<[f32; 4]>,
    origin: Vec<ultraviolet::Vec2>,
}

impl Sprites {
    pub fn new() -> Self {
        Self {
            texture_index: vec![],
            source_position: vec![],
            source_size: vec![],
            position: vec![],
            angle: vec![],
            scale: vec![],
            depth: vec![],
            color: vec![],
            origin: vec![],
        }
    }

    pub fn add(
        &mut self,
        texture_index: u32,
        source_position: ultraviolet::Vec2,
        source_size: ultraviolet::Vec2,
        position: ultraviolet::Vec2,
        angle: f32,
        scale: ultraviolet::Vec2,
        depth: f32,
        color: [f32; 4],
        origin: ultraviolet::Vec2,
    ) -> usize {
        let index = self.texture_index.len();
        self.texture_index.push(texture_index);
        self.source_position.push(source_position);
        self.source_size.push(source_size);
        self.position.push(position);
        self.angle.push(angle);
        self.scale.push(scale);
        self.depth.push(depth);
        self.color.push(color);
        self.origin.push(origin);

        index
    }

    pub fn update(
        &mut self,
        index: usize,
        texture_index: u32,
        source_position: ultraviolet::Vec2,
        source_size: ultraviolet::Vec2,
        position: ultraviolet::Vec2,
        angle: f32,
        scale: ultraviolet::Vec2,
        depth: f32,
        color: [f32; 4],
        origin: ultraviolet::Vec2,
    ) -> Option<()> {
        *(self.texture_index.get_mut(index)?) = texture_index;
        *(self.source_position.get_mut(index)?) = source_position;
        *(self.source_size.get_mut(index)?) = source_size;
        *(self.position.get_mut(index)?) = position;
        *(self.angle.get_mut(index)?) = angle;
        *(self.scale.get_mut(index)?) = scale;
        *(self.depth.get_mut(index)?) = depth;
        *(self.color.get_mut(index)?) = color;
        *(self.origin.get_mut(index)?) = origin;

        Some(())
    }

    pub fn get(
        &self,
        index: usize,
    ) -> Option<(
        &u32,
        &ultraviolet::Vec2,
        &ultraviolet::Vec2,
        &ultraviolet::Vec2,
        &f32,
        &ultraviolet::Vec2,
        &f32,
        &[f32; 4],
        &ultraviolet::Vec2,
    )> {
        Some((
            self.texture_index.get(index)?,
            self.source_position.get(index)?,
            self.source_size.get(index)?,
            self.position.get(index)?,
            self.angle.get(index)?,
            self.scale.get(index)?,
            self.depth.get(index)?,
            self.color.get(index)?,
            self.origin.get(index)?,
        ))
    }

    pub fn len(&self) -> usize {
        self.texture_index.len()
    }

    pub fn remove(&mut self, index: usize) {
        self.texture_index.remove(index);
        self.source_position.remove(index);
        self.source_size.remove(index);
        self.position.remove(index);
        self.angle.remove(index);
        self.scale.remove(index);
        self.depth.remove(index);
        self.color.remove(index);
        self.origin.remove(index);
    }
}
