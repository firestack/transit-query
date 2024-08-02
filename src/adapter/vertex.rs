use crate::gtfs_realtime::{CarriageDetails, TripDescriptor, VehiclePosition};

use gtfs_schedule_types::records::{Routes, Stops, Trips};

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub(crate) enum Vertex<'a> {
    Route(&'a Routes),
    Stop(&'a Stops),
    Trip(&'a Trips),
    #[allow(dead_code)]
    TripDescriptor(&'a TripDescriptor),
    Vehicle(&'a VehiclePosition),
    CarriageDetails(&'a CarriageDetails),
}
