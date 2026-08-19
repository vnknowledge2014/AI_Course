// filename: src/lib.rs (tests section)

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_catalog() -> HashMap<String, u64> {
        let mut c = HashMap::new();
        c.insert("W1234".into(), 85_000);
        c
    }

    fn valid_order() -> UnvalidatedOrder {
        UnvalidatedOrder {
            order_id: "ORD-TEST".into(),
            customer_email: "test@co.com".into(),
            lines: vec![UnvalidatedOrderLine { product_code: "W1234".into(), quantity: 2 }],
        }
    }

    // --- Validation tests ---
    #[test]
    fn validates_good_order() {
        assert!(validate_order(valid_order()).is_ok());
    }

    #[test]
    fn collects_all_validation_errors() {
        let bad = UnvalidatedOrder {
            order_id: "".into(),
            customer_email: "bad".into(),
            lines: vec![UnvalidatedOrderLine { product_code: "".into(), quantity: 0 }],
        };
        match validate_order(bad) {
            Err(OrderError::Validation(errors)) => assert!(errors.len() >= 3),
            _ => panic!("Expected multiple validation errors"),
        }
    }

    // --- Pricing tests ---
    #[test]
    fn prices_order_correctly() {
        let validated = validate_order(valid_order()).unwrap();
        let priced = price_order(validated, &sample_catalog()).unwrap();
        assert_eq!(priced.subtotal.0, 170_000); // 85k * 2
        assert_eq!(priced.tax.0, 17_000);       // 10%
        assert_eq!(priced.total.0, 187_000);    // sub + tax
    }

    #[test]
    fn unknown_product_fails_pricing() {
        let mut o = valid_order();
        o.lines[0].product_code = "UNKNOWN".into();
        let validated = validate_order(o).unwrap();
        assert!(price_order(validated, &sample_catalog()).is_err());
    }
}

fn main() {}
