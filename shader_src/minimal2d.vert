// NOTE: This shader requires being manually compiled to SPIR-V in order to
// avoid having downstream users require building shaderc and compiling the
// shader themselves. If you update this shader, be sure to also re-compile it
// and update `vert.spv`. You can do so using `glslangValidator` with the
// following command: `glslangValidator -V shader.vert`
#version 450

layout(location = 0) in vec2 position;
layout(location = 1) out vec2 v_pos;


void main() {
    v_pos = position.xy;
    gl_Position = vec4(position.x,position.y, 0.0, 1.0);
}