pub use wgpu_procs::*;

pub trait VertexLayout {
  const LAYOUT: wgpu::VertexBufferLayout<'static>;
}
