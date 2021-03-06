// NOTE: This shader requires being manually compiled to SPIR-V in order to
// avoid having downstream users require building shaderc and compiling the
// shader themselves. If you update this shader, be sure to also re-compile it
// and update `frag.spv`. You can do so using `glslangValidator` with the
// following command: `glslangValidator -V shader.frag`
#version 450

layout(location = 0) out vec4 f_color;
layout(location = 1) in vec2 p_pos;
layout(location = 2) in vec3 v_col;

layout(set = 0, binding = 0) uniform Data {
    vec4 color;
    float time;
    mat4 trans;
} uniforms;

void main() {
    vec4 color = uniforms.color;
    float time = uniforms.time;
    f_color =  color * abs(vec4(sin(time), sin(time/2.0), sin(time/3.0), 1.0));
}