// NOTE: This shader requires being manually compiled to SPIR-V in order to
// avoid having downstream users require building shaderc and compiling the
// shader themselves. If you update this shader, be sure to also re-compile it
// and update `frag.spv`. You can do so using `glslangValidator` with the
// following command: `glslangValidator -V shader.frag`
#version 450

layout(location = 0) out vec4 f_color;
layout(location = 1) in vec2 v_pos;

layout(set = 0, binding = 0) uniform Data {
    vec4 color;
    float time;
    float freq;
    float hard;
    float duty;
    float angle;
} uniforms;

void main() {
    vec4 color = uniforms.color;
    float time = uniforms.time;
    float freq = uniforms.freq;
    float angle = uniforms.angle;

    float phase = (v_pos.x * sin(angle) + v_pos.y * cos(angle)) * freq * 20.14  + time;
    float intensity =  (sin( phase ) + 1.0)/2.0;

    //duty
    intensity = pow(intensity, 6);
    
    intensity = (intensity*2)-1;

    //hardniss
    intensity = sin(intensity * 5 / 3.14);
    intensity = sin(intensity * 5 / 3.14);
    intensity = sin(intensity * 5 / 3.14);
    intensity = sin(intensity * 5 / 3.14);

    intensity = (intensity+1)/2; 


    f_color =  color * intensity;
}