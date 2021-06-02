use std::ops::{Range, Deref};
use std::fmt;
use lazy_static::lazy_static;
use regex::Regex;
use crate::regex_generator;

#[derive(Clone)]
pub struct Url<S: AsRef<str>> {
    full: S,
    scheme: Option<Range<usize>>,
    user: Option<Range<usize>>,
    password: Option<Range<usize>>,
    host: Option<Range<usize>>,
    port: Option<Range<usize>>,
    path: Option<Range<usize>>,
    query: Option<Range<usize>>,
    fragment: Option<Range<usize>>,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Query<'a> {
    Field(&'a str),
    Form(&'a str, &'a str),
}
impl<S> Url<S> 
    where S: AsRef<str>
{
    pub fn new(v: S) -> Result<Url<S>, &'static str> {
        lazy_static! {
            static ref URL_REGEX: Regex = {
                Regex::new(regex_generator::url_regex())
                    .expect(r"/!\ Regex error from internal string. /!\")
            };
        }
        let test = v.as_ref();
        let captures: regex::Captures = match URL_REGEX.captures(test) {
            Some(captures) => captures,
            None => return Err("Url not valid"),
        };
        let (scheme, user, password, host, port, path, query, fragment) = {
            let to_opt_range = |i| (&captures).get(i).map(|m| m.range());
            (
                to_opt_range(1),
                to_opt_range(2),
                to_opt_range(3),
                to_opt_range(4),
                to_opt_range(5),
                to_opt_range(6),
                to_opt_range(7),
                to_opt_range(8),
            )
        };
        
        Ok(Url {
            full: v,
            scheme, user, password, host, port, path, query, fragment
        })
    }
    /**
    Returns the complete url.
    */
    pub fn full(&self) -> &str {
        self.full.as_ref()
    }
    fn range_to_str(&self, range: &Option<Range<usize>>) -> Option<&str> {
        range.clone().map(|s| &self.full.as_ref()[s])
    }
    /**
    Returns the scheme of the url if present, [`None`] otherwise.

    ## Example
    ```
    use url_compose::Url;

    let url = Url::new("https://username:password@www.example.com:8080/path/to/file?field=value#fragment").unwrap();
    assert_eq!(url.scheme(), Some("https"));
    let url = Url::new("/path/example").unwrap();
    assert_eq!(url.scheme(), None);
    ```
    */
    pub fn scheme(&self) -> Option<&str>
    {
        self.range_to_str(&self.scheme)
    }
    /**
    Returns the username of the url if present, [`None`] otherwise.

    In a URL (to connect ftp for example), you can pass userinfo as `username:password`.

    ## Example
    ```
    use url_compose::Url;

    let url = Url::new("https://username:password@www.example.com:8080/path/to/file?field=value#fragment").unwrap();
    assert_eq!(url.user(), Some("username"));
    let url = Url::new("https://www.example.com").unwrap();
    assert_eq!(url.user(), None);
    ```
    */
    pub fn user(&self) -> Option<&str>
    {
        self.range_to_str(&self.user)
    }
    /**
    Returns the password of the url if present, [`None`] otherwise.

    In a URL (to connect ftp for example), you can pass userinfo as `username:password`.
    Note that the password in the url is deprecated as the data is displayed in clear. 

    ## Example
    ```
    use url_compose::Url;

    let url = Url::new("https://username:password@www.example.com:8080/path/to/file?field=value#fragment").unwrap();
    assert_eq!(url.password(), Some("password"));
    let url = Url::new("https://www.example.com").unwrap();
    assert_eq!(url.password(), None);
    ```
    */
    pub fn password(&self) -> Option<&str>
    {
        self.range_to_str(&self.password)
    }
    /**
    Returns the host of the url if present, [`None`] otherwise.

    ## Example
    ```
    use url_compose::Url;

    let url = Url::new("https://username:password@www.example.com:8080/path/to/file?field=value#fragment").unwrap();
    assert_eq!(url.host(), Some("www.example.com"));
    let url = Url::new("http://localhost:8080").unwrap();
    assert_eq!(url.host(), Some("localhost"));
    let url = Url::new("/path/example").unwrap();
    assert_eq!(url.host(), None);
    ```
    */
    pub fn host(&self) -> Option<&str>
    {
        self.range_to_str(&self.host)
    }
    /**
    Returns the port of the url if present, [`None`] otherwise.
    
    ## Example
    ```
    use url_compose::Url;

    let url = Url::new("https://username:password@www.example.com:8080/path/to/file?field=value#fragment").unwrap();
    assert_eq!(url.port(), Some("8080"));
    let url = Url::new("/path/example").unwrap();
    assert_eq!(url.port(), None);
    ```
    */
    pub fn port(&self) -> Option<&str>
    {
        self.range_to_str(&self.port)
    }
    /**
    Returns the path of the url.

    Even if the path returns an [`Option`], it will never return [`None`], but can return Some("")
    
    ## Example
    ```
    use url_compose::Url;

    let url = Url::new("https://username:password@www.example.com:8080/path/to/file?field=value#fragment").unwrap();
    assert_eq!(url.path(), Some("/path/to/file"));
    let url = Url::new("https://www.example.com").unwrap();
    assert_eq!(url.path(), Some(""));
    ```
    */
    pub fn path(&self) -> Option<&str>
    {
        self.range_to_str(&self.path)
    }
    /**
    Returns the query of the url if present, [`None`] otherwise.

    It returns the full query in a string slice, including the question mark. 
    To format into a [`Vec`], see [`Self::dispatch_query()`].
    
    ## Example
    ```
    use url_compose::Url;

    let url = Url::new("https://username:password@www.example.com:8080/path/to/file?field=value#fragment").unwrap();
    assert_eq!(url.query(), Some("?field=value"));
    let url = Url::new("/path/example").unwrap();
    assert_eq!(url.query(), None);
    ```
    */
    pub fn query(&self) -> Option<&str>
    {
        self.range_to_str(&self.query)
    }
    /**
    Returns the fragment of the url if present, [`None`] otherwise.
    
    ## Example
    ```
    use url_compose::Url;

    let url = Url::new("https://username:password@www.example.com:8080/path/to/file?field=value#fragment").unwrap();
    assert_eq!(url.frament(), Some("#fragment"));
    let url = Url::new("/path/example").unwrap();
    assert_eq!(url.frament(), None);
    ```
    */
    pub fn fragment(&self) -> Option<&str>
    {
        self.range_to_str(&self.fragment)
    }
    /**
    Split the query into a [`Vec`] of [`Query`].
    
    ## Example
    ```
    use url_compose::Url;
    use url_compose::Query::{Form, Field};

    let url = Url::new("/?field1=value1&field2=value2&present").unwrap();
    assert_eq!(url.dispatch_query(), [Form("field1", "value1"), Form("field2", "value2"), Field("present")]);
    let url = Url::new("/?field1=value1;field2=value2").unwrap();
    assert_eq!(url.dispatch_query(), [Form("field1", "value1"), Form("field2", "value2")]);
    let url = Url::new("/").unwrap();
    assert_eq!(url.dispatch_query(), []);
    ```
    */
    pub fn dispatch_query(&self) -> Vec<Query> {
        let queries = match self.query() {
            Some(v) => v,
            None => return Vec::new()
        };
        let queries = &queries[1..];
        let mut result = vec![];
        for query in queries.split(|c| c=='&' || c==';') {
            result.push(match query.find('=') {
                Some(pos) => Query::Form(&query[0..pos], &query[pos+1..]),
                None => Query::Field(query)
            });
        }
        result
    }
}

impl<S> From<Url<S>> for String 
    where S: AsRef<str>
{
    fn from(v: Url<S>) -> String {
        String::from(v.full.as_ref())
    }
}

impl<S> fmt::Debug for Url<S>
    where S: AsRef<str>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Url")
         .field("full", &self.full.as_ref())
         .field("scheme", &self.scheme())
         .field("user", &self.user())
         .field("host", &self.host())
         .field("port", &self.port())
         .field("path", &self.path())
         .field("query", &self.query())
         .field("fragment", &self.fragment())
         .finish()
    }
}
impl<S> fmt::Display for Url<S>
    where S: AsRef<str>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full.as_ref())
    }
}
impl<S> AsRef<str> for Url<S>
    where S: AsRef<str> 
{
    fn as_ref(&self) -> &str {
        self.full()
    }
}
impl<S> AsRef<[u8]> for Url<S>
    where S: AsRef<str> 
{
    fn as_ref(&self) -> &[u8] {
        self.full().as_ref()
    }
}
impl<S> Deref for Url<S>
    where S: AsRef<str> 
{
    type Target = str;
    fn deref(&self) -> &str {
        self.full()
    }
}