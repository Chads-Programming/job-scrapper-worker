#[cfg(test)] #[macro_use]
extern crate assert_matches;

use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    Response::ok("Hello, World!")
}
