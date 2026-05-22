Rust 实验二任务列表

任务一: 使用 hyper 或 actix-web 库编写一个简单的 HTTP 服务器，当访问 /hello 路径时返回 "Hello, Rust!" 文本响应。

任务二: 扩展基础 HTTP 服务器，使其能托管静态文件（如 HTML/CSS)，当访问根路径 / 时返回一个自定义的欢迎页面。

任务三: 用 serde 和 actix-web 实现一个图书管理 API，支持 GET/books（获取所有书籍）和 POST/books（添加新书籍)，数据暂存内存中。

任务四: 为 Web 服务器添加日志中间件，记录每个请求的路径、响应状态码和耗时，使用 tracing 或 log 库输出到控制台。

---

## 运行与验证

1. **启动服务器**  
   ```bash
   cargo run
   ```

2. **测试任务一**  
   浏览器访问 `http://127.0.0.1:8080/hello` → 显示 `Hello, Rust!`

3. **测试任务二**  
   访问 `http://127.0.0.1:8080/` → 显示自定义欢迎页面

4. **测试任务三**  
   - 获取所有书籍：`curl http://127.0.0.1:8080/books`  
   - 添加新书：  
     ```bash
     curl -X POST http://127.0.0.1:8080/books \
       -H "Content-Type: application/json" \
       -d "{\"id\":3,\"title\":\"Water Margin\",\"author\":\"Shi Nai'an\"}"
     ```

5. **任务四**  
   控制台会输出类似下面的日志（自动记录每个请求的路径、状态码、耗时）：
   ```
   [2025-XX-XXT...] INFO: 127.0.0.1:12345 "GET /books HTTP/1.1" 200 0.001ms
   [2025-XX-XXT...] INFO: 127.0.0.1:12346 "POST /books HTTP/1.1" 201 0.002ms
   ```

---

## 代码说明

- **任务一 & 二**：分别由 `hello` 和 `welcome` 处理器实现。  
- **任务三**：使用 `web::Data<Mutex<Vec<Book>>>` 作为全局内存存储；`GET /books` 返回所有书籍，`POST /books` 添加新书并自动分配递增 ID。  
- **任务四**：通过 `.wrap(Logger::default())` 和更详细的日志格式记录每个请求的路径、状态码和耗时（毫秒）。  

所有功能都已集成在同一个服务器中，符合“层层递进”的设计。



<https://chat.deepseek.com/a/chat/s/2cc06920-011b-4cbb-b1c7-1092ea89613a>

<https://chat.deepseek.com/a/chat/s/c8f168ab-3fd6-4d57-aa86-641375311fb3>