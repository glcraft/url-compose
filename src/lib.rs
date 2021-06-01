mod regex_generator;
mod tests;
use std::ops::Range;
use std::fmt;
use lazy_static::lazy_static;
use regex::Regex;

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
#[derive(Debug)]
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
                // println!("{}", regex_generator::url_regex());
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
    pub fn full(&self) -> &str {
        self.full.as_ref()
    }
    fn range_to_str(&self, range: &Option<Range<usize>>) -> Option<&str> {
        range.clone().map(|s| &self.full.as_ref()[s])
    }
    /**
    Returns the scheme of the url if present
    ## Example
    ```
    use url_compose::Url;
    let url = Url::new("https://a").unwrap();
    assert_eq!(url.scheme().unwrap(), "https");
    ```
    */
    pub fn scheme(&self) -> Option<&str>
    {
        self.range_to_str(&self.scheme)
    }
    /// Returns the user if present in the url (ex: ftp )
    pub fn user(&self) -> Option<&str>
    {
        self.range_to_str(&self.user)
    }
    pub fn password(&self) -> Option<&str>
    {
        self.range_to_str(&self.password)
    }
    pub fn host(&self) -> Option<&str>
    {
        self.range_to_str(&self.host)
    }
    pub fn port(&self) -> Option<&str>
    {
        self.range_to_str(&self.port)
    }
    pub fn path(&self) -> Option<&str>
    {
        self.range_to_str(&self.path)
    }
    pub fn query(&self) -> Option<&str>
    {
        self.range_to_str(&self.query)
    }
    pub fn fragment(&self) -> Option<&str>
    {
        self.range_to_str(&self.fragment)
    }

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