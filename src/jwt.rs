mod jwt;
mod jwte;
mod utils;

use std::str::FromStr;

use web_sys::HtmlInputElement;
use yew::{classes, function_component, html, use_state, Callback, Html, TargetCast};

use crate::jwt::{
    jwt::{editor::JwtEditor, viewer::JwtViewer},
    jwte::Jwte,
    utils::Utils,
};

const TEST_JWT: &str = "eyJhbGciOiJIUzI1NiJ9.eyJSb2xlIjoiQWRtaW4iLCJJc3N1ZXIiOiJJc3N1ZXIiLCJVc2VybmFtZSI6IkphdmFJblVzZSIsImV4cCI6MTY3MDAwNDI1NCwiaWF0IjoxNjcwMDA0MjU0fQ.ZGsN42vr-bM4uxXowtlNl7xRerkdKu6i29VS8DFQ4Tw";

#[function_component(Jwt)]
pub fn jwt() -> Html {
    let jwt = use_state(|| TEST_JWT.to_owned());
    let jwte = use_state(|| Jwte::default());

    let jwt_setter = jwt.setter();
    let oninput = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        jwt_setter.set(input.value());
    });

    let jwte_setter = jwte.setter();
    let onclick = Callback::from(move |_| match Jwte::from_str(TEST_JWT) {
        Ok(jwte) => jwte_setter.set(jwte),
        Err(error) => log::error!("{}", error),
    });

    html! {
        <article class={classes!("vertical")}>
            <textarea
                rows="5"
                placeholder={"base64 encoded JWT(JWE)"}
                class={classes!("base-input")}
                value={(*jwt).clone()}
                {oninput}
            />
            <button {onclick}>{"Process"}</button>
            {match &(*jwte) {
                Jwte::Jwt(jwt) => html! {
                    <div class={classes!("jwt-page")}>
                        <JwtViewer jwt={jwt.clone()} />
                        <JwtEditor jwt={jwt.clone()} />
                    </div>
                },
                Jwte::Jwe(_jwe) => html! {},
            }}
            <div class={classes!("container")}>
                <Utils />
            </div>
        </article>
    }
}
