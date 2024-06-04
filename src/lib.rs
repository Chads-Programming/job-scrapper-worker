#[cfg(test)] #[macro_use]
extern crate assert_matches;
use worker::console_debug;


use worker::*;

mod jobs;
mod providers;
mod indeed;

#[event(fetch)]
pub async fn main(_: Request, _: Env, _: Context) -> Result<Response> {

    let jobs = jobs::fetch_jobs().await;
    console_debug!("{:?}", jobs);

    Response::ok("Hello, World!")
}
