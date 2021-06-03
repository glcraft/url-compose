/*!
Provides a struct [`Url`] to decompose the url into a set of string slice. 
Some utilities functions are also provided.

# Motivation

You can notice this crate is very similar to the [url crate]. That's right : 
url-decompose almost do the same thing. 

As I started to make this crate, I didn't know the existance of "url" so I went to 
URI RFC to make my own implementation (using a regex). When I discover "url" crate, 
I decided to keep on my crate for some reasons :

### Url\<S\> can either own data or not

`S` has to implement `AsRef<str>`, so Url can stock whatever type you want. 
It can be String (so Url owns the string) or &str (so Url keeps a reference to 
another String). 

### This crate is light

As I'm writing, there is 2 files 400 lines in totale, and needs 3 dependencies. 
Of course, the crate should evolve to something better in parsing, but it'll
still remain light.

[url crate]: https://crates.io/crates/url
[RFC 3986]: https://datatracker.ietf.org/doc/html/rfc3986
*/

#[cfg(test)]
mod tests;
pub(crate) mod regex_generator;
pub(crate) mod url;

pub use url::*;