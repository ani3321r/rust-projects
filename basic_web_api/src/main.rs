mod models;
mod database;
mod handlers;

use models::*;
use database::Database;
use handlers::*;

use iron::prelude::Chain;
use iron::Iron;
use router::Router;
use logger::Logger;
use uuid::Uuid;

fn main() {
  env_logger::init();
  let (logger_before, logger_after) = Logger::new(None);
  let mut db = Database::new();
  let p1 = Post::new(
      "First Post",
      "First Api post",
      "Raiden",
      chrono::offset::Utc::now(),
      Uuid::new_v4()
    );
  db.add_post(p1);

  let p2 = Post::new(
      "Next Post",
      "Second Api Post",
      "Fatbrad",
      chrono::offset::Utc::now(),
      Uuid::new_v4()
    );
  db.add_post(p2);

  let handlers = Handlers::new(db);
  let json_content_middleware = JsonAfterMiddleware;

  let mut router = Router::new();
  router.get("/post_feed", handlers.post_feed, "post_feed");
  router.post("/post", handlers.post_post, "post_post");
  router.get("/post/:id", handlers.post, "post");

  let mut chain = Chain::new(router);
  chain.link_before(logger_before);
  chain.link_after(json_content_middleware);
  chain.link_after(logger_after);

  Iron::new(chain).http("localhost:8080").unwrap();
}
