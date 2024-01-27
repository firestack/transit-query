mod adapter_impl;
mod edges;
mod entrypoints;
mod properties;
mod vertex;

#[cfg(test)]
mod tests;

pub(crate) use adapter_impl::Adapter;
#[allow(unused_imports)]
pub(crate) use vertex::Vertex;
