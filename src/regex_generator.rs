
/// 
/// Regular expression based on the (RFC 3986)[https://datatracker.ietf.org/doc/html/rfc3986#section-4.2]
/// 
use const_format::formatcp;

const PCT_ENC:&str = "%[[:xdigit:]]{2}";
const UNRESERVED:&str = r"[a-zA-Z0-9\-._~]";
const SUB_DELIMS: &str = "[!$&'()*+,;=]";
const PCHAR:&str = formatcp!("(?:{UNRESERVED}|{PCT_ENC}|{SUB_DELIMS}|[:@])");

/// Complete Url regular expression
pub const fn url_regex() -> &'static str {
    const PORT: &str = "[0-9]*";
    formatcp!(r"^(?:({})://({})(?::({}))?@)?({})(?::({PORT}))?({})(\?{})?(#{})?$", scheme(), user(), password(), host(), path(), query(), fragment())
}
/// https://datatracker.ietf.org/doc/html/rfc3986#section-4.2
#[allow(dead_code)]
const fn relative_ref() -> &'static str {
    const RELATIVE_PART: &str = formatcp!("//{}{}", authority(), path());
    formatcp!(r"{RELATIVE_PART}(?:\?{})(?:#{})", query(), fragment())
}
/// https://datatracker.ietf.org/doc/html/rfc3986#section-3.1
const fn scheme() -> &'static str {
    r"[a-zA-Z][a-zA-Z0-9+\-.]*"
}
/// https://datatracker.ietf.org/doc/html/rfc3986#section-3.2
#[allow(dead_code)]
const fn authority() -> &'static str {
    const PORT: &str = "[0-9]*";
    formatcp!(r"(?:{}@)?{}(?::{PORT})", user(), host())
}
/// https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.1
const fn user() -> &'static str {
    formatcp!("(?:{UNRESERVED}|{PCT_ENC}|{SUB_DELIMS})*")
}
const fn password() -> &'static str {
    formatcp!("(?:{UNRESERVED}|{PCT_ENC}|{SUB_DELIMS})*")
}
/// https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.2
const fn host() -> &'static str {
    const IPVFUTURE: &str = formatcp!(r"v[[:xdigit:]]+\.(?:{UNRESERVED}|{SUB_DELIMS}|:)+");
    const IP_LITTERAL: &str = formatcp!(r"\[{}|{IPVFUTURE}\]", ipv6());
    formatcp!(r"{IP_LITTERAL}|{}|{}", ipv4(), register_name())
}
/// https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.2
const fn ipv4() -> &'static str {
    const DEC_OCTET:&str = r"(?:\d|[1-9]\d|1\d{2}|2[0-4]\d|25[0-5])";
    formatcp!(r"(?:{DEC_OCTET}\.{DEC_OCTET}\.{DEC_OCTET}\.{DEC_OCTET})")
}
/// https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.2
const fn ipv6() -> &'static str {
    const H16:&str = "[[:xdigit:]]{1,4}";
    const LS32:&str = formatcp!("{H16}:{H16}|{}", ipv4());
    const CASE:[&str;9] = [
        formatcp!("(?:{H16}:){{6}}{LS32}"),
        formatcp!("::(?:{H16}:){{5}}{LS32}"),
        formatcp!("(?:{H16})?::(?:{H16}:){{4}}{LS32}"),
        formatcp!("(?:(?:{H16}:){{0,1}}{H16})?::(?:{H16}:){{3}}{LS32}"),
        formatcp!("(?:(?:{H16}:){{0,2}}{H16})?::(?:{H16}:){{2}}{LS32}"),
        formatcp!("(?:(?:{H16}:){{0,3}}{H16})?::{H16}:{LS32}"),
        formatcp!("(?:(?:{H16}:){{0,4}}{H16})?::{LS32}"),
        formatcp!("(?:(?:{H16}:){{0,5}}{H16})?::{H16}"),
        formatcp!("(?:(?:{H16}:){{0,6}}{H16})?::")
    ];
    formatcp!(r"(?:{}|{}|{}|{}|{}|{}|{}|{}|{})", CASE[0], CASE[1], CASE[2], CASE[3], CASE[4], CASE[5], CASE[6], CASE[7], CASE[8])
}
/// https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.2
const fn register_name() -> &'static str {
    formatcp!("(?:{UNRESERVED}|{PCT_ENC}|{SUB_DELIMS})*")
}
/// https://datatracker.ietf.org/doc/html/rfc3986#section-3.3
const fn path() -> &'static str {
    const SEGMENT:&str = formatcp!("{PCHAR}*");
    const SEGMENT_NZ:&str = formatcp!("{PCHAR}+");
    const PATH_ABEMPTY:&str = formatcp!("(?:/{SEGMENT})*");
    const PATH_ABSOLUTE:&str = formatcp!("(?:/(?:{SEGMENT_NZ}{PATH_ABEMPTY})?)");
    formatcp!("{PATH_ABSOLUTE}|{PATH_ABEMPTY}")
}
/// https://datatracker.ietf.org/doc/html/rfc3986#section-3.4
const fn query() -> &'static str {
    formatcp!(r"(?:{PCHAR}|[/?])*")
}
/// https://datatracker.ietf.org/doc/html/rfc3986#section-3.5
const fn fragment() -> &'static str {
    formatcp!(r"(?:{PCHAR}|[/?])*")
}