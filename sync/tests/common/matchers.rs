use googletest::{
    matcher::MatcherResult,
    prelude::{Matcher, MatcherBase},
};
use iceblink_sync::models;

#[derive(MatcherBase)]
pub struct CodeFixtureDefault;

pub fn code_fixture_default() -> CodeFixtureDefault {
    CodeFixtureDefault
}

impl Matcher<&Vec<models::codes::Code>> for CodeFixtureDefault {
    fn matches(&self, input: &Vec<models::codes::Code>) -> googletest::matcher::MatcherResult {
        let expected_user1_codes = vec![
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
        ];

        let expected_user2_codes = vec![models::codes::Code {
            id: "fUJveqJaNpPhTUkR".into(),
            owner_id: "3Ck0d8WrkRjK6gkc".into(),
            content: "djnaW1Pl2WjhWrU6".into(),
            display_name: "Dummy INC".into(),
            icon_url: Some("https://dummy.com/favicon.ico".into()),
            website_url: Some("dummy.com".into()),
        }];

        let first_code = match input.first() {
            Some(c) => c,
            None => return MatcherResult::NoMatch,
        };

        let user_code_vec = match first_code.owner_id.as_str() {
            "k0d8WrkRjK6gkc3C" => &expected_user1_codes,
            "3Ck0d8WrkRjK6gkc" => &expected_user2_codes,
            _ => return MatcherResult::NoMatch,
        };

        if input.len() != user_code_vec.len() {
            return MatcherResult::NoMatch;
        }

        for (actual, expected) in input.iter().zip(user_code_vec.iter()) {
            if actual != expected {
                return MatcherResult::NoMatch;
            }
        }

        for expected_code in user_code_vec {
            let found_code = match input.iter().find(|c| c.id == expected_code.id) {
                Some(c) => c,
                None => return MatcherResult::NoMatch,
            };

            if found_code != expected_code {
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
            MatcherResult::Match => "is an expected code entry".into(),
            MatcherResult::NoMatch => "isn't an expected code entry".into(),
        }
    }
}
