use handlebars::Handlebars;
// use reqwest::Client;
use rust_config::Configurator;
// use serde::Serialize;
use serde_json::json;
// use std::collections::HashMap;
use std::sync::Arc;
use warp::Filter;

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars>) -> impl warp::Reply
where
    T: Serialize,
{
    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());
    warp::reply::html(render)
}

#[tokio::main]
async fn main() {
    let config = rust_config::new("WEBSITE").unwrap();

    let mut hb = Handlebars::new();
    hb.register_template_file("index", "./static/index.html")
        .unwrap();

    let hb = Arc::new(hb);

    let handlebars = move |with_template| render(with_template, hb.clone());

    let index_route = warp::path::end()
        .map(|| WithTemplate {
            name: "index",
            value: json!({}),
        })
        .map(handlebars.clone());
    // let expandable_route =
    //     warp::path!("expandable").and(warp::fs::file("./static/expandable.html"));
    // let scalable_route = warp::path!("scalable").and(warp::fs::file("./static/scalable.html"));
    // let highly_available_route =
    //     warp::path!("highly-available").and(warp::fs::file("./static/highly-available.html"));
    // let full_stack_route =
    //     warp::path!("full-stack").and(warp::fs::file("./static/full-stack.html"));
    // let full_service_route =
    //     warp::path!("full-service").and(warp::fs::file("./static/full-service.html"));
    // let cloud_route = warp::path!("cloud").and(warp::fs::file("./static/cloud.html"));
    // let css_routes = warp::path!("css" / ..).and(warp::fs::dir("./static/css"));

    // let slack_webhook_url = config.get_str("slack.webhook").unwrap();
    // let message_route = warp::path!("message")
    //     .and(warp::post())
    //     .and(warp::body::content_length_limit(1024 * 20))
    //     .and(warp::body::form())
    //     .map(|form_map: HashMap<String, String>| {
    //         form_map
    //             .get("message")
    //             .map(|m| m.to_string())
    //             .unwrap_or("".to_string())
    //     })
    //     .and_then(move |msg: String| {
    //         let slack_webhook_url = slack_webhook_url.clone();
    // 			Ok(WithTemplate {
    // 			    name: "index",
    // 			    value: json!({ "success-msg": "Message received, expect a response within 24 hours" }),
    // 			})
    //         }
    //     })
    //     .map(handlebars.clone());

    let static_routes = index_route;
    // .or(expandable_route)
    // .or(scalable_route)
    // .or(highly_available_route)
    // .or(full_stack_route)
    // .or(full_service_route)
    // .or(cloud_route)
    // .or(css_routes)
    // .or(message_route);

    warp::serve(static_routes).run(([0, 0, 0, 0], 8080)).await;
}
