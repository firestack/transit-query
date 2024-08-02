use std::path::Path;

use trustfall::provider::check_adapter_invariants;

use crate::deserialize_feed;

use super::Adapter;

use gtfs_schedule_types::Dataset;

#[test]
fn adapter_satisfies_trustfall_invariants() {
    let vehicles = deserialize_feed(include_str!("../../test_data/VehiclePositions.json"));
    let trips = deserialize_feed(include_str!("../../test_data/TripUpdates.json"));
    let schedule = Dataset::read_from_path(Path::new(
        &std::env::var_os("GTFS_STATIC").expect("GTFS_STATIC env var should be present"),
    )).unwrap();

    let adapter = Adapter::new(&vehicles, &trips, &schedule);
    let schema = Adapter::schema();
    check_adapter_invariants(schema, adapter);
}
