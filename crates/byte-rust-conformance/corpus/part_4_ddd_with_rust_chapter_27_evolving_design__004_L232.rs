// filename: src/main.rs

// Feature flags = enum variants
#[derive(Debug, Clone, PartialEq)]
enum Feature {
    DarkMode,
    BetaSearch,
    NewCheckout,
    AIRecommendations,
}

#[derive(Debug)]
struct FeatureConfig {
    enabled: Vec<Feature>,
}

impl FeatureConfig {
    fn new(features: Vec<Feature>) -> Self {
        FeatureConfig { enabled: features }
    }

    fn is_enabled(&self, feature: &Feature) -> bool {
        self.enabled.contains(feature)
    }

    // Typed feature check → compiler ensures handling
    fn checkout_flow(&self) -> CheckoutFlow {
        if self.is_enabled(&Feature::NewCheckout) {
            CheckoutFlow::New
        } else {
            CheckoutFlow::Legacy
        }
    }
}

#[derive(Debug)]
enum CheckoutFlow { Legacy, New }

fn render_checkout(flow: &CheckoutFlow, total: u32) -> String {
    match flow {
        CheckoutFlow::Legacy => {
            format!("=== Classic Checkout ===\nTotal: {}đ\n[Pay Now]", total)
        }
        CheckoutFlow::New => {
            format!("╔══ Modern Checkout ══╗\n║ Total: {}đ          ║\n║ [💳 Pay] [QR] [COD] ║\n╚═════════════════════╝", total)
        }
    }
}

fn render_search(config: &FeatureConfig, query: &str) -> String {
    if config.is_enabled(&Feature::BetaSearch) {
        format!("🔍 AI-powered search for '{}' (beta)", query)
    } else {
        format!("Search results for '{}'", query)
    }
}

fn main() {
    // Production config
    let config = FeatureConfig::new(vec![
        Feature::DarkMode,
        Feature::NewCheckout,
    ]);

    println!("{}", render_checkout(&config.checkout_flow(), 500_000));
    println!();
    println!("{}", render_search(&config, "coffee"));
    println!();

    // Beta config
    let beta = FeatureConfig::new(vec![
        Feature::DarkMode,
        Feature::NewCheckout,
        Feature::BetaSearch,
        Feature::AIRecommendations,
    ]);

    println!("{}", render_search(&beta, "coffee"));
}
