// ═══ Tests: no mocks needed! Pure functions! ═══
#[cfg(test)]
mod tests {
    use super::domain::*;

    #[test]
    fn tier_levels() {
        assert_eq!(tier_from_total_spent(500_000), "Bronze");
        assert_eq!(tier_from_total_spent(2_000_000), "Silver");
        assert_eq!(tier_from_total_spent(10_000_000), "Gold");
        assert_eq!(tier_from_total_spent(50_000_000), "Platinum");
    }

    #[test]
    fn shipping_heavy_domestic() {
        // 1500g = 3 × 500g surcharges
        assert_eq!(shipping_cost(1500, "domestic"), 30_000 + 15_000);
    }
}

fn main() {}
