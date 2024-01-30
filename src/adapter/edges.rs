use trustfall::provider::{
    AsVertex, ContextIterator, ContextOutcomeIterator, EdgeParameters, ResolveEdgeInfo,
    VertexIterator,
};

use crate::{gtfs_realtime::VehiclePositions, gtfs_schedule::GtfsSchedule};

use super::vertex::Vertex;

pub(super) fn resolve_trip_edge<'a, V: AsVertex<Vertex<'a>> + 'a>(
    vehicles: &'a VehiclePositions,
    schedule: &'a GtfsSchedule,
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex<'a>>> {
    match edge_name {
        "route" => trip::route(&schedule.routes, contexts, resolve_info),
        "vehicles" => trip::vehicle(vehicles, contexts, resolve_info),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Trip'")
        }
    }
}

mod trip {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use crate::{
        gtfs_realtime::VehiclePositions,
        gtfs_schedule::{Route, Trip},
    };

    use super::super::vertex::Vertex;

    pub(super) fn route<'a, V: AsVertex<Vertex<'a>> + 'a>(
        routes: &'a [Route],
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex<'a>>> {
        resolve_neighbors_with(contexts, |vertex| {
            let vertex = vertex
                .as_trip()
                .expect("conversion failed, vertex was not a Trip");
            let route_id = &vertex.route_id;
            let matching_routes = routes.iter().filter_map(move |vertex| {
                if &vertex.route_id == route_id {
                    Some(Vertex::Route(vertex))
                } else {
                    None
                }
            });
            Box::new(matching_routes)
        })
    }

    pub(super) fn vehicle<'a, V: AsVertex<Vertex<'a>> + 'a>(
        vehicles: &'a VehiclePositions,
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex<'a>>> {
        resolve_neighbors_with(contexts, |vertex| {
            let Trip { trip_id, .. } = &vertex
                .as_trip()
                .expect("conversion failed, vertex was not a Trip");

            let matching_vehicles = vehicles
                .entity
                .iter()
                .filter_map(move |vertex| match &vertex.vehicle.trip {
                    Some(vehicle_trip) if vehicle_trip.trip_id.eq(trip_id) => {
                        Some(Vertex::Vehicle(&vertex.vehicle))
                    }
                    _ => None,
                });

            Box::new(matching_vehicles)
        })
    }
}

pub(super) fn resolve_vehicle_edge<'a, V: AsVertex<Vertex<'a>> + 'a>(
    schedule: &'a GtfsSchedule,
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex<'a>>> {
    match edge_name {
        "stop" => vehicle::stop(&schedule.stops, contexts, resolve_info),
        "trip" => vehicle::trip(&schedule.trips, contexts, resolve_info),
        "trip_descriptor" => vehicle::trip(&schedule.trips, contexts, resolve_info),
        "multi_carriage_details" => vehicle::multi_carriage_details(contexts, resolve_info),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Vehicle'")
        }
    }
}

mod vehicle {

    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use crate::{
        gtfs_realtime::VehiclePosition,
        gtfs_schedule::{Stop, Trip},
    };

    use super::super::vertex::Vertex;

    pub(super) fn stop<'a, V: AsVertex<Vertex<'a>> + 'a>(
        stops: &'a [Stop],
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex<'a>>> {
        resolve_neighbors_with(contexts, |vertex| {
            let vertex = vertex
                .as_vehicle()
                .expect("conversion failed, vertex was not a Vehicle");
            if let Some(stop_id) = &vertex.stop_id {
                let matching_stops = stops.iter().filter_map(move |vertex| {
                    if &vertex.stop_id == stop_id {
                        Some(Vertex::Stop(vertex))
                    } else {
                        None
                    }
                });
                Box::new(matching_stops)
            } else {
                Box::new(std::iter::empty())
            }
        })
    }

    pub(super) fn trip<'a, V: AsVertex<Vertex<'a>> + 'a>(
        trips: &'a [Trip],
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex<'a>>> {
        resolve_neighbors_with(contexts, |vertex| {
            let vertex: &VehiclePosition = vertex
                .as_vehicle()
                .expect("conversion failed, vertex was not a Vehicle");
            if let Some(trip_id) = vertex.trip.as_ref().map(|trip| &trip.trip_id) {
                let matching_stops = trips.iter().filter_map(move |vertex| {
                    if &vertex.trip_id == trip_id {
                        Some(Vertex::Trip(vertex))
                    } else {
                        None
                    }
                });
                Box::new(matching_stops)
            } else {
                Box::new(std::iter::empty())
            }
        })
    }

    pub(super) fn multi_carriage_details<'a, V: AsVertex<Vertex<'a>> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex<'a>>> {
        resolve_neighbors_with(contexts, |vertex| {
            let vertex: &VehiclePosition = vertex
                .as_vehicle()
                .expect("conversion failed, vertex was not a Vehicle");

            if let Some(carriage_details) = &vertex.multi_carriage_details {
                return Box::new(carriage_details.iter().map(Vertex::CarriageDetails));
            } else {
                Box::new(std::iter::empty())
            }
        })
    }
}
