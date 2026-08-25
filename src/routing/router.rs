#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Cache,
    RateLimited,
    Shed,
    Execute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoutingPolicy {
    cache_enabled: bool,
    rate_limit_enabled: bool,
    load_shedding_enabled: bool,
}

impl RoutingPolicy {
    pub const fn new(
        cache_enabled: bool,
        rate_limit_enabled: bool,
        load_shedding_enabled: bool,
    ) -> Self {
        Self {
            cache_enabled,
            rate_limit_enabled,
            load_shedding_enabled,
        }
    }

    pub const fn route(&self, cache_hit: bool, rate_limited: bool, overloaded: bool) -> Route {
        if overloaded && self.load_shedding_enabled {
            return Route::Shed;
        }

        if rate_limited && self.rate_limit_enabled {
            return Route::RateLimited;
        }

        if cache_hit && self.cache_enabled {
            return Route::Cache;
        }

        Route::Execute
    }
}

impl Default for RoutingPolicy {
    fn default() -> Self {
        Self::new(true, true, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prioritizes_load_shedding_when_overloaded() {
        let policy = RoutingPolicy::default();

        assert_eq!(policy.route(true, true, true), Route::Shed);
    }

    #[test]
    fn routes_rate_limited_requests() {
        let policy = RoutingPolicy::default();

        assert_eq!(policy.route(false, true, false), Route::RateLimited);
    }

    #[test]
    fn routes_cache_hits() {
        let policy = RoutingPolicy::default();

        assert_eq!(policy.route(true, false, false), Route::Cache);
    }

    #[test]
    fn routes_misses_to_execution() {
        let policy = RoutingPolicy::default();

        assert_eq!(policy.route(false, false, false), Route::Execute);
    }

    #[test]
    fn disabled_features_fall_back_to_execution() {
        let policy = RoutingPolicy::new(false, false, false);

        assert_eq!(policy.route(true, true, true), Route::Execute);
    }
}
