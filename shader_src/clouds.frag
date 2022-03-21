#version 450

// 0: no LOD
// 1: yes LOD
#define USE_LOD 1

layout(location = 0) out vec4 f_color;
layout(location = 1) in vec2 v_pos;

layout(set = 0, binding = 0) uniform Data {
    float iTime;  //factor offset
} uniforms;

#include "perlin.glsl"

vec4 rand(vec2 co){
    float f1 = fract(sin(dot(co, vec2(12.9898, 78.233))) * 43758.5453);
    float f2 = fract(sin(dot(co+vec2(50,50), vec2(12.9898, 78.233))) * 43758.5453);
    float f3 = fract(sin(dot(co+vec2(150,50), vec2(12.9898, 78.233))) * 43758.5453);
    float f4 = fract(sin(dot(co+vec2(50,150), vec2(12.9898, 78.233))) * 43758.5453);

    return vec4(f1,f2,f3,f4);
}

float noise( in vec3 x )
{
    /*vec3 p = floor(x);
    vec3 f = fract(x);
	f = f*f*(3.0-2.0*f);

	vec2 uv = (p.xy+vec2(37.0,239.0)*p.z) + f.xy;
    vec2 rg = vec2 (cnoise(vec3(uv.x*3, uv.y*3, 0.0)), cnoise(vec3(uv.x*3, uv.y*3, 0.3)));
	return mix( rg.x, rg.y, f.z )*2.0-1.0;  
  */
  return cnoise(x)*1.5;
}

float map( in vec3 p, int oct )
{
	vec3 q = p - vec3(0.0,0.1,1.0)*uniforms.iTime;
    float g = 0.5+0.5*noise( q*0.3 );
    
	float f;
    f  = 0.50000*noise( q ); q = q*2.02;
    #if USE_LOD==1
    if( oct>=2 ) 
    #endif
    f += 0.25000*noise( q ); q = q*2.23;
    #if USE_LOD==1
    if( oct>=3 )
    #endif
    f += 0.12500*noise( q ); q = q*2.41;
    #if USE_LOD==1
    if( oct>=4 )
    #endif
    f += 0.06250*noise( q ); q = q*2.62;
    #if USE_LOD==1
    if( oct>=5 )
    #endif
    f += 0.03125*noise( q ); 
    
    f = mix( f*0.1-0.75, f, g*g ) + 0.1;
    return 1.5*f - 0.5 - p.y;
}

const vec3 sundir = normalize( vec3(-1.0,0.0,-1.0) );
const int kDiv = 1; // make bigger for higher quality

vec4 raymarch( in vec3 ro, in vec3 rd, in vec3 bgcol /*, in ivec2 px*/ )
{
    // bounding planes	
    const float yb = -3.0;
    const float yt =  0.6;
    float tb = (yb-ro.y)/rd.y;
    float tt = (yt-ro.y)/rd.t;

    // find tigthest possible raymarching segment
    float tmin, tmax;
    if( ro.y>yt )
    {
        // above top plane
        if( tt<0.0 ) return vec4(0.0); // early exit
        tmin = tt;
        tmax = tb;
    }
    else
    {
        // inside clouds slabs
        tmin = 0.0;
        tmax = 60.0;
        if( tt>0.0 ) tmax = min( tmax, tt );
        if( tb>0.0 ) tmax = min( tmax, tb );
    }
    
    // dithered near distance
    float t = 0.8; // tmin + 0.1*texelFetch( iChannel1, px&1023, 0 ).x;
    
    // raymarch loop
	vec4 sum = vec4(0.0);
    for( int i=0; i<190*kDiv; i++ )
    {
       // step size
       float dt = max(0.05,0.02*t/float(kDiv));

       // lod
       #if USE_LOD==0
       const int oct = 5;
       #else
       int oct = 5 - int( log2(1.0+t*0.5) );
       #endif
       
       // sample cloud
       vec3 pos = ro + t*rd;
       float den = map( pos,oct );
       if( den>0.01 ) // if inside
       {
           // do lighting
           float dif = clamp((den - map(pos+0.3*sundir,oct))/0.3, 0.0, 1.0 );
           vec3  lin = vec3(0.65,0.65,0.75)*1.1 + 0.8*vec3(1.0,0.6,0.3)*dif;
           vec4  col = vec4( mix( vec3(1.0,0.95,0.8), vec3(0.25,0.3,0.35), den ), den );
           col.xyz *= lin;
           // fog
           col.xyz = mix(col.xyz,bgcol, 1.0-exp2(-0.075*t));
           // composite front to back
           col.w    = min(col.w*8.0*dt,1.0);
           col.rgb *= col.a;
           sum += col*(1.0-sum.a);
       }
       // advance ray
       t += dt;
       // until far clip or full opacity
       if( t>tmax || sum.a>0.99 ) break;
    }

    return clamp( sum, 0.0, 1.0 );
}

mat3 setCamera( in vec3 ro, in vec3 ta, float cr )
{
	vec3 cw = normalize(ta-ro);
	vec3 cp = vec3(sin(cr), cos(cr),0.0);
	vec3 cu = normalize( cross(cw,cp) );
	vec3 cv = normalize( cross(cu,cw) );
    return mat3( cu, cv, cw );
}

vec4 render( in vec3 ro, in vec3 rd/*, in ivec2 px*/ )
{
	float sun = clamp( dot(sundir,rd), 0.0, 1.0 );

    // background sky
    vec3 col = vec3(0.76,0.75,0.86);
    col -= 0.6*vec3(0.90,0.75,0.95)*rd.y;
	col += 0.2*vec3(1.00,0.60,0.10)*pow( sun, 8.0 );

    // clouds    
    vec4 res = raymarch( ro, rd, col/*, px */);
    col = col*(1.0-res.w) + res.xyz;
    
    // sun glare    
	col += 0.2*vec3(1.0,0.4,0.2)*pow( sun, 3.0 );

    // tonemap
    col = smoothstep(0.15,1.1,col);
 
    return vec4( col, 1.0 );
}

void main()
{
    vec2 p = v_pos;
    vec2 m = vec2(sin(uniforms.iTime*0.8),sin(uniforms.iTime*0.5));

    // camera
    vec3 ro = 4.0*normalize(vec3(sin(3.0*m.x), 0.8*m.y, cos(3.0*m.x))) - vec3(0.0,0.1,0.0);
	vec3 ta = vec3(0.0, -1.0, 0.0);
    mat3 ca = setCamera( ro, ta, 0.07*cos(0.25*uniforms.iTime) );
    // ray
    vec3 rd = ca * normalize( vec3(p.xy,1.5));
    
    f_color = render( ro, rd /*, ivec2(v_pos*512-0.5)*/ );
}
