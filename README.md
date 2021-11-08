# ledge_engine
## Table of contents
* [General info](#general-info)
* [Technologies](#technologies)
* [Setup](#setup)
* [License](#license)

## General info
This project is a small game framework written in Rust using winit and vulkano. The goal is to get it to a point where it can be used to help develop 2D platformer PvP/PvE games, as a a stretch goal I'd like to make it useful for other game types, including 3D games.
Hopefully if you find this repo you can use it to understand vulkan or vulkano or Rust or something else. Currently the renderer/engine/idk is very early on and needs a lot of work, but I am slowly making progress towards a usable tool. I try to keep it up to date with vulkano and winit to get the best features from them both.

## Resources
These are some resources I found useful when making this project
* https://renderdoc.org/
* https://vulkan-tutorial.com/
* https://vkguide.dev/
* https://vulkan.lunarg.com/doc/view/1.2.189.0/linux/capture_tools.html
* https://github.com/David-DiGioia/vulkan-diagrams
* The vulkano issues and examples page are great.
	
## Technologies
Project is created with:
* Rust: *
* Winit: 0.25
* Vulkano: 0.26
	
## Setup
Hopefully all goes well with:

```
$ git clone <this repo>
$ cd ./ledge-engine-master
$ cargo run --example texture
```

## License
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)