// sprite.vert
#version 450

layout(set = 0, binding = 0) uniform Globals {
    mat4 ortho;
    mat4 transform;
} global;

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_tex_coords;

layout(location=0) out vec2 v_tex_coords;

void main() {
    v_tex_coords = a_tex_coords;

    gl_Position = global.ortho * global.transform * vec4(a_position, 1.0);
}
