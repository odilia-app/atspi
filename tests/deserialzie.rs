#[cfg(test)]
mod tests {
    use atspi::events::names::ObjectEvents;
    use serde_plain;

    #[test]
    fn serialize_object_variant() {
        assert_eq!(
            serde_plain::to_string(&ObjectEvents::StateChanged)
                .expect("Could not deserialize ObjectEvents::StateChanged"),
            "StateChanged".to_string()
        );
    }
}
