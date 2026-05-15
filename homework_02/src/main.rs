use actix_web::{App, HttpResponse, HttpServer, Responder, middleware::Logger, web};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// ---------- 图书数据结构 ----------
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Book {
    id: u32,
    title: String,
    author: String,
}

// 全局内存数据库（Mutex 保证线程安全）
type BookDb = Mutex<Vec<Book>>;

// ---------- 路由处理器 ----------
// 任务一：返回 "Hello, Rust!"
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Rust!")
}

// 任务二：根路径返回自定义欢迎页面（HTML 字符串）
async fn welcome() -> impl Responder {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head><title>Welcome</title><style>body { font-family: sans-serif; text-align: center; margin-top: 50px; }</style></head>
        <body>
            <h1>Book Manage System</h1>
            <p>Welcome to Rust Web Server !</p>
            <p>Visit <code>/books</code> to see book list (GET)</p>
            <p>Use POST request <code>/books</code> to add new book (JSON format)</p>
        </body>
        </html>
    "#;
    HttpResponse::Ok().content_type("text/html").body(html)
}

// 任务三：GET /books - 获取所有书籍
async fn get_books(db: web::Data<BookDb>) -> impl Responder {
    let books = db.lock().unwrap().clone();
    HttpResponse::Ok().json(books)
}

// 任务三：POST /books - 添加新书
async fn add_book(db: web::Data<BookDb>, new_book: web::Json<Book>) -> impl Responder {
    let mut books = db.lock().unwrap();
    // 简单生成新 id（模拟自增）
    let next_id = books.iter().map(|b| b.id).max().unwrap_or(0) + 1;
    let mut book = new_book.into_inner();
    book.id = next_id;
    books.push(book.clone());
    HttpResponse::Created().json(book) // 返回添加后的书籍（包含 id）
}

// ---------- 启动服务器 ----------
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志（tracing 结合 actix-web 的 Logger 中间件）
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .with_level(true)
        .init();

    // 准备共享数据：初始两本示例书
    let initial_books = vec![
        Book {
            id: 1,
            title: "Journey to the West".to_string(),
            author: "Wu Cheng'en".to_string(),
        },
        Book {
            id: 2,
            title: "Romance of the Three Kingdoms".to_string(),
            author: "Luo Guanzhong".to_string(),
        },
    ];
    let book_db = web::Data::new(Mutex::new(initial_books));

    println!("🚀 Run Server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            // 任务四：日志中间件（记录路径、状态码、耗时）
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i %r %s %Dms"))
            .app_data(book_db.clone())
            .route("/hello", web::get().to(hello))
            .route("/", web::get().to(welcome))
            .service(
                web::scope("/books")
                    .route("", web::get().to(get_books))
                    .route("", web::post().to(add_book)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
