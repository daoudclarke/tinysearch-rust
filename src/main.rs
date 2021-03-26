use rusqlite::{Connection, Result, MappedRows};
use std::path::Path;
use std::env::var;
use std::vec::Vec;
use serde::Serialize;
use std::io::Write;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

#[derive(Debug)]
#[derive(Serialize)]
struct SearchResult {
    title: String,
    url: String,
}


async fn greet(req: HttpRequest) -> impl Responder {
    println!("Start");
    let conn = Connection::open("/home/daoud/data/tinysearch/index.sqlite3").unwrap();
    let mut stmt = conn.prepare("
        SELECT title, url
        FROM terms INNER JOIN pages
        ON terms.page_id = pages.id
        WHERE term = ?
        GROUP BY title, url
        ORDER BY count(*) DESC, length(title)
        LIMIT 5
    ",).unwrap();

    let query = req.match_info().get("query").unwrap();
    println!("Query {}", query);
    let results = stmt.query_map(vec![query], |row| {
        Ok(SearchResult {
            title: row.get(0)?,
            url: row.get(1)?,
        })
    });

    let mut search_results = Vec::new();
    for result in results.unwrap() {
        search_results.push(result.unwrap());
    }

    println!("Result {:?}", search_results);
    web::Json(search_results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/{query}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}