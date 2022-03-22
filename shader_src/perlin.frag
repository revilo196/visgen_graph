// NOTE: This shader requires being manually compiled to SPIR-V in order to
// avoid having downstream users require building shaderc and compiling the
// shader themselves. If you update this shader, be sure to also re-compile it
// and update `frag.spv`. You can do so using `glslangValidator` with the
// following command: `glslangValidator -V shader.frag`
#version 450

layout(location = 0) out vec4 f_color;
layout(location = 1) in vec2 v_pos;

layout(set = 0, binding = 0) uniform Data {
    vec3 color;
    float tx;
    float ty;
    float tz;
    float sx;
    float sy;
    int octave;
} uniforms;

#include "perlin.glsl"

void main() {
    vec3 color = uniforms.color;
    float tx = uniforms.tx + 100;
    float ty = uniforms.ty + 100;
    float tz = uniforms.tz + 100;
    float sx = uniforms.sx;
    float sy = uniforms.sy;
    int octave = uniforms.octave;

    //first octave
    float intensity = 1+(cnoise(vec3(tx + v_pos.x * sx,ty + v_pos.y * sy, tz)));
    //intensity = (intensity);
   for(int i = 1; i < octave; i=i+1){
      float ss = pow(2,i);
      intensity *= 1+(cnoise(vec3(10 + tx*ss + v_pos.x * sx*ss,10 + ty*ss + v_pos.y * sy*ss,tz*ss)))*(1.0/(ss));
    }



    f_color =  vec4(color, 1.0) * intensity*0.5 *intensity*0.5;
}