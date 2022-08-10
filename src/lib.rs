use worker::kv::{GetOptionsBuilder, KvStore, ListResponse};
use worker::*;

mod utils;

const LINK_STORE: &str = "LINKS";

async fn retrieve_link_from_link_store(path: &str, env: &Env) -> Result<Response> {
    let link_store: KvStore = env.kv(LINK_STORE)?;

    let path = path.strip_prefix("/").unwrap_or(path);
    if path.is_empty() {
        utils::log_not_present_error("LINKS", path);
        return Response::error(
            utils::create_error_response(
                "Not Found",
                "404 Not Found",
                "Oops, looks like we weren't able to find the link you were looking for.",
            ),
            404,
        );
    }
    let result: GetOptionsBuilder = link_store.get(path);
    return match result.text().await? {
        Some(link) => {
            let mut redirect_headers: Headers = Headers::new();
            redirect_headers.set("Location", &link)?;
            Ok(Response::empty()?.with_headers(redirect_headers).with_status(302))
        },
        None => {
            utils::log_not_present_error("LINKS", path);
            Response::error(
                utils::create_error_response(
                    "Not Found",
                    "404 Not Found",
                    "Oops, looks like we weren't able to find the link you were looking for.",
                ),
                404,
            )
        }
    };
}

async fn list_links(env: &Env) -> Result<Response> {
    let link_store: KvStore = env.kv(LINK_STORE)?;

    let links: ListResponse = link_store.list().execute().await?;

    let mut page: String = format!(
        r#"<!DOCTYPE html><html lang="en"><head><title>Links</title>{header}</head><body><div class="content"><h1>Links</h1>"#,
        header = utils::MINIMAL_HEADER
    );

    for link in &links.keys {
        let redirect: String = link_store.get(link.name.as_str()).text().await?.unwrap();

        page.push_str("<p><a class=\"hover-link\" href=\"");
        page.push_str(redirect.as_str());
        page.push_str("\">");
        page.push_str(link.name.as_str());
        page.push_str("</a></p>");
    }

    page.push_str("</div></body>");

    return Response::from_html(page);
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    router
        .get_async("/", |_, ctx| async move {
            return match list_links(&ctx.env).await {
                Ok(response) => Ok(response),
                Err(e) => {
                    utils::log_generic_error("/", &e.to_string());
                    // Generic error message
                    Response::error(utils::create_error_response("Bad Request", "500 Internal Server Error", "Sorry, something went wrong and we're unable to handle your request."), 500)
                }
            }
        })
        .get_async("/:path", |_, ctx| async move {
            if let Some(path) = ctx.param("path") {
                return match retrieve_link_from_link_store(path, &ctx.env).await {
                    Ok(response) => Ok(response),
                    Err(e) => {
                        utils::log_generic_error(path, &e.to_string());
                        // Generic error message
                        Response::error(utils::create_error_response("Bad Request", "500 Internal Server Error", "Sorry, something went wrong and we're unable to handle your request."), 500)
                    }
                };
            } else {
                // No path parameter - bad client request
                return Response::error(utils::create_error_response("Bad Request", "400 Bad Request", "Looks like that's not a valid path on this server!"), 400);
            }
        })
        .get_async("/:path/*extra", |_, _| async move {
            return Response::error(
                utils::create_error_response(
                    "Not Found",
                    "404 Not Found",
                    "Oops, looks like we weren't able to find the link you were looking for.",
                ),
                404,
            )
        })
        .run(req, env)
        .await
}
