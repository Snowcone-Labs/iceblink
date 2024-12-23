use googletest::{
    matcher::MatcherResult,
    prelude::{Matcher, MatcherBase},
};
use iceblink_sync::models;

fn get_codes_for_user(userid: &str) -> Vec<models::codes::Code> {
    match userid {
        "k0d8WrkRjK6gkc3C" => vec![
            models::codes::Code {
                id: "Ckpt4eFi1pw9fxI3".into(),
                owner_id: "k0d8WrkRjK6gkc3C".into(),
                content: "GK6ZFMqk18fuWnCw".into(),
                display_name: "Google".into(),
                icon_url: None,
                website_url: Some("google.com".into()),
            },
            models::codes::Code {
                id: "DxLCqi4ZlHPD8YxA".into(),
                owner_id: "k0d8WrkRjK6gkc3C".into(),
                content: "XGDi8FlvZ5OGBoxG".into(),
                display_name: "google.com".into(),
                icon_url: None,
                website_url: Some("google.com".into()),
            },
        ],
        "3Ck0d8WrkRjK6gkc" => vec![models::codes::Code {
            id: "fUJveqJaNpPhTUkR".into(),
            owner_id: "3Ck0d8WrkRjK6gkc".into(),
            content: "djnaW1Pl2WjhWrU6".into(),
            display_name: "Dummy INC".into(),
            icon_url: Some("https://dummy.com/favicon.ico".into()),
            website_url: Some("dummy.com".into()),
        }],
        _ => panic!("Unexpected UserId in code_is_expected"),
    }
}

pub fn code_is_expected(userid: &str, code: &models::codes::Code) -> bool {
    let expected = get_codes_for_user(userid);
    expected.iter().any(|c| c == code)
}

#[derive(MatcherBase)]
pub struct CodeFixtureMatcher;

pub fn code_fixture() -> CodeFixtureMatcher {
    CodeFixtureMatcher
}

impl Matcher<&Vec<models::codes::Code>> for CodeFixtureMatcher {
    fn matches(&self, input: &Vec<models::codes::Code>) -> googletest::matcher::MatcherResult {
        let first_code = match input.first() {
            Some(c) => c,
            None => return MatcherResult::NoMatch,
        };

        let expected_user_codes = get_codes_for_user(&first_code.owner_id);

        if input.len() != expected_user_codes.len() {
            return MatcherResult::NoMatch;
        }

        for expected_code in expected_user_codes {
            let found_code = match input.iter().find(|c| c.id == expected_code.id) {
                Some(c) => c,
                None => return MatcherResult::NoMatch,
            };

            if *found_code != expected_code {
                return MatcherResult::NoMatch;
            }
        }

        MatcherResult::Match
    }

    fn describe(
        &self,
        matcher_result: googletest::matcher::MatcherResult,
    ) -> googletest::description::Description {
        match matcher_result {
            MatcherResult::Match => {
                "is an expected code entry based on `users` + `code` SQLx fixture".into()
            }
            MatcherResult::NoMatch => {
                "isn't an expected code entry based on `users` + `code` SQLx fixture".into()
            }
        }
    }
}
