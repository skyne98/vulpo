use anyhow::Result;
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread::JoinHandle,
};

use crate::backend::vertex::Vertex;

const BASE_VEC_A: ultraviolet::Vec3 = ultraviolet::Vec3::new(0.0, 1.0, 1.0);
const BASE_VEC_B: ultraviolet::Vec3 = ultraviolet::Vec3::new(0.0, 0.0, 1.0);
const BASE_VEC_C: ultraviolet::Vec3 = ultraviolet::Vec3::new(1.0, 0.0, 1.0);
const BASE_VEC_D: ultraviolet::Vec3 = ultraviolet::Vec3::new(1.0, 1.0, 1.0);

const BASE_UV_A: ultraviolet::Vec2 = ultraviolet::Vec2::new(0.0, 0.0);
const BASE_UV_B: ultraviolet::Vec2 = ultraviolet::Vec2::new(0.0, 1.0);
const BASE_UV_C: ultraviolet::Vec2 = ultraviolet::Vec2::new(1.0, 1.0);
const BASE_UV_D: ultraviolet::Vec2 = ultraviolet::Vec2::new(1.0, 0.0);

const BASELINE_INDICES: &[u16] = &[0, 1, 3, 3, 1, 2];

#[inline]
fn calculate_scale_mat(
    scale: ultraviolet::Vec2,
    source_size: ultraviolet::Vec2,
) -> ultraviolet::Mat3 {
    ultraviolet::Mat3::from_nonuniform_scale(ultraviolet::Vec3::new(
        source_size.x * scale.x,
        source_size.y * scale.y,
        1.0,
    ))
}
#[inline]
fn calculate_origin_translation_mat(
    origin: ultraviolet::Vec2,
    scale: ultraviolet::Vec2,
    source_size: ultraviolet::Vec2,
) -> ultraviolet::Mat3 {
    ultraviolet::Mat3::from_translation(ultraviolet::Vec2::new(
        -source_size.x * origin.x * scale.x,
        -source_size.y * origin.y * scale.y,
    ))
}
#[inline]
fn calculate_rotation_mat(angle: f32) -> ultraviolet::Mat3 {
    ultraviolet::Mat3::from_rotation_z(angle * std::f32::consts::PI / 180.0)
}
#[inline]
fn calculate_translation_mat(position: ultraviolet::Vec2) -> ultraviolet::Mat3 {
    ultraviolet::Mat3::from_translation(ultraviolet::Vec2::new((position).x, (position).y))
}

pub struct Sprites {
    length: usize,
    texture_index: Vec<u32>,
    source_position: Vec<ultraviolet::Vec2>,
    source_size: Vec<ultraviolet::Vec2>,
    position: Vec<ultraviolet::Vec2>,
    angle: Vec<f32>,
    scale: Vec<ultraviolet::Vec2>,
    depth: Vec<f32>,
    color: Vec<[f32; 4]>,
    origin: Vec<ultraviolet::Vec2>,

    scale_mat: Vec<ultraviolet::Mat3>,
    origin_translation_mat: Vec<ultraviolet::Mat3>,
    rotation_mat: Vec<ultraviolet::Mat3>,
    translation_mat: Vec<ultraviolet::Mat3>,

    threads: Vec<SpriteThread>,
}

impl Sprites {
    pub fn new() -> Self {
        Self {
            length: 0,
            texture_index: vec![],
            source_position: vec![],
            source_size: vec![],
            position: vec![],
            angle: vec![],
            scale: vec![],
            depth: vec![],
            color: vec![],
            origin: vec![],

            scale_mat: vec![],
            origin_translation_mat: vec![],
            rotation_mat: vec![],
            translation_mat: vec![],

            threads: vec![SpriteThread::new()],
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
        self.length += 1;
        self.texture_index.push(texture_index);
        self.source_position.push(source_position);
        self.source_size.push(source_size);
        self.position.push(position);
        self.angle.push(angle);
        self.scale.push(scale);
        self.depth.push(depth);
        self.color.push(color);
        self.origin.push(origin);

        self.scale_mat.push(calculate_scale_mat(scale, source_size));
        self.origin_translation_mat
            .push(calculate_origin_translation_mat(origin, scale, source_size));
        self.rotation_mat.push(calculate_rotation_mat(angle));
        self.translation_mat
            .push(calculate_translation_mat(position));

        index
    }

    pub fn set_texture_index(&mut self, index: usize, val: u32) -> Option<()> {
        *(self.texture_index.get_mut(index)?) = val;
        Some(())
    }
    pub fn set_source_position(&mut self, index: usize, val: ultraviolet::Vec2) -> Option<()> {
        *(self.source_position.get_mut(index)?) = val;
        Some(())
    }
    pub fn set_source_size(&mut self, index: usize, val: ultraviolet::Vec2) -> Option<()> {
        *(self.source_size.get_mut(index)?) = val;

        // Recreate matrices
        let scale = self.scale.get(index)?;
        let origin = self.origin.get(index)?;
        *(self.scale_mat.get_mut(index)?) = calculate_scale_mat(*scale, val);
        *(self.origin_translation_mat.get_mut(index)?) =
            calculate_origin_translation_mat(*origin, *scale, val);

        Some(())
    }
    pub fn set_position(&mut self, index: usize, val: ultraviolet::Vec2) -> Option<()> {
        *(self.position.get_mut(index)?) = val;

        // Recreate matrices
        *(self.translation_mat.get_mut(index)?) = calculate_translation_mat(val);

        Some(())
    }
    pub fn set_angle(&mut self, index: usize, val: f32) -> Option<()> {
        *(self.angle.get_mut(index)?) = val;

        // Recreate matrices
        *(self.rotation_mat.get_mut(index)?) = calculate_rotation_mat(val);

        Some(())
    }
    pub fn set_scale(&mut self, index: usize, val: ultraviolet::Vec2) -> Option<()> {
        *(self.scale.get_mut(index)?) = val;

        // Recreate matrices
        let origin = self.origin.get(index)?;
        let source_size = self.source_size.get(index)?;
        *(self.scale_mat.get_mut(index)?) = calculate_scale_mat(val, *source_size);
        *(self.origin_translation_mat.get_mut(index)?) =
            calculate_origin_translation_mat(*origin, val, *source_size);

        Some(())
    }
    pub fn set_depth(&mut self, index: usize, val: f32) -> Option<()> {
        *(self.depth.get_mut(index)?) = val;
        Some(())
    }
    pub fn set_color(&mut self, index: usize, val: [f32; 4]) -> Option<()> {
        *(self.color.get_mut(index)?) = val;
        Some(())
    }
    pub fn set_origin(&mut self, index: usize, val: ultraviolet::Vec2) -> Option<()> {
        *(self.origin.get_mut(index)?) = val;

        // Recreate matrices
        let scale = self.scale.get(index)?;
        let source_size = self.source_size.get(index)?;
        *(self.origin_translation_mat.get_mut(index)?) =
            calculate_origin_translation_mat(val, *scale, *source_size);

        Some(())
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn remove(&mut self, index: usize) {
        self.length -= 1;
        self.texture_index.remove(index);
        self.source_position.remove(index);
        self.source_size.remove(index);
        self.position.remove(index);
        self.angle.remove(index);
        self.scale.remove(index);
        self.depth.remove(index);
        self.color.remove(index);
        self.origin.remove(index);

        self.scale_mat.remove(index);
        self.origin_translation_mat.remove(index);
        self.rotation_mat.remove(index);
        self.translation_mat.remove(index);
    }

    pub fn vertices_indices(
        &self,
        texture_width: u32,
        texture_height: u32,
    ) -> (Vec<Vertex>, Vec<u16>) {
        let mut result_vertices = vec![];
        let mut result_indices = vec![];
        let main_thread = self.threads.first().unwrap();

        let thread_count = self.threads.len();
        let chunk_size = self.length / thread_count;
        let leftover = self.length - (chunk_size * thread_count);

        /* First process leftovers */
        if leftover > 0 {
            let leftover_length = leftover;
            let leftover_source_positions = Vec::from(&self.source_position[0..leftover_length]);
            let leftover_source_size = Vec::from(&self.source_size[0..leftover_length]);
            let leftover_scale_mat = Vec::from(&self.scale_mat[0..leftover_length]);
            let leftover_origin_translation_mat =
                Vec::from(&self.origin_translation_mat[0..leftover_length]);
            let leftover_rotation_mat = Vec::from(&self.rotation_mat[0..leftover_length]);
            let leftover_translation = Vec::from(&self.translation_mat[0..leftover_length]);
            main_thread
                .send(Some((
                    0,
                    leftover_length,
                    texture_width,
                    texture_height,
                    leftover_source_positions,
                    leftover_source_size,
                    leftover_scale_mat,
                    leftover_origin_translation_mat,
                    leftover_rotation_mat,
                    leftover_translation,
                )))
                .unwrap();
            let (vertices, indices) = main_thread.recv().unwrap();
            result_vertices.extend_from_slice(&vertices[..]);
            result_indices.extend_from_slice(&indices[..]);
        }

        /* Process all other chunks */
        if chunk_size > 0 {
            for thread_index in 0..thread_count {
                let thread = &self.threads[thread_index];
                let thread_slice_start = leftover + chunk_size * thread_index;
                let thread_slice_range = thread_slice_start..thread_slice_start + chunk_size;
                let thread_source_positions =
                    Vec::from(&self.source_position[thread_slice_range.clone()]);
                let thread_source_size = Vec::from(&self.source_size[thread_slice_range.clone()]);
                let thread_scale_mat = Vec::from(&self.scale_mat[thread_slice_range.clone()]);
                let thread_origin_translation_mat =
                    Vec::from(&self.origin_translation_mat[thread_slice_range.clone()]);
                let thread_rotation_mat = Vec::from(&self.rotation_mat[thread_slice_range.clone()]);
                let thread_translation =
                    Vec::from(&self.translation_mat[thread_slice_range.clone()]);
                thread
                    .send(Some((
                        thread_slice_start,
                        thread_slice_range.len(),
                        texture_width,
                        texture_height,
                        thread_source_positions,
                        thread_source_size,
                        thread_scale_mat,
                        thread_origin_translation_mat,
                        thread_rotation_mat,
                        thread_translation,
                    )))
                    .unwrap();
            }
            for thread_index in 0..thread_count {
                let thread = &self.threads[thread_index];
                let (vertices, indices) = thread.recv().unwrap();
                result_vertices.extend_from_slice(&vertices[..]);
                result_indices.extend_from_slice(&indices[..]);
            }
        }

        (result_vertices, result_indices)
    }
}

type SpriteInputData = Option<(
    usize,
    usize,
    u32,
    u32,
    Vec<ultraviolet::Vec2>,
    Vec<ultraviolet::Vec2>,
    Vec<ultraviolet::Mat3>,
    Vec<ultraviolet::Mat3>,
    Vec<ultraviolet::Mat3>,
    Vec<ultraviolet::Mat3>,
)>;
type SpriteOutputData = (Vec<Vertex>, Vec<u16>);

pub struct SpriteThread {
    handle: JoinHandle<()>,
    receiver: Receiver<SpriteOutputData>,
    sender: Sender<SpriteInputData>,
}

impl SpriteThread {
    pub fn new() -> Self {
        let (from_thread_sender, from_thread_receiver): (
            Sender<SpriteOutputData>,
            Receiver<SpriteOutputData>,
        ) = channel();
        let (to_thread_sender, to_thread_receiver): (
            Sender<SpriteInputData>,
            Receiver<SpriteInputData>,
        ) = channel();

        let handle = std::thread::spawn(move || {
            let from_thread_sender = from_thread_sender;
            let to_thread_receiver = to_thread_receiver;

            loop {
                if let Some((
                    starting_index,
                    length,
                    texture_width,
                    texture_height,
                    source_position,
                    source_size,
                    scale_mat,
                    origin_translation_mat,
                    rotation_mat,
                    translation_mat,
                )) = to_thread_receiver.recv().unwrap()
                {
                    let range = 0..length;
                    let mut result_vertices = vec![];
                    let mut result_indices = vec![];

                    for local_index in range {
                        let source_position = source_position.get(local_index).unwrap().clone();
                        let source_size = source_size.get(local_index).unwrap().clone();

                        let scale_mat = scale_mat.get(local_index).unwrap().clone();
                        let origin_translation_mat =
                            origin_translation_mat.get(local_index).unwrap().clone();
                        let rotation_mat = rotation_mat.get(local_index).unwrap().clone();
                        let translation_mat = translation_mat.get(local_index).unwrap().clone();

                        // Relative texture coordinates
                        /*
                        let _src_relative_min_x: f32 = source_position.x / texture_width as f32;
                        let _src_relative_min_y: f32 = source_position.y / texture_height as f32;
                        let _src_relative_max_x: f32 =
                            source_position.x + source_size.x / texture_width as f32;
                        let _src_relative_max_y: f32 =
                            source_position.y + source_size.y / texture_height as f32;
                        */

                        // Transform matrix
                        let transformation =
                            translation_mat * rotation_mat * origin_translation_mat * scale_mat;

                        // Calculate the position vectors
                        let vec_a = transformation * BASE_VEC_A;
                        let vec_b = transformation * BASE_VEC_B;
                        let vec_c = transformation * BASE_VEC_C;
                        let vec_d = transformation * BASE_VEC_D;

                        // Create the UV arrays
                        let uv_a = BASE_UV_A;
                        let uv_b = BASE_UV_B;
                        let uv_c = BASE_UV_C;
                        let uv_d = BASE_UV_D;

                        // Calculate the indices
                        let indices = BASELINE_INDICES
                            .iter()
                            .map(|i| *i + (4 * (starting_index as u16 + local_index as u16)))
                            .collect::<Vec<_>>();

                        // Generate the vertices
                        let vertices = vec![
                            Vertex {
                                position: [vec_a.x, vec_a.y, vec_a.z],
                                tex_coords: [uv_a.x, uv_a.y],
                            }, // A
                            Vertex {
                                position: [vec_b.x, vec_b.y, vec_b.z],
                                tex_coords: [uv_b.x, uv_b.y],
                            }, // B
                            Vertex {
                                position: [vec_c.x, vec_c.y, vec_c.z],
                                tex_coords: [uv_c.x, uv_c.y],
                            }, // C
                            Vertex {
                                position: [vec_d.x, vec_d.y, vec_d.z],
                                tex_coords: [uv_d.x, uv_d.y],
                            }, // D
                        ];

                        result_vertices.extend_from_slice(&vertices[..]);
                        result_indices.extend_from_slice(&indices[..]);
                    }

                    from_thread_sender
                        .send((result_vertices, result_indices))
                        .unwrap();
                } else {
                    break;
                }
            }
        });

        Self {
            handle: handle,
            receiver: from_thread_receiver,
            sender: to_thread_sender,
        }
    }

    pub fn send(&self, input: SpriteInputData) -> Result<()> {
        self.sender.send(input)?;
        Ok(())
    }

    pub fn recv(&self) -> Result<SpriteOutputData> {
        Ok(self.receiver.recv()?)
    }
}

impl Drop for SpriteThread {
    fn drop(&mut self) {
        self.send(None).unwrap();
    }
}
