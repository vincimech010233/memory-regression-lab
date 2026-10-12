use memory_regression_lab::{Config, summarize};
#[test]
fn reference() {
    let s = summarize(&[1., 2., 3., 4., 5.]).unwrap();
    assert_eq!(s.mean, 3.);
    assert_eq!(s.median, 3.);
    assert!((s.sample_sd - 2.5_f64.sqrt()).abs() < 1e-12);
    assert!(s.approximate_mean_ci95[0] < 3.);
}
#[test]
fn invalid() {
    for x in [&[1., 2.][..], &[1., 2., f64::NAN], &[1., 2., -1.]] {
        assert!(summarize(x).is_err());
    }
}
#[test]
fn constant_and_even() {
    assert_eq!(summarize(&[2., 2., 2., 2.]).unwrap().sample_sd, 0.);
    assert_eq!(summarize(&[1., 2., 3., 4.]).unwrap().median, 2.5);
}
#[test]
fn limits() {
    let mut c: Config = serde_json::from_str(include_str!("../examples/experiment.json")).unwrap();
    assert!(c.validate().is_ok());
    c.repeats = 31;
    assert!(c.validate().is_err());
}
