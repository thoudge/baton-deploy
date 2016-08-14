struct RoutingKey {
    pub app: Option<String>,
    pub environment: Option<String>,
    pub fqdn: Option<String>,
}

impl RoutingKey {
    pub fn parse(routing_key: &str) -> RoutingKey {
        let mut parts = routing_key.splitn(3, '.').map(|i| String::from(i));

        RoutingKey {
            app: parts.next(),
            environment: parts.next(),
            fqdn: parts.next(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RoutingKey;

    #[test]
    fn test_parse_app_from_routing_key() {
        let result = RoutingKey::parse("test.production.example.com");

        assert_eq!(result.app.unwrap(), "test")
    }

    #[test]
    fn test_parse_environment_from_routing_key() {
        let result = RoutingKey::parse("test.production.example.com");

        assert_eq!(result.environment.unwrap(), "production")
    }

    #[test]
    fn test_parse_fqdn_from_routing_key() {
        let result = RoutingKey::parse("test.production.example.com");

        assert_eq!(result.fqdn.unwrap(), "example.com")
    }

    #[test]
    fn test_parse_no_fqdn_from_routing_key() {
        let result = RoutingKey::parse("test.production");

        assert_eq!(result.fqdn, None)
    }
}
