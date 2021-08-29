use embedded_time::fraction::Fraction;
use embedded_time::ConversionError;
use test_case::test_case;

#[test_case((6, 3) => Ok((2,1)) ; "Reduce the fraction if possible")]
#[test_case((6, 0) => Err(ConversionError::DivByZero) ; "A denominator of 0 will fail")]
fn new_reduce(fraction: (u32, u32)) -> Result<(u32, u32), ConversionError> {
    Fraction::new_reduce(fraction.0, fraction.1)
        .map(|fraction| (fraction.numerator(), fraction.denominator()))
}

#[test_case((3, 1) => 3 ; "Returns integer, no truncation required")]
#[test_case((5, 2) => 2 ; "Returns integer, truncation required")]
fn to_integer(fraction: (u32, u32)) -> u32 {
    Fraction::new(fraction.0, fraction.1).to_integer()
}

#[test_case(3 => (3,1) ; "Returns integer as fraction")]
fn from_integer(integer: u32) -> (u32, u32) {
    let fraction = Fraction::from_integer(integer);
    (fraction.numerator(), fraction.denominator())
}

#[test_case(u32::MAX, (3,5) => u32::MAX / 5 * 3 ; "Properly handles potential overflows when possible")]
#[test_case(u32::MAX, (2,1) => panics "")]
fn u32_mul_fraction(integer: u32, fraction: (u32, u32)) -> u32 {
    integer * Fraction::new(fraction.0, fraction.1)
}

#[test_case(u64::MAX, (3,5) => u64::MAX / 5 * 3 ; "Properly handles potential overflows when possible")]
#[test_case(u64::MAX, (2,1) => panics "")]
fn u64_mul_fraction(integer: u64, fraction: (u32, u32)) -> u64 {
    integer * Fraction::new(fraction.0, fraction.1)
}

#[test]
fn fraction_mul_fraction() {
    let product = Fraction::new(1_000, 1) * Fraction::new(5, 5);
    assert_eq!(product.numerator(), 1_000_u32);
    assert_eq!(product.denominator(), 1_u32);
}

#[test_case(12, (4,3) => 9 ; "Returns integer result")]
#[test_case(u32::MAX, (5,3) => u32::MAX / 5 * 3 ; "Properly handles potential overflows when possible")]
#[test_case(u32::MAX, (1,2) => panics "")]
fn u32_div_fraction(integer: u32, fraction: (u32, u32)) -> u32 {
    integer / Fraction::new(fraction.0, fraction.1)
}

#[test_case(12_u64, (4,3) => 9_u64 ; "Returns integer result")]
#[test_case(u64::MAX, (5,3) => u64::MAX / 5 * 3 ; "Properly handles potential overflows when possible")]
#[test_case(u64::MAX, (1,2) => panics "")]
fn u64_div_fraction(integer: u64, fraction: (u32, u32)) -> u64 {
    integer / Fraction::new(fraction.0, fraction.1)
}

#[test]
fn fraction_div_fraction() {
    let product = Fraction::new(1_000, 1) / Fraction::new(10, 1_000);
    assert_eq!(product.numerator(), 100_000_u32);
    assert_eq!(product.denominator(), 1_u32);
}
