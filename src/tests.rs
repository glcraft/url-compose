use crate::Url;
#[test]
fn lksgj() {
    let url = Url::new("https://a").unwrap();
    assert_eq!(url.scheme().unwrap(), "https");
}