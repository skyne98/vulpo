### To-Do
* Move vertex and index data to the pipeline definition
* Create a `CoreWindow` implementation, which does all the necessary `winit` setup
* Make every pipeline have its own Vertex type/implementation
* __(GLOBAL CAMERA TRANSFORMATION?)__
* Make a global camera/view transformation matrix and transform in shader (should solve the strange size/stretch issue)

### Done
* Resolve on how to pass resources to the bind group builder (cannot just ask for a trait)
* Make the sprite example work