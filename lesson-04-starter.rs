// Lesson 4: classify a synthetic support message; never send it.
// Run: rustc --edition=2021 --test lesson-04-starter.rs -o lesson-04-tests
// Then: ./lesson-04-tests
// Only implement route. Draft means a template choice for human review,
// not a sent reply. This one-file exercise has no network or dependencies.

#[derive(Debug, PartialEq, Eq)]
pub enum Template {
    Install,
    Update,
    WatchLater,
    Favorites,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Reason {
    Empty,
    Unknown,
    Ambiguous,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    Draft(Template),
    HumanReview(Reason),
}

pub fn route(_message: &str) -> Outcome {
    // Trim the input, then split on ASCII whitespace.
    // No first token: Empty. Compare the complete first token with the
    // four known commands, ignoring ASCII case only. Unknown first token:
    // Unknown. A known first token with any extra token: Ambiguous.
    // Only a single known token may select a Draft.
    todo!("implement the draft-only router")
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! fixture {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(route($input), $expected);
            }
        };
    }

    fixture!(install_command, "/install", Outcome::Draft(Template::Install));
    fixture!(update_command, "/update", Outcome::Draft(Template::Update));
    fixture!(
        watch_later_command,
        "/watchlater",
        Outcome::Draft(Template::WatchLater)
    );
    fixture!(
        favorites_command,
        "/favorites",
        Outcome::Draft(Template::Favorites)
    );
    fixture!(
        mixed_ascii_case,
        "/WaTcHlAtEr",
        Outcome::Draft(Template::WatchLater)
    );
    fixture!(
        surrounding_whitespace_is_trimmed,
        "  \t/FAVORITES\r\n ",
        Outcome::Draft(Template::Favorites)
    );
    fixture!(
        unicode_surrounding_whitespace_is_not_trimmed,
        "\u{2003}/install\u{00a0}",
        Outcome::HumanReview(Reason::Unknown)
    );
    fixture!(empty_input, "", Outcome::HumanReview(Reason::Empty));
    fixture!(
        only_whitespace,
        "\t \n\x0c",
        Outcome::HumanReview(Reason::Empty)
    );
    fixture!(
        free_text_is_not_a_command,
        "How do I install?",
        Outcome::HumanReview(Reason::Unknown)
    );
    fixture!(
        unknown_command_with_arguments_stays_unknown,
        "/help please",
        Outcome::HumanReview(Reason::Unknown)
    );
    fixture!(
        later_known_token_does_not_route,
        "please /install",
        Outcome::HumanReview(Reason::Unknown)
    );
    fixture!(
        bot_suffix_is_not_stripped,
        "/install@bot",
        Outcome::HumanReview(Reason::Unknown)
    );
    fixture!(
        command_prefix_is_not_enough,
        "/installing",
        Outcome::HumanReview(Reason::Unknown)
    );
    fixture!(
        unicode_case_is_not_folded,
        "/\u{0130}nstall",
        Outcome::HumanReview(Reason::Unknown)
    );
    fixture!(
        known_command_with_extra_words,
        "/install please help",
        Outcome::HumanReview(Reason::Ambiguous)
    );
    fixture!(
        second_command_is_an_extra_token,
        "/UPDATE\t/favorites",
        Outcome::HumanReview(Reason::Ambiguous)
    );
    fixture!(
        internal_unicode_whitespace_is_not_a_separator,
        "/install\u{00a0}please",
        Outcome::HumanReview(Reason::Unknown)
    );
}
