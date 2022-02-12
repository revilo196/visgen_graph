// NOTE: This shader requires being manually compiled to SPIR-V in order to
// avoid having downstream users require building shaderc and compiling the
// shader themselves. If you update this shader, be sure to also re-compile it
// and update `frag.spv`. You can do so using `glslangValidator` with the
// following command: `glslangValidator -V shader.frag`
#version 450

layout(location = 0) out vec4 f_color;
layout(location = 1) in vec2 v_pos;

layout(set = 0, binding = 0) uniform Data {
    float f0;  //factor offset
    float f1;  //factor tex1 
    float f2;  //factor tex2
    float f12; //factor tex1 and tex2 (c1*c2 / Multiply / Mask) 
    float fi1; //factor inverse tex1 (1-c1)
    float fi2; //factor inverse tex2 (1-c2)
    float f1i2;//factor tex1 and inverse tex2 (c1*(1-c2) / Multiply / Mask) 
    float fi12;//factor inverse tex1 and tex2 ((1-c1)*c2 / Multiply / Mask) 
    float fi1i2;//factor inverse tex1 and inverse tex2 ((1-c1)*(1-c2) / Multiply / Mask)
} uniforms;

layout(set =0, binding = 1) uniform texture2D tex1;
layout(set =0, binding = 2) uniform texture2D tex2;
layout(set =0, binding = 3) uniform sampler samp;


void main() {
    float f0 = uniforms.f0;
    float f1 = uniforms.f1;
    float f2 = uniforms.f2;
    float f12 = uniforms.f12;
    float fi1 = uniforms.fi1;
    float fi2 = uniforms.fi2;
    float f1i2 = uniforms.f1i2;
    float fi12 = uniforms.fi12;
    float fi1i2 = uniforms.fi1i2;

    vec4 c0 = vec4(1.0, 1.0, 1.0, 1.0);
    vec4 c1 = vec4(texture(sampler2D(tex1, samp), (v_pos+vec2(1,1))*0.5 ));
    vec4 c2 = vec4(texture(sampler2D(tex2, samp), (v_pos+vec2(1,1))*0.5 ));
    vec4 ci1 = c0 - c1;
    vec4 ci2 = c0 - c2;

    f_color = c0*f0 + c1*f1 + c2*f2 + c1*c2*f12 + ci1*fi1 + ci2*fi2 + c1*ci2*f1i2 + ci1*c2*fi12 + ci1*ci2*fi1i2;
    
    // fully expanded  and simplified version
    //f_color = -c2*fi2 + c0*fi2 + c1*c2*fi1i2 - c2*fi1i2 - c1*fi1i2 + c0*fi1i2 - c1*c2*fi12 + c2*fi12 - c1*fi1 + c0*fi1 + c2*f2 - c1*c2*f1i2 + c1*f1i2 + c1*c2*f12 + c1*f1 + c0*f0;
}