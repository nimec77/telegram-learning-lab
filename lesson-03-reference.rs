// Offline teaching policy only. No filesystem, upload, or Telegram connection.
// Observations stand in for a trusted adapter; booleans do NOT secure real files.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Kind { Png, Jpeg }
struct Request<'a> { root: &'a str, path: &'a str, expected: Kind, caption: &'a str }
struct Observation { size: u64, detected: Kind, safe_regular: bool, stable: bool }
#[derive(Debug, PartialEq)]
struct Ready { size: u64, kind: Kind }
fn preflight(r: &Request<'_>, o: &Observation) -> Result<Ready, &'static str> {
    if r.root != "outgoing" { return Err("root"); }
    // Deliberately narrow portable teaching grammar, not a filesystem sandbox.
    if r.path.is_empty() || r.path.contains(['\\', ':', '\0']) ||
        r.path.split('/').any(|part| part.is_empty() || part == "." || part == "..") {
        return Err("path");
    }
    if r.caption.encode_utf16().count() > 1024 { return Err("caption"); }
    if !o.safe_regular || !o.stable { return Err("file boundary"); }
    if o.size == 0 || o.size > 10 * 1024 * 1024 { return Err("size"); }
    if r.expected != o.detected { return Err("kind"); }
    Ok(Ready { size: o.size, kind: o.detected })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn req(path: &str) -> Request<'_> { Request {root:"outgoing",path,expected:Kind::Png,caption:""} }
    fn obs() -> Observation { Observation {size:4096,detected:Kind::Png,safe_regular:true,stable:true} }
    #[test] fn valid_returns_ready_not_sent() { assert_eq!(preflight(&req("images/chart.png"),&obs()),Ok(Ready {size:4096,kind:Kind::Png})); }
    #[test] fn root_is_explicit() { let mut r=req("a.png"); r.root="other"; assert_eq!(preflight(&r,&obs()),Err("root")); }
    #[test] fn rejects_ambiguous_paths() { for p in ["", "/a.png", "../a.png", "a/../b.png", "a//b.png", "./a.png", "a/", "a\\b.png", "C:/a.png", "a\0.png"] { assert_eq!(preflight(&req(p),&obs()),Err("path"),"{p:?}"); } }
    #[test] fn zero_rejected() { let mut o=obs(); o.size=0; assert_eq!(preflight(&req("a.png"),&o),Err("size")); }
    #[test] fn exact_limit_allowed() { let mut o=obs(); o.size=10*1024*1024; assert!(preflight(&req("a.png"),&o).is_ok()); }
    #[test] fn above_limit_rejected() { let mut o=obs(); o.size=10*1024*1024+1; assert_eq!(preflight(&req("a.png"),&o),Err("size")); }
    #[test] fn suffix_is_not_evidence() { let mut o=obs(); o.detected=Kind::Jpeg; assert_eq!(preflight(&req("a.png"),&o),Err("kind")); }
    #[test] fn unsafe_file_rejected() { let mut o=obs(); o.safe_regular=false; assert_eq!(preflight(&req("a.png"),&o),Err("file boundary")); }
    #[test] fn changing_source_rejected() { let mut o=obs(); o.stable=false; assert_eq!(preflight(&req("a.png"),&o),Err("file boundary")); }
    #[test] fn caption_uses_utf16() { let text="😀".repeat(512); let mut r=req("a.png"); r.caption=&text; assert!(preflight(&r,&obs()).is_ok()); let over="😀".repeat(513); r.caption=&over; assert_eq!(preflight(&r,&obs()),Err("caption")); }
}
