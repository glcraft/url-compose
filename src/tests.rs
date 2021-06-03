use crate::Url;
#[test]
fn url_scheme() {
    let url = Url::new("https://a").unwrap();
    assert_eq!(url.scheme().unwrap(), "https");
}