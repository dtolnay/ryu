extern crate ryu;

#[test]
fn test_ryu() {
    let cases = vec![
        (0.3, "3E-1"),
        (1.234e20f64, "1.234E20"),
        (1.234e21f64, "1.234E21"),
        (2.71828f64, "2.71828E0"),
        (0.0f64, "0E0"),
        (-0.0f64, "-0E0"),
        (1.1e128f64, "1.1E128"),
        (1.1e-64f64, "1.1E-64"),
        (2.718281828459045f64, "2.718281828459045E0"),
        (5e-324f64, "5E-324"),
        (::std::f64::MAX, "1.7976931348623157E308"),
    ];
    for (f, s) in cases {
        let mut bytes = [0u8; 24];
        let n = unsafe { ryu::d2s_buffered_n(f, &mut bytes[0]) };
        assert_eq!(s, std::str::from_utf8(&bytes[..n]).unwrap());
    }
}
