//! This module implements serialization and deserialization logic for the [Component] instance.

use crate::Component;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error, Visitor},
};

impl Serialize for dyn Component {
    /// Serialize this component name into the given Serde serializer.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.component_name())
    }
}

/// The [ComponentVisitor] struct is used to deserialize a [Component] from a string.
pub struct ComponentVisitor;

impl<'de> Visitor<'de> for ComponentVisitor {
    type Value = Box<dyn Component>;

    /// Format a message stating what data the component visitor expects to receive.
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string representing a component identifier")
    }

    // The input contains a string.
    fn visit_str<E: Error>(self, name: &str) -> Result<Self::Value, E> {
        super::get_component_by_name(name)
            .ok_or_else(|| E::custom(format!("unknown component name: {}", name)))
    }
}

impl<'de> Deserialize<'de> for Box<dyn Component> {
    /// Deserialize this component from the given Serde deserializer.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(ComponentVisitor)
    }
}
