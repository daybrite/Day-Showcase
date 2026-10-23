use day::prelude::*;
use day_part_auth::apple::SignInWithApple;
use day_part_auth::google::GoogleSignIn;
use day_part_auth::{AuthError, Client, Provider, TokenSet};
use serde_json::json;

use crate::widgets::page;

/// Sign-in (day-part-auth): the system's auth browser and an OpenID Connect client with PKCE and
/// verified ID tokens; the token lifecycle; passkeys; Sign in with Apple; Google.
///
/// The first section signs in to a provider: by default the demo provider the app starts on
/// loopback, which signs in two demo accounts with no account anywhere, or Google and Apple with a
/// client id you paste in, or any OpenID Connect issuer by discovery. The silent sign-in runs the
/// whole flow with an HTTP client standing in for the browser, which is what the walkthrough
/// drives on every target; the browser sign-in and the system sheets need a person.
pub(crate) fn signin_page() -> AnyPiece {
    page(
        crate::res::str::nav_signin(),
        day::id!(signin_title),
        form((
            oidc_section(),
            passkeys_section(),
            apple_section(),
            google_section(),
        ))
        .any(),
    )
    .any()
}

/// The three-way support label every section here shows.
fn support_text(support: Support) -> LocalizedText {
    match support {
        Support::Native => crate::res::str::speech_native(),
        Support::Emulated => crate::res::str::speech_emulated(),
        Support::Unsupported => crate::res::str::speech_unsupported(),
    }
}

/// Which provider the first section talks to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Which {
    Demo,
    Google,
    Apple,
    Custom,
}

/// The demo provider, started once for the life of the app, on native targets.
#[cfg(not(target_arch = "wasm32"))]
mod demo {
    use std::sync::OnceLock;

    use day_part_auth::demo_provider::{DemoConfig, DemoProvider, sign_in_headless};
    use day_part_auth::{AuthError, Client, TokenSet};

    static PROVIDER: OnceLock<Option<DemoProvider>> = OnceLock::new();

    pub(super) fn provider() -> Option<&'static DemoProvider> {
        PROVIDER
            .get_or_init(|| match DemoProvider::start(DemoConfig::default()) {
                Ok(p) => Some(p),
                Err(e) => {
                    day::prelude::info!("the demo provider did not start: {e}");
                    None
                }
            })
            .as_ref()
    }

    pub(super) fn client() -> Option<Client> {
        let demo = provider()?;
        Some(Client::new(demo.provider(), demo.client_id()).scopes(["openid", "profile", "email"]))
    }

    /// The silent sign-in: `prompt=none` as the first demo account, no browser.
    pub(super) async fn silent(client: Client) -> Result<TokenSet, AuthError> {
        let user = provider().and_then(|p| p.users().first().map(|u| u.sub.clone()));
        sign_in_headless(&client, user.as_deref()).await
    }

    pub(super) fn issuer() -> Option<String> {
        provider().map(|p| p.issuer().to_string())
    }
}

/// A browser page cannot run a loopback server: on the web the demo provider is absent and the
/// section signs in to the provider you name.
#[cfg(target_arch = "wasm32")]
mod demo {
    use day_part_auth::{AuthError, Client, TokenSet};

    pub(super) fn client() -> Option<Client> {
        None
    }

    pub(super) async fn silent(_client: Client) -> Result<TokenSet, AuthError> {
        Err(AuthError::Unsupported)
    }

    pub(super) fn issuer() -> Option<String> {
        None
    }
}

/// The signed-in line for a token set: name and email from the ID token, the subject always.
fn signed_in_text(tokens: &TokenSet) -> String {
    let claims = tokens.id_token.as_ref().map(|t| &t.claims);
    crate::res::str::signin_signed_in(crate::res::str::SigninSignedIn {
        name: claims
            .and_then(|c| c.name.clone())
            .unwrap_or_else(|| "—".into()),
        email: claims
            .and_then(|c| c.email.clone())
            .unwrap_or_else(|| "—".into()),
        sub: tokens.subject().unwrap_or("—").to_string(),
    })
    .format()
}

/// What the token set says about itself, for the detail line.
fn tokens_text(tokens: &TokenSet) -> String {
    let expires = tokens
        .expires_in()
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|| "?".into());
    let refresh = if tokens.refresh_token.is_some() {
        crate::res::str::signin_refresh_yes()
    } else {
        crate::res::str::signin_refresh_no()
    };
    crate::res::str::signin_tokens(crate::res::str::SigninTokens {
        secs: expires,
        scope: tokens.scope.clone().unwrap_or_else(|| "—".into()),
        refresh: refresh.format(),
    })
    .format()
}

/// OpenID Connect: a provider, a client id, the browser sign-in and the silent one, and the token
/// lifecycle. The demo provider needs no id; Google and Apple take the one you registered.
fn oidc_section() -> impl Piece {
    let which = Signal::new(Which::Demo);
    let client_id = Signal::new(String::new());
    let issuer = Signal::new(String::new());
    let status = Signal::new(crate::res::str::signin_idle().format());
    let detail = Signal::new(String::new());
    let tokens: Signal<Option<TokenSet>> = Signal::new(None);
    let caps = day_part_auth::session::capabilities();
    let demo_issuer = demo::issuer();

    // The client for the chosen provider, or the reason there is none.
    let build = move || -> Result<Client, AuthError> {
        let id = client_id.with(|s| s.trim().to_string());
        match which.get() {
            Which::Demo => demo::client().ok_or(AuthError::Unsupported),
            Which::Google if id.is_empty() => Err(AuthError::InvalidRequest(
                crate::res::str::signin_need_client_id().format(),
            )),
            Which::Google => {
                Ok(Client::new(Provider::google(), id).scopes(["openid", "email", "profile"]))
            }
            Which::Apple if id.is_empty() => Err(AuthError::InvalidRequest(
                crate::res::str::signin_need_client_id().format(),
            )),
            // Apple's web flow wants an https:// redirect the Services ID registered, and answers
            // it with response_mode=form_post when name or email is asked for; with the plain
            // openid scope the code comes back in the query, which the client reads.
            Which::Apple => Ok(Client::new(Provider::apple(), id).scopes(["openid"])),
            Which::Custom => Err(AuthError::InvalidRequest(
                crate::res::str::signin_need_discovery().format(),
            )),
        }
    };
    let settle =
        move |outcome: Result<TokenSet, AuthError>, on_ok: fn(&TokenSet) -> String| match outcome {
            Ok(set) => {
                status.set(on_ok(&set));
                detail.set(tokens_text(&set));
                tokens.set(Some(set));
            }
            Err(e) => status.set(e.to_string()),
        };
    let access_token = move || tokens.with(|t| t.as_ref().map(|t| t.access_token.clone()));

    // The custom issuer: discovered when Sign in is pressed, so a typo is reported as the
    // provider's own words.
    let sign_in = move || {
        status.set(crate::res::str::signin_working().format());
        if which.get() == Which::Custom {
            let issuer = issuer.with(|s| s.trim().to_string());
            let id = client_id.with(|s| s.trim().to_string());
            if issuer.is_empty() || id.is_empty() {
                status.set(crate::res::str::signin_need_discovery().format());
                return;
            }
            day::task(async move {
                let discovered = match Provider::discover(&issuer).await {
                    Ok(p) => p,
                    Err(e) => {
                        status.set(e.to_string());
                        return;
                    }
                };
                let client = Client::new(discovered, id).scopes(["openid", "email", "profile"]);
                settle(client.sign_in().await, signed_in_text);
            });
            return;
        }
        match build() {
            Ok(client) => {
                day::task(async move {
                    settle(client.sign_in().await, signed_in_text);
                });
            }
            Err(e) => status.set(e.to_string()),
        }
    };
    let sign_in_silent = move || {
        status.set(crate::res::str::signin_working().format());
        match demo::client() {
            Some(client) if which.get() == Which::Demo => {
                day::task(async move {
                    settle(demo::silent(client).await, signed_in_text);
                });
            }
            Some(_) => status.set(crate::res::str::signin_silent_demo_only().format()),
            None => status.set(AuthError::Unsupported.to_string()),
        }
    };
    let refresh = move || {
        let Some(refresh_token) = tokens.with(|t| t.as_ref().and_then(|t| t.refresh_token.clone()))
        else {
            status.set(crate::res::str::signin_nothing_to_refresh().format());
            return;
        };
        match build() {
            Ok(client) => {
                day::task(async move {
                    settle(client.refresh(&refresh_token).await, |_| {
                        crate::res::str::signin_refreshed().format()
                    });
                });
            }
            Err(e) => status.set(e.to_string()),
        }
    };
    let userinfo = move || {
        let Some(access) = access_token() else {
            status.set(crate::res::str::signin_idle().format());
            return;
        };
        match build() {
            Ok(client) => {
                day::task(async move {
                    match client.userinfo(&access).await {
                        Ok(info) => {
                            let sub = info["sub"].as_str().unwrap_or("—").to_string();
                            status.set(crate::res::str::signin_userinfo_ok(sub).format());
                            detail.set(info.to_string());
                        }
                        Err(e) => status.set(e.to_string()),
                    }
                });
            }
            Err(e) => status.set(e.to_string()),
        }
    };
    let revoke = move || {
        let Some(access) = access_token() else {
            status.set(crate::res::str::signin_idle().format());
            return;
        };
        match build() {
            Ok(client) => {
                day::task(async move {
                    status.set(match client.revoke(&access, "access_token").await {
                        Ok(()) => crate::res::str::signin_revoked().format(),
                        Err(e) => e.to_string(),
                    });
                });
            }
            Err(e) => status.set(e.to_string()),
        }
    };
    let sign_out = move || {
        tokens.set(None);
        detail.set(String::new());
        status.set(crate::res::str::signin_signed_out().format());
    };

    section((
        crate::widgets::support_note(caps.support),
        label(crate::res::str::signin_caption()).font(Font::Footnote),
        labeled(
            crate::res::str::signin_browser_label(),
            label(support_text(caps.support)).id(day::id!(signin_browser_support)),
        ),
        labeled(
            crate::res::str::signin_provider_label(),
            picker(
                which,
                [
                    (Which::Demo, crate::res::str::signin_provider_demo()),
                    (Which::Google, crate::res::str::signin_provider_google()),
                    (Which::Apple, crate::res::str::signin_provider_apple()),
                    (Which::Custom, crate::res::str::signin_provider_custom()),
                ],
            )
            .menu()
            .id(day::id!(signin_provider)),
        ),
        // The demo provider's address, so the person can open it in a browser and see the
        // consent page the sign-in button opens; or the reason it is absent.
        label(move || {
            format!(
                "{}: {}",
                crate::res::str::signin_issuer_label().format(),
                match (which.get(), &demo_issuer) {
                    (Which::Demo, Some(issuer)) => issuer.clone(),
                    (Which::Demo, None) => crate::res::str::signin_demo_absent().format(),
                    (Which::Google, _) => "https://accounts.google.com".to_string(),
                    (Which::Apple, _) => "https://appleid.apple.com".to_string(),
                    (Which::Custom, _) => issuer.get(),
                }
            )
        })
        .font(Font::Footnote)
        .id(day::id!(signin_issuer)),
        labeled(
            crate::res::str::signin_custom_issuer_label(),
            text_field(issuer)
                .placeholder(crate::res::str::signin_issuer_placeholder())
                .id(day::id!(signin_issuer_field)),
        ),
        labeled(
            crate::res::str::signin_client_id_label(),
            text_field(client_id)
                .placeholder(crate::res::str::signin_client_id_placeholder())
                .id(day::id!(signin_client_id)),
        ),
        row((
            button(crate::res::str::signin_sign_in())
                .action(sign_in)
                .tint(crate::widgets::primary())
                .id(day::id!(signin_sign_in)),
            button(crate::res::str::signin_sign_in_silent())
                .bordered()
                .action(sign_in_silent)
                .id(day::id!(signin_sign_in_silent)),
        ))
        .spacing(Space::S),
        // Four buttons outgrow a phone-portrait row, so they wrap in two pairs on a Compact window
        // (docs/size-classes.md) and sit in one row elsewhere.
        adaptive((
            row((
                button(crate::res::str::signin_refresh())
                    .bordered()
                    .action(refresh)
                    .id(day::id!(signin_refresh)),
                button(crate::res::str::signin_userinfo())
                    .bordered()
                    .action(userinfo)
                    .id(day::id!(signin_userinfo)),
            ))
            .spacing(Space::S),
            row((
                button(crate::res::str::signin_revoke())
                    .bordered()
                    .action(revoke)
                    .id(day::id!(signin_revoke)),
                button(crate::res::str::signin_sign_out())
                    .bordered()
                    .action(sign_out)
                    .id(day::id!(signin_sign_out)),
            ))
            .spacing(Space::S),
        )),
        // The outcome and the token facts take whole lines: a claim set or an error message is
        // longer than a phone's value column.
        label(move || status.get())
            .font(Font::Footnote)
            .id(day::id!(signin_status)),
        label(move || detail.get())
            .font(Font::Footnote)
            .id(day::id!(signin_detail)),
    ))
    .title(crate::res::str::signin_oidc_title())
}

/// Passkeys: the authenticator probe, a relying party, and the two ceremonies.
fn passkeys_section() -> impl Piece {
    let rp = Signal::new(String::new());
    let result = Signal::new(crate::res::str::passkeys_idle().format());
    let available = Signal::new(String::new());
    let support = crate::support::passkeys();
    if support != Support::Unsupported {
        day::task(async move {
            let yes = day_part_auth::passkeys::platform_authenticator_available().await;
            available.set(
                if yes {
                    crate::res::str::passkeys_available_yes()
                } else {
                    crate::res::str::passkeys_available_no()
                }
                .format(),
            );
        });
    } else {
        available.set(crate::res::str::passkeys_available_no().format());
    }
    let rp_id = move || {
        let typed = rp.with(|r| r.trim().to_string());
        if typed.is_empty() {
            DEFAULT_RP.to_string()
        } else {
            typed
        }
    };
    let create = move || {
        let rp_id = rp_id();
        let (Ok(challenge), Ok(user_id)) = (
            day_part_auth::crypto::random_token(32),
            day_part_auth::crypto::random_token(16),
        ) else {
            result.set(AuthError::Platform("no randomness".into()).to_string());
            return;
        };
        let options = json!({
            "rp": { "id": rp_id, "name": "Day Showcase" },
            "user": { "id": user_id, "name": "ada@example.com", "displayName": "Ada Lovelace" },
            "challenge": challenge,
            "pubKeyCredParams": [
                { "type": "public-key", "alg": -7 },
                { "type": "public-key", "alg": -257 }
            ],
            "authenticatorSelection": {
                "authenticatorAttachment": "platform",
                "residentKey": "required",
                "userVerification": "preferred"
            },
            "attestation": "none",
            "timeout": 60000
        });
        result.set(crate::res::str::passkeys_prompting().format());
        day::task(async move {
            result.set(match day_part_auth::passkeys::create(&options).await {
                Ok(credential) => crate::res::str::passkeys_created(
                    credential["id"].as_str().unwrap_or("—").to_string(),
                )
                .format(),
                Err(e) => e.to_string(),
            });
        });
    };
    let get = move || {
        let rp_id = rp_id();
        let Ok(challenge) = day_part_auth::crypto::random_token(32) else {
            result.set(AuthError::Platform("no randomness".into()).to_string());
            return;
        };
        let options = json!({
            "rpId": rp_id,
            "challenge": challenge,
            "userVerification": "preferred",
            "timeout": 60000
        });
        result.set(crate::res::str::passkeys_prompting().format());
        day::task(async move {
            result.set(match day_part_auth::passkeys::get(&options).await {
                Ok(credential) => crate::res::str::passkeys_asserted(
                    credential["id"].as_str().unwrap_or("—").to_string(),
                )
                .format(),
                Err(e) => e.to_string(),
            });
        });
    };

    section((
        crate::widgets::support_note(support),
        label(crate::res::str::passkeys_caption()).font(Font::Footnote),
        labeled(
            crate::res::str::passkeys_support_label(),
            label(support_text(support)).id(day::id!(passkeys_support)),
        ),
        labeled(
            crate::res::str::passkeys_available_label(),
            label(move || available.get()).id(day::id!(passkeys_available)),
        ),
        labeled(
            crate::res::str::passkeys_rp_label(),
            text_field(rp)
                .placeholder(DEFAULT_RP)
                .id(day::id!(passkeys_rp)),
        ),
        // Two long labels outgrow a phone-portrait row: side by side where they fit, stacked on
        // a Compact window (docs/size-classes.md).
        adaptive((
            button(crate::res::str::passkeys_create())
                .action(create)
                .tint(crate::widgets::primary())
                .id(day::id!(passkeys_create)),
            button(crate::res::str::passkeys_get())
                .bordered()
                .action(get)
                .id(day::id!(passkeys_get)),
        )),
        label(move || result.get())
            .font(Font::Footnote)
            .id(day::id!(passkeys_result)),
    ))
    .title(crate::res::str::passkeys_title())
}

/// The relying party the passkey ceremonies name when the field is empty. On the web it must be
/// the page's own domain; on Apple and Android it must be a domain associated with the app, which
/// the sheet says in its own words when it is not.
#[cfg(target_arch = "wasm32")]
const DEFAULT_RP: &str = "localhost";
#[cfg(not(target_arch = "wasm32"))]
const DEFAULT_RP: &str = "daybrite.dev";

/// Sign in with Apple: the system sheet on iOS and macOS, and what it says elsewhere.
fn apple_section() -> impl Piece {
    let result = Signal::new(crate::res::str::signin_idle().format());
    let support = crate::support::apple_sign_in();
    section((
        crate::widgets::support_note(support),
        label(crate::res::str::apple_caption()).font(Font::Footnote),
        labeled(
            crate::res::str::apple_support_label(),
            label(support_text(support)).id(day::id!(apple_support)),
        ),
        button(crate::res::str::apple_sign_in())
            .tint(crate::widgets::primary())
            .action(move || {
                result.set(crate::res::str::signin_working().format());
                day::task(async move {
                    result.set(
                        match day_part_auth::apple::sign_in(SignInWithApple::new()).await {
                            Ok(credential) => {
                                crate::res::str::apple_signed_in(credential.user).format()
                            }
                            Err(e) => e.to_string(),
                        },
                    );
                });
            })
            .id(day::id!(apple_sign_in)),
        label(move || result.get())
            .font(Font::Footnote)
            .id(day::id!(apple_result)),
    ))
    .title(crate::res::str::apple_title())
}

/// Google through Credential Manager on Android, with the server client id pasted in.
fn google_section() -> impl Piece {
    let server_client_id = Signal::new(String::new());
    let result = Signal::new(crate::res::str::signin_idle().format());
    let support = crate::support::google_sign_in();
    section((
        crate::widgets::support_note(support),
        label(crate::res::str::google_caption()).font(Font::Footnote),
        labeled(
            crate::res::str::google_support_label(),
            label(support_text(support)).id(day::id!(google_support)),
        ),
        text_field(server_client_id)
            .placeholder(crate::res::str::google_client_placeholder())
            .id(day::id!(google_client)),
        button(crate::res::str::google_sign_in())
            .tint(crate::widgets::primary())
            .action(move || {
                let id = server_client_id.with(|s| s.trim().to_string());
                result.set(crate::res::str::signin_working().format());
                day::task(async move {
                    result.set(
                        match day_part_auth::google::sign_in(GoogleSignIn::new(id)).await {
                            Ok(credential) => {
                                crate::res::str::google_signed_in(credential.id).format()
                            }
                            Err(e) => e.to_string(),
                        },
                    );
                });
            })
            .id(day::id!(google_sign_in)),
        label(move || result.get())
            .font(Font::Footnote)
            .id(day::id!(google_result)),
    ))
    .title(crate::res::str::google_title())
}
