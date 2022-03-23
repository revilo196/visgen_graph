// NOTE: This shader requires being manually compiled to SPIR-V in order to
// avoid having downstream users require building shaderc and compiling the
// shader themselves. If you update this shader, be sure to also re-compile it
// and update `frag.spv`. You can do so using `glslangValidator` with the
// following command: `glslangValidator -V shader.frag`
#version 450

layout(location = 0) out vec4 f_color;
layout(location = 1) in vec2 v_pos;

layout(set = 0, binding = 0) uniform Data {
    vec3 c0;
    float f0;
    vec3 c1;
    float f1;
    vec3 c2;
    float f2;
    int mode;
} uniforms;

layout(set =0, binding = 1) uniform texture2D tex1;
layout(set =0, binding = 2) uniform sampler samp;

float h1(float x) {
  return -2*x*x*x+3*x*x;
}
float h2(float x) {
 return h1(h1(x));
}
float h4(float x){
  return h2(h2(x));
}

void main() {
    vec3 c0 = uniforms.c0;
    vec3 c1 = uniforms.c1;
    vec3 c2 = uniforms.c2;
    float f0 = uniforms.f0;
    float f1 = uniforms.f1;
    float f2 = uniforms.f2;
    int mode = uniforms.mode;

    float t = length(vec4(texture(sampler2D(tex1, samp), (v_pos+vec2(1,1))*0.5 )));
    //t = clamp(t, 0.0, 1.0);

      if (t < f0) {
        f_color = vec4(c0,1.0);
      } else if (t < f1) {

        switch(mode) {
          case 0:
            f_color = vec4(mix(c0,c1, (t-f0)/(f1-f0)), 1.0); //mode2 linear
            break;
          case 1:
            f_color = vec4(mix(c0,c1, h1((t-f0)/(f1-f0))), 1.0); //mode3 h1
            break;
          case 2:
            f_color = vec4(mix(c0,c1, h2((t-f0)/(f1-f0))), 1.0); //mode4 h2
            break;
          case 3:
            f_color = vec4(mix(c0,c1, h4((t-f0)/(f1-f0))), 1.0); //mode4 h4
          case 4:
          default:
            f_color = vec4(c1,1.0); // mode1 hard
            break;
        }

      } else if (t < f2) {
        switch(mode) {
          case 0:
            f_color = vec4(mix(c1,c2, (t-f1)/(f2-f1)), 1.0); //mode2 linear
            break;
          case 1:
            f_color = vec4(mix(c1,c2, h1((t-f1)/(f2-f1))), 1.0); //mode3 h1
            break;
          case 2:
            f_color = vec4(mix(c1,c2, h2((t-f1)/(f2-f1))), 1.0); //mode4 h2
            break;
          case 3:
            f_color = vec4(mix(c1,c2, h4((t-f1)/(f2-f1))), 1.0); //mode4 h4
          case 4:
          default:
            f_color = vec4(c2,1.0); // mode1 hard
            break;
        }


      } else {
        f_color = vec4(c2,1.0);
      }
}

