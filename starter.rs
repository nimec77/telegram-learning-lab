// A teaching model only: no Telegram, credentials, files, or network.
#[derive(Debug, PartialEq)]
enum Outcome { Preview, Sent(u64), NotSent(&'static str), Unknown }
#[derive(Clone, Copy)]
enum Mode { Preview, Dispatch }
struct Request<'a> { destination: &'a str, text: &'a str }
trait Sender { fn send(&mut self, request: &Request<'_>) -> Outcome; }
fn execute(request: &Request<'_>, enabled: bool, mode: Mode, sender: &mut impl Sender) -> Outcome {
    let _ = (request, enabled, mode, sender);
    todo!("Implement the local policy, preview and one-attempt dispatch")
}
struct FakeSender { calls: usize, next: Outcome, last_text: String }
impl Sender for FakeSender {
    fn send(&mut self, request: &Request<'_>) -> Outcome {
        self.calls += 1;
        self.last_text = request.text.to_owned();
        std::mem::replace(&mut self.next, Outcome::Unknown)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fake(next: Outcome) -> FakeSender { FakeSender { calls: 0, next, last_text: String::new() } }
    fn request(text: &str) -> Request<'_> { Request { destination: "test-room", text } }
    #[test] fn disabled_never_dispatches() {
        let mut f = fake(Outcome::Sent(42));
        assert_eq!(execute(&request("hello"), false, Mode::Dispatch, &mut f), Outcome::NotSent("disabled"));
        assert_eq!(f.calls, 0);
    }
    #[test] fn blank_never_dispatches() {
        let mut f = fake(Outcome::Sent(42));
        assert_eq!(execute(&request(" \n"), true, Mode::Dispatch, &mut f), Outcome::NotSent("text"));
        assert_eq!(f.calls, 0);
    }
    #[test] fn other_destination_never_dispatches() {
        let mut f = fake(Outcome::Sent(42));
        let r = Request { destination: "other-room", text: "hello" };
        assert_eq!(execute(&r, true, Mode::Dispatch, &mut f), Outcome::NotSent("destination"));
        assert_eq!(f.calls, 0);
    }
    #[test] fn preview_never_dispatches() {
        let mut f = fake(Outcome::Sent(42));
        assert_eq!(execute(&request("hello"), true, Mode::Preview, &mut f), Outcome::Preview);
        assert_eq!(f.calls, 0);
    }
    #[test] fn success_preserves_text_and_receipt() {
        let mut f = fake(Outcome::Sent(42));
        assert_eq!(execute(&request(" hello \n"), true, Mode::Dispatch, &mut f), Outcome::Sent(42));
        assert_eq!(f.calls, 1); assert_eq!(f.last_text, " hello \n");
    }
    #[test] fn ambiguous_result_is_not_retried() {
        let mut f = fake(Outcome::Unknown);
        assert_eq!(execute(&request("hello"), true, Mode::Dispatch, &mut f), Outcome::Unknown);
        assert_eq!(f.calls, 1);
    }
    #[test] fn utf16_limit_is_inclusive() {
        let mut f = fake(Outcome::Sent(42));
        assert_eq!(execute(&request(&"😀".repeat(2048)), true, Mode::Preview, &mut f), Outcome::Preview);
        assert_eq!(execute(&request(&"😀".repeat(2049)), true, Mode::Dispatch, &mut f), Outcome::NotSent("text"));
        assert_eq!(f.calls, 0);
    }
    #[test] fn definite_refusal_stays_not_sent() {
        let mut f = fake(Outcome::NotSent("refused"));
        assert_eq!(execute(&request("hello"), true, Mode::Dispatch, &mut f), Outcome::NotSent("refused"));
        assert_eq!(f.calls, 1);
    }
}
