// NOTE: This shader requires being manually compiled to SPIR-V in order to
// avoid having downstream users require building shaderc and compiling the
// shader themselves. If you update this shader, be sure to also re-compile it
// and update `vert.spv`. You can do so using `glslangValidator` with the
// following command: `glslangValidator -V shader.vert`
#version 450

layout(location = 0) in vec2 position;
layout(location = 1) out vec2 f_pos;
layout(location = 2) out vec3 v_col;

layout(set = 0, binding = 0) uniform Data {
    vec4 color;
    float time;
    mat4 trans;
} uniforms;

void main() {
    float time = uniforms.time;
    vec4 color = uniforms.color;
    f_pos = position;
    v_col = color.xyz;
    gl_Position = uniforms.trans * vec4(position, 0.0, 1.0) ;
}