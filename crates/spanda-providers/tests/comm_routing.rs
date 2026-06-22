//! Provider registry comm-bus routing tests.
//!
use spanda_comm::TransportKind;
use spanda_providers::{bootstrap_providers_for_packages, sync_comm_bus_for_official_packages};
use spanda_transport_routing::RoutingCommBus;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn sync_comm_bus_routes_mqtt_through_provider_registry() {
    let registry = Rc::new(RefCell::new(bootstrap_providers_for_packages(&[
        "spanda-mqtt",
    ])));
    let mut comm_bus = RoutingCommBus::new();
    comm_bus.attach_provider_registry(Rc::clone(&registry));
    sync_comm_bus_for_official_packages(&mut comm_bus, &mut registry.borrow_mut());
    assert!(comm_bus.is_registry_backed(TransportKind::Mqtt));
    assert!(!comm_bus.is_registry_backed(TransportKind::Ros2));
}

#[test]
fn sync_comm_bus_routes_ros2_when_official_package_installed() {
    let registry = Rc::new(RefCell::new(bootstrap_providers_for_packages(&[
        "spanda-ros2",
    ])));
    let mut comm_bus = RoutingCommBus::new();
    comm_bus.attach_provider_registry(Rc::clone(&registry));
    sync_comm_bus_for_official_packages(&mut comm_bus, &mut registry.borrow_mut());
    assert!(comm_bus.is_registry_backed(TransportKind::Ros2));
}
