pub mod conceptual {
    /// Defines an injectable strategy for building routes.
    trait RouteStrategy {
        fn build_route(&self, from: &str, to: &str);
    }

    struct WalkingStrategy;

    impl RouteStrategy for WalkingStrategy {
        fn build_route(&self, from: &str, to: &str) {
            println!("Walking route from {} to {}: 4km, 30", from, to);
        }
    }

    struct PublicTransportStrategy;

    impl RouteStrategy for PublicTransportStrategy {
        fn build_route(&self, from: &str, to: &str) {
            println!(
                "Public transport route from {} to {}: 3km, 5 min",
                from, to
            )
        }
    }

    struct Navigator<T> {
        route_strategy: T,
    }

    impl<T: RouteStrategy> Navigator<T>  {
        fn new(route_strategy: T) -> Self {
            Self { route_strategy }
        }

        fn route(&self, from: &str, to: &str) {
            self.route_strategy.build_route(from, to)
        }
    }

    pub fn conceptual_strategy_main() {
        let navigator = Navigator::new(WalkingStrategy);
        navigator.route("Home", "Club");
        navigator.route("Club", "Work");

        let navigator = Navigator::new(PublicTransportStrategy);
        navigator.route("Home", "Club");
        navigator.route("Club", "Work");
    }
}

pub mod functional {
    type RouteStrategy = fn(From: &str, to: &str);

    fn walking_strategy(from: &str, to: &str) {
        println!("Walking route from {} to {}: 4km, 30", from, to);
    }

    fn public_transport_strategy(from: &str, to: &str) {
        println!( "Public transport route from {} to {}: 3km, 5 min", from, to);
    }

    struct Navigator {
        route_strategy: RouteStrategy,
    }

    impl Navigator  {
        fn new(route_strategy: RouteStrategy) -> Self {
            Self { route_strategy }
        }

        fn route(&self, from: &str, to: &str) {
            (self.route_strategy)(from, to);
        }
    }

    pub fn functional_strategy_main() {
           let navigator = Navigator::new(walking_strategy);
        navigator.route("Home", "Club");
        navigator.route("Club", "Work");

        let navigator = Navigator::new(public_transport_strategy);
        navigator.route("Home", "Club");
        navigator.route("Club", "Work");

         let navigator = Navigator::new(|from, to| println!("Specific route from {} to {}", from, to));
        navigator.route("Home", "Club");
        navigator.route("Club", "Work");
    }

}