### Backend
Features:
* Thin wrapper over `wgpu` (custom wrappers for most types)
* Defining custom pipelines (`wgpu` pipelines packaged for specific purposes)
* Easily binding and uploading resources
* `CoreTexture` which is an abstraction over a texture
* Easily creating `CoreTexture`'s, downloading, uploading and rendering to them
* `CoreResource` trait for easily binding things to a custom pipeline

### Core
#### Shader
`RenderShader` is a wrapper around a `CorePipeline` which declares its inputs and outputs for use inside the `RenderPass`.
When implemented, should return its input and output `CoreResource`'es.
#### Graph ([Reference](https://apoorvaj.io/render-graphs-1/))
`RenderGraph` as a graph of `RenderPass`'es. Each pass consists of a Shader