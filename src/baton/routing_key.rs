use std::error::Error;

struct RoutingKey {
    pub app: Option<String>,
    pub environment: Option<String>,
    pub fqdn: Option<String>,
}

impl RoutingKey {
    pub fn parse(routing_key: &str) -> Result<RoutingKey, Box<Error>> {
        let parts: Vec<&str> = routing_key.split('.').collect();
        let count = parts.len();
        match count {
            2 => {
                Ok(RoutingKey {
                    app: Some(parts[0].to_owned()),
                    environment: Some(parts[1].to_owned()),
                    fqdn: None,
                })
            }
            3...99 => {
                Ok(RoutingKey {
                    app: Some(parts[0].to_owned()),
                    environment: Some(parts[1].to_owned()),
                    fqdn: Some(parts[2..count].join(".").to_owned()),
                })
            }
            _ => panic!("Something strange happend with the pattern matching"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RoutingKey;

    #[test]
    fn test_parse_routing_key_with_app_and_env() {
        let routing_key = "test.production";
        let result = RoutingKey::parse(routing_key).unwrap();
        assert_eq!(result.app.unwrap(), "test")
    }

    #[test]
    fn test_parse_routing_key_with_app_env_fqdn() {
        let routing_key = "test.production.test.example.com";
        let result = RoutingKey::parse(routing_key).unwrap();
        assert_eq!(result.fqdn.unwrap(), "test.example.com")
    }

    #[test]
    fn test_parse_routing_key_with_app_env_long_fqdn() {
        let routing_key = "test.production.test.prod.aws.example.com";
        let result = RoutingKey::parse(routing_key).unwrap();
        assert_eq!(result.fqdn.unwrap(), "test.prod.aws.example.com")
    }
}
