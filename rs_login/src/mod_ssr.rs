use serde::{Deserialize, Serialize};
use serde_json::json;
//use std::fmt::Write;
use std::fs;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub completed: bool,    
}

pub fn get_htm_top() -> String
{
  let ssr_htm: String = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>HTMX Todo App</title>
    <script src="https://unpkg.com/htmx.org@1.9.12"></script>
    <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
    <script src="/js/client.js"></script>
  </head>
  <body class="bg-stone-100 min-h-screen flex justify-center text-stone-900 font-sans">
    <div class="max-w-3xl mx-auto p-4 py-12" hx-get="/api/todo/list" hx-trigger="load" hx-swap="outerHTML">
      Top
      <div class="flex justify-center mt-12">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-stone-800"></div>
      </div>
    </div>
  </body>
  
</html>
"#
    .to_string();

    return ssr_htm;
}

pub fn get_htm_about() -> String
{
  let ssr_htm: String = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>HTMX Todo App</title>
    <script src="https://unpkg.com/htmx.org@1.9.12"></script>
    <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
  </head>
  <body class="bg-white min-h-screen text-stone-900 font-sans">
    <div>
      <a href="/" class="font-bold ms-4" >Home</a>
      <a href="/about" class="ms-4" >[ about ]</a>
      <hr class="my-2" />
    </div>      
    <div class="max-w-3xl mx-auto p-4 py-12" >
      <h1 class="font-bold text-xl" >About</h1>
      <hr /> 
    </div>
  </body>
  
</html>
"#
    .to_string();

    return ssr_htm;
}

pub fn render_dialog(todo: &TodoItem) -> String {
    let status_badge_class = if todo.completed {
        "bg-green-100 text-green-800"
    } else {
        "bg-yellow-100 text-yellow-800"
    };
    let status_label = if todo.completed { "Completed" } else { "Pending" };

    let mut html = String::new();
   html = format!(
        r##"
    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50" id="todo-modal" onclick="this.remove()">
      <div class="bg-white rounded-xl shadow-xl w-full max-w-md overflow-hidden" onclick="event.stopPropagation()">
        <div class="p-6">
          <div class="flex justify-between items-start mb-4">
            <h2 class="text-xl font-semibold text-stone-800">{}</h2>
            <button class="text-stone-400 hover:text-stone-600 transition-colors" onclick="document.getElementById('todo-modal').remove()">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>

          <form hx-put="/api/todos/{}" hx-target="#todo-container" hx-swap="outerHTML">
            <div class="space-y-4">
              <div>
                <label class="block text-sm font-medium text-stone-700 mb-1">Status</label>
                <div class="flex items-center gap-2">
                  <span class="px-2.5 py-0.5 rounded-full text-xs font-medium {}">{}</span>
                </div>
              </div>
            </div>

            <div class="mt-6 flex justify-end gap-3">
              <button type="button" class="px-4 py-2 text-stone-700 hover:bg-stone-100 rounded-lg transition-colors font-medium" onclick="document.getElementById('todo-modal').remove()">Cancel</button>
            </div>
          </form>
        </div>
      </div>
    </div>
  "##,
        todo.title,
        todo.id,
        status_badge_class,
        status_label
    );
 
    html
}

pub fn get_htm_login() -> String
{
  let ssr_htm: String = r##"<!DOCTYPE html>
<html lang="ja">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>ログイン画面</title>
  <script src="https://unpkg.com/htmx.org@1.9.12"></script>
  <script src="https://cdn.tailwindcss.com"></script>
  <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0-beta3/css/all.min.css">
</head>
<body class="bg-gray-100 flex items-center justify-center min-h-screen font-sans antialiased">
  <div class="w-full max-w-md p-8 bg-white rounded-2xl shadow-xl border border-gray-200/60 transition-all">
    
    <h1 class="text-3xl font-bold text-gray-800 text-center tracking-tight">Login</h1>
    <p class="text-gray-500 text-center mt-1 text-sm font-medium">アカウントにログインしてください</p>

    <form class="mt-8 space-y-5" 
        hx-post="/api/user/login"
        hx-trigger="submit"
        hx-target="#h2"
        hx-on=""
        hx-on--after-request="after_login();">
      <div>
        <label for="email" class="block text-sm font-semibold text-gray-700 mb-1">メールアドレス</label>
        <div class="relative">
          <span class="absolute inset-y-0 left-3 flex items-center text-gray-400 text-sm">
            <i class="fas fa-envelope"></i>
          </span>
          <input type="email" id="email" name="email" value="" required
                 class="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-xl text-gray-700 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:border-transparent transition">
        </div>
      </div>

      <div>
        <div class="flex items-center justify-between mb-1">
          <label for="password" class="block text-sm font-semibold text-gray-700">パスワード</label>
          <a href="#" class="text-sm text-blue-600 hover:text-blue-800 font-medium hover:underline transition">お忘れですか？</a>
        </div>
        <div class="relative">
          <span class="absolute inset-y-0 left-3 flex items-center text-gray-400 text-sm">
            <i class="fas fa-lock"></i>
          </span>
          <input type="password" id="password" name="password" placeholder="パスワードを入力" required
            class="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-xl text-gray-700 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:border-transparent transition">
        </div>
      </div>

      <button type="submit" 
        class="w-full flex items-center justify-center gap-2 bg-blue-600 hover:bg-blue-700 text-white font-bold py-3.5 px-4 rounded-xl transition shadow-md hover:shadow-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2">
        <i class="fas fa-lock text-white text-sm"></i>
        ログイン
      </button>
      <div>
        <h3 id="h2"></h3>
      </div>

      <div class="text-center text-sm text-gray-600 pt-2">
        アカウントをお持ちでないですか？ 
        <a href="#" class="text-blue-600 hover:text-blue-800 font-semibold hover:underline transition">新規登録</a>
      </div>
    </form>

    <div class="mt-4 text-center text-xs text-gray-400 border-t border-gray-100 pt-4">
      <span>セキュアなログイン 🔒</span>
    </div>
    <div id="user-container"></div>
  </div>
  <script>function after_login(){
    const v = document.getElementById("result_login").value
    if(v){ location.href = "/"; }
  }
  </script>
</body>
</html>
"##
    .to_string();

    return ssr_htm;
}