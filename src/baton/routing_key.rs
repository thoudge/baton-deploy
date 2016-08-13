struct RoutingKey {}

#[cfg(test)]
mod tests {
    use super::parse_routing_key;

    #[test]
    fn test_parse_routing_key_with_app_and_env() {
        let routing_key = "test.production";
        assert_eq!(app, "test")
    }
}
