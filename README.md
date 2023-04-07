# visgen_graph

visgen_graph is a creative coding project that generates controllable background visuals for live projection using a user-defined graph of texture processors. The project is built using the **nannou** framework for creative coding in Rust, and provides a video stream over NDI that can be used inside other projects.

## Features:
 - [x] Tree Based Rendering Textures Graph
 - [x] OSC-control with global parameter store
 - [x] Video output over NDI
 - [x] Simple base for Rendering 2D/3D with shaders
 - [x] Texture combination blending + masking
 - [x] First Steps to create Scene Programmes

### Texture Generators:
 - [x] simple circles
 - [x] simple stripes
 - [x] wave texture
 - [x] path-tracing clouds (gpu low framerate)
 - [x] perlin noise texture

### Texture Modifiers:
 - [x] simple Addition of 2 Textures
 - [x] Blend/Fade Combining of 2 Textures
 - [x] Masking Combine 2 Texture using a 3rd Texture as Mask

### Texture EFX:
 - [x] bw to color using a color ramp (like the blender node)

### Open Stage Control
Open Stage Control can used as an interface to provide all the OSC parameters.
Examples projects and fragments are included inside the tool path.

# Architecture

visgen_graph uses a Tree of TextureNode's to represent the user-defined graph of textures. The root of the tree is the output node, while the leaves of the tree are generators that generate a texture without an input texture. The project also defines models and targets that help set up different types of rendering, such as Shader2DTarget for rendering 2D graphics using a shader.

OSC receiving is handled by `nannou_osc`, which uses `rosc`. Each node defines its own parameters that can be received via OSC messages. All parameters are stored in a global ParameterStorage.

For NDI output, visgen_graph uses a modified version of `ndi-rs`.

# Ideas
- new speed implementation, to make changing speed, not changing the position
- rework program parameter storage. (order independent, only store changed parameters)

## Shadertoy Ideas:
- [shadertoy](https://www.shadertoy.com/view/3l23Rh)
- [shadertoy](https://www.shadertoy.com/view/XsX3zl)
- [shadertoy](https://www.shadertoy.com/view/tdG3Rd)
- [shadertoy](https://www.shadertoy.com/view/sdSyDW)
- [shadertoy](https://www.shadertoy.com/view/4dfGzs)
- [shadertoy](https://www.shadertoy.com/view/fdsfRH)
- [shadertoy](https://www.shadertoy.com/view/NdfBzn)
- [shadertoy](https://www.shadertoy.com/view/7lKSWW)
- [shadertoy](https://www.shadertoy.com/view/sdSyDW)
- [shadertoy](https://www.shadertoy.com/view/XlfGRj)
- [shadertoy](https://www.shadertoy.com/view/4slGz4)
- [shadertoy](https://www.shadertoy.com/view/MslGWN)
- [shadertoy](https://www.shadertoy.com/view/XtGGRt)
- [shadertoy](https://www.shadertoy.com/view/XlB3zV)
- [shadertoy](https://www.shadertoy.com/view/4sl3Dr)
- [shadertoy](https://www.shadertoy.com/view/4dl3zn)
- [youtue](https://youtu.be/9NKeyTjwre0?t=85)

### Filter
- [shadertoy Egde Glow](https://www.shadertoy.com/view/Mdf3zr)


### nice to have
- [ ] load and store NodeGraph
- [ ] tool for generating open-stage-control interface/ fragments for some NodeGraph
    - see revilo196/ofVisualGenerator

### creative 
- more visuals
- efx Glow, Blur...

### graph tool
some tool is needed to generate the node tree, look for some UI or application frameworks to help with that. like [gazpatcho](https://github.com/zlosynth/gazpatcho/)



## Getting Started

As visgen_graph is still in early progress, there is not much a user can do without detailed knowledge of the inner workings of the software. However, if you are interested in contributing to the project or expanding upon the existing functionality, here are some steps to get started:

   - Clone the project from the repository
   - Install Rust and the required dependencies (see `Cargo.toml`)
   - Build and run the project using `cargo run`

## Known Issues

As visgen_graph is still in development, there may be some limitations or known issues to keep in mind. For example, there may be performance considerations when rendering a large number of texture nodes in real-time without dropping frames.

## Contributions

If you are interested in contributing to visgen_graph, please feel free to submit a pull request or open an issue on the repository. We welcome any contributions that can help improve the project's functionality or usability.

Im also interested on any ideas how to improve the projects. Im also open to feedback on the current architecture.

## License

visgen_graph is licensed under the MIT License. See LICENSE for more information.

## Credits

visgen_graph was created by Oliver Walter. Thank you to the developers of `nannou`, `rosc`, `ndi-rs`, and other dependencies used in this project.

