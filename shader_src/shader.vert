// NOTE: This shader requires being manually compiled to SPIR-V in order to
// avoid having downstream users require building shaderc and compiling the
// shader themselves. If you update this shader, be sure to also re-compile it
// and update `vert.spv`. You can do so using `glslangValidator` with the
// following command: `glslangValidator -V shader.vert`
#version 450

layout(location = 0) in vec2 position;
layout(location = 1) out vec2 v_pos;


layout(set = 0, binding = 0) uniform Data {
    vec4 color;
    float time;
    mat4 trans;
    float p1;
    float p2;
    float p3;
    float p4;
    float p5;
    float p6;
} uniforms;




void main() {
    vec3 pos = (uniforms.trans*vec4(position,1.0,1.0)).xyz;
    vec4 color = uniforms.color;
    float time = uniforms.time;
    v_pos = pos.xy;
    gl_Position = vec4(pos.x,pos.y, 0.0, 1.0);
}