use axum::response::{Html, IntoResponse};
use chrono::NaiveDate;
use domain::{FutureMeetUp, FutureMeetUpState, User};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

pub mod admin;
pub mod index;
pub mod past_meet_up;
pub mod user;

pub struct HtmlError;

impl IntoResponse for HtmlError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        let html = r#"
            <!doctype html>
            <html lang="pt">
                <head>
                    <title>RustCWB - Panic</title>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <link rel="stylesheet" type="text/css" href="/assets/css/styles.css" />
                    <link rel="preconnect" href="https://fonts.googleapis.com" />
                    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                    <link
                        rel="icon"
                        sizes="16x16"
                        type="image/png"
                        href="/assets/favicon-16x16.png"
                    />
                    <link
                        rel="icon"
                        sizes="32x32"
                        type="image/png"
                        href="/assets/favicon-32x32.png"
                    />
                    <link rel="icon" type="image/svg+xml" href="/assets/favicon.svg" />
                    <link
                        href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&display=swap"
                        rel="stylesheet"
                    />
                    <script src="https://unpkg.com/htmx.org@1.9.12"></script>
                </head>
                <body>
                    <div class="font-jetBrains container mx-auto p-4">
                        <h1 class="text-4xl font-bold">Dont PANIC!!</h1>
                        <p class="text-xl">An error occurred while processing your request.</p>
                    </div>
                </body>
            </html>
        "#;
        Html(html).into_response()
    }
}

impl<E> From<E> for HtmlError
where
    E: std::fmt::Display,
{
    fn from(err: E) -> Self {
        tracing::error!("Unexpected error: {}", err);
        HtmlError
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserPresenter {
    nickname: String,
}

impl From<User> for UserPresenter {
    fn from(user: User) -> Self {
        Self {
            nickname: user.nickname.chars().take(10).collect::<String>(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FutureMeetUpPresenter {
    id: Ulid,
    title: String,
    state: String,
    description: String,
    speaker: String,
    date: NaiveDate,
    location: String,
}

impl From<FutureMeetUp> for FutureMeetUpPresenter {
    fn from(meetup: FutureMeetUp) -> Self {
        let (state, title, description, speaker) = match meetup.state {
            FutureMeetUpState::Scheduled {
                title,
                description,
                speaker,
            } => ("Scheduled".into(), title, description, speaker),
            FutureMeetUpState::CallForPapers => (
                "CallForPapers".into(),
                String::new(),
                String::new(),
                String::new(),
            ),
            FutureMeetUpState::Voting => {
                ("Voting".into(), String::new(), String::new(), String::new())
            }
        };
        Self {
            id: meetup.id,
            title,
            state,
            description,
            speaker,
            date: meetup.date,
            location: meetup.location,
        }
    }
}
