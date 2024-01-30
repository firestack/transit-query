use crate::{
    gtfs_realtime::{CarriageDetails, TripDescriptor, TripUpdate, VehiclePosition},
    gtfs_schedule::{Route, Stop, Trip},
};

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub(crate) enum Vertex<'a> {
    Route(&'a Route),
    Stop(&'a Stop),
    Trip(&'a Trip),
    #[allow(dead_code)]
    TripDescriptor(&'a TripDescriptor),
    TripUpdate(&'a TripUpdate),
    Vehicle(&'a VehiclePosition),
    CarriageDetails(&'a CarriageDetails),
}
