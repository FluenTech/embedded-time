use embedded_time::fraction::Fraction;

#[test]
fn mul_integer_by_fraction() {
    assert_eq!(u32::MAX * Fraction::new(3, 5), u32::MAX / 5 * 3);
}

#[test]
fn fraction_mul_fraction() {
    let product = Fraction::new(1_000, 1) * Fraction::new(5, 5);
    assert_eq!(*product.numerator(), 1_000_u32);
    assert_eq!(*product.denominator(), 1_u32);
}

#[test]
fn fraction_div_fraction() {
    let product = Fraction::new(1_000, 1) / Fraction::new(10, 1_000);
    assert_eq!(*product.numerator(), 100_000_u32);
    assert_eq!(*product.denominator(), 1_u32);
}
