// NOTE: This shader requires being manually compiled to SPIR-V in order to
// avoid having downstream users require building shaderc and compiling the
// shader themselves. If you update this shader, be sure to also re-compile it
// and update `frag.spv`. You can do so using `glslangValidator` with the
// following command: `glslangValidator -V shader.frag`
#version 450

layout(location = 0) out vec4 f_color;
layout(location = 1) in vec2 v_pos;

layout(set = 0, binding = 0) uniform Data {
    float f0;  //factor a b 
} uniforms;

layout(set =0, binding = 1) uniform texture2D tex1;
layout(set =0, binding = 2) uniform texture2D tex2;
layout(set =0, binding = 3) uniform texture2D mask;
layout(set =0, binding = 4) uniform sampler samp;


void main() {
    float f0 = uniforms.f0;


    vec4 c0 = vec4(1.0, 1.0, 1.0, 1.0);
    vec4 c1 = vec4(texture(sampler2D(tex1, samp), (v_pos+vec2(1,1))*0.5 ));
    vec4 c2 = vec4(texture(sampler2D(tex2, samp), (v_pos+vec2(1,1))*0.5 ));
    vec4 m = vec4(texture(sampler2D(mask, samp), (v_pos+vec2(1,1))*0.5 ));

    f_color = c1*m + c2*(c0-m);
}