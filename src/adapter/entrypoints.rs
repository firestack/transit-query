use trustfall::provider::{ResolveInfo, VertexIterator};

use crate::gtfs_realtime::VehiclePositions;

use super::vertex::Vertex;

pub(super) fn vehicle<'a>(
    message: &'a VehiclePositions,
    _resolve_info: &ResolveInfo,
) -> VertexIterator<'a, Vertex<'a>> {
    Box::new(
        message
            .entity
            .iter()
            .map(|entity| Vertex::Vehicle(&entity.vehicle)),
    )
}
