use rusqlite::{Connection};
use std::vec::Vec;
use serde::Serialize;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use qstring::QString;

#[derive(Debug)]
#[derive(Serialize)]
struct SearchResult {
    title: String,
    url: String,
}


async fn greet(req: HttpRequest) -> impl Responder {
    println!("Start");
    let query_string = QString::from(req.query_string());
    let query = query_string.get("q").unwrap();
    let query_terms: Vec<String> = query.split_whitespace().map(str::to_string).collect();

    let conn = Connection::open("/home/daoud/data/tinysearch/index.sqlite3").unwrap();

    // let question_marks = std::iter::repeat("?, ").take(query_terms.len() - 1);
    let question_marks = std::iter::repeat("?").take(query_terms.len())
        .map(str::to_string).collect::<Vec::<String>>().join(", ");
    let mut stmt = conn.prepare(format!("
        SELECT title, url
        FROM terms INNER JOIN pages
        ON terms.page_id = pages.id
        WHERE term IN ({})
        GROUP BY title, url
        ORDER BY count(*) DESC, length(title)
        LIMIT 5
    ", question_marks).as_str()).unwrap();

   // let query = req.match_info().get("query").unwrap();
   println!("Query {}", query);
   let results = stmt.query_map(query_terms, |row| {
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
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}