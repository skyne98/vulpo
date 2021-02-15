##12.02.2021
### Done
* Resolve on how to pass resources to the bind group builder (cannot just ask for a trait)
* Make the sprite example work
* Fix bug with multiple sprites on the screen (current sprite example) - try still use indices
* Create a `Window` implementation, which does all the necessary `winit` setup
* Move vertex and index data to the pipeline definition
* Revert everything to a working state after upgrade
* Give pipeline size information from the very beginning

##15.02.2021
### To-do
* Use proper UV calculations
* Make a global camera/view transformation matrix and transform in shader
* Make every pipeline have its own `Vertex` type/implementation
* Create simple benchmarks for sprite vertex generation time, resize time and frame rendering time
* Support color-tinting sprites
* Support giving specific sprites depth
* Reduce code repetition in pipelines
* Support multiple textures in the sprite pipeline (texture_index)
* Create bindings using reflection (via `spirq` crate)
* Support SVG's and other vector graphics using the `lyon` crate
* Put all the specific renderers into their own features (texture, sprite, vector)

### Done
