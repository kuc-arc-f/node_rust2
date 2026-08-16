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

pub fn get_htm_todo() -> String
{
  let ssr_htm: String = r##"<!doctype html>
<html lang="ja">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>TODOアプリ</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <!-- Font Awesome 6 (アイコン用) -->
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0-beta3/css/all.min.css" />
    <style>
        .d-none{ display: none; }
        /* 行にホバーしたときのエフェクト */
        .todo-row:hover {
            background-color: #f9fafb;
            cursor: pointer;
        }
        /* ラベルバッジのスタイル */
        .badge {
            font-size: 0.70rem;
            padding: 0.20rem 0.60rem;
            border-radius: 9999px;
            background-color: #e5e7eb;
            color: #1f2937;
        }
        .badge-urgent {
            background-color: #fecaca;
            color: #991b1b;
        }
        .badge-working {
            background-color: #fde68a;
            color: #92400e;
        }
        .badge-pending {
            background-color: #d1d5db;
            color: #374151;
        }
        /* ダイアログのオーバーレイ */
        .dialog-overlay {
            background-color: rgba(0, 0, 0, 0.4);
            backdrop-filter: blur(3px);
        }
        /* ダイアログアニメーション */
        .dialog-content {
            animation: fadeSlideIn 0.2s ease-out;
        }
        @keyframes fadeSlideIn {
            0% {
                opacity: 0;
                transform: scale(0.95) translateY(-10px);
            }
            100% {
                opacity: 1;
                transform: scale(1) translateY(0);
            }
        }
    </style>
</head>
<body class="bg-gray-100 flex items-center justify-center min-h-screen p-4 font-sans antialiased">

    <div class="w-full max-w-2xl bg-white rounded-2xl shadow-xl p-6 md:p-8 transition-all">
        <!-- ヘッダー -->
        <div class="flex items-center justify-between gap-3 mb-6">
            <h1 class="text-2xl font-bold text-gray-800 flex items-center gap-2">
                <i class="fas fa-list-check text-blue-500"></i> TODOリスト
            </h1>
            <button id="addTodoBtn" class="bg-blue-600 hover:bg-blue-700 text-white font-medium px-6 py-2.5 rounded-xl flex items-center justify-center gap-2 transition shadow-sm">
                <i class="fas fa-plus"></i> 追加
            </button>
        </div>

        <!-- リスト表示エリア -->
        <div id="todoListContainer" class="space-y-2 mt-2">
            <!-- ここに動的にTODOアイテムが表示されます -->
            <p class="text-gray-400 text-sm text-center py-8" id="emptyMessage">
                <i class="fas fa-inbox mr-2"></i> TODOがありません。追加してみましょう！
            </p>
        </div>

        <!-- フッター情報 -->
        <div class="mt-5 text-xs text-gray-400 border-t border-gray-100 pt-4 flex justify-between items-center">
            <span><i class="far fa-circle mr-1"></i> 行をクリックで詳細を表示 / 編集 で内容を変更できます</span>
            <span id="todoCount" class="bg-gray-200 px-3 py-0.5 rounded-full text-gray-600 text-xs">0件</span>
        </div>
    </div>

    <!-- 追加ダイアログ -->
    <div id="addDialog" class="fixed inset-0 flex items-center justify-center z-50 dialog-overlay hidden transition-opacity">
        <div class="bg-white w-full max-w-lg max-h-[90vh] overflow-y-auto mx-4 rounded-2xl shadow-2xl dialog-content p-6 relative">
            <button id="closeAddDialogBtn" class="absolute top-3 right-4 text-gray-400 hover:text-gray-700 transition text-xl">
                <i class="fas fa-times"></i>
            </button>

            <h2 class="text-xl font-semibold text-gray-800 mb-4 flex items-center gap-2">
                <i class="fas fa-plus text-blue-500"></i> TODOを追加
            </h2>

            <div class="space-y-4">
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">タイトル</label>
                    <input type="text" id="addTitle" placeholder="やることを入力..." class="w-full px-4 py-2 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-400" />
                </div>

                <div class="d-none">
                    <label class="block text-sm font-medium text-gray-700 mb-1">詳細メモ</label>
                    <textarea id="addMemo" rows="3" class="w-full px-4 py-2 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-400 resize-none" placeholder="詳細な説明を入力..."></textarea>
                </div>

                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">内容</label>
                    <input type="text" id="content" placeholder="内容を入力..." class="w-full px-4 py-2 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-400" />
                </div>

                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">公開設定</label>
                    <div class="flex items-center gap-6">
                        <label class="flex items-center gap-2 text-sm text-gray-700 cursor-pointer">
                            <input type="radio" name="public" value="公開" checked class="accent-blue-600" /> 公開
                        </label>
                        <label class="flex items-center gap-2 text-sm text-gray-700 cursor-pointer">
                            <input type="radio" name="public" value="非公開" class="accent-blue-600" /> 非公開
                        </label>
                    </div>
                </div>

                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">食材</label>
                    <div class="flex flex-wrap gap-4">
                        <label class="flex items-center gap-2 text-sm text-gray-700 cursor-pointer">
                            <input type="checkbox" id="food_orange" class="accent-orange-500" /> オレンジ
                        </label>
                        <label class="flex items-center gap-2 text-sm text-gray-700 cursor-pointer">
                            <input type="checkbox" id="food_apple" class="accent-red-500" /> りんご
                        </label>
                        <label class="flex items-center gap-2 text-sm text-gray-700 cursor-pointer">
                            <input type="checkbox" id="food_banana" class="accent-yellow-500" /> バナナ
                        </label>
                    </div>
                </div>

                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">公開日</label>
                    <input type="date" id="pub_date" class="w-full px-4 py-2 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-400 bg-white" />
                </div>

                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">数量</label>
                    <div class="grid grid-cols-3 gap-3">
                        <input type="number" id="qty1" placeholder="数量1" class="w-full px-3 py-2 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-400" />
                        <input type="number" id="qty2" placeholder="数量2" class="w-full px-3 py-2 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-400" />
                        <input type="number" id="qty3" placeholder="数量3" class="w-full px-3 py-2 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-400" />
                    </div>
                </div>

                <div class="flex flex-col sm:flex-row gap-3 pt-2">
                    <button id="registerTodoBtn" class="flex-1 bg-blue-600 hover:bg-blue-700 text-white font-medium py-2.5 rounded-xl transition shadow-sm flex items-center justify-center gap-2">
                        <i class="fas fa-check"></i> 登録
                    </button>
                    <button id="cancelAddBtn" class="flex-1 bg-gray-200 hover:bg-gray-300 text-gray-700 font-medium py-2.5 rounded-xl transition shadow-sm flex items-center justify-center gap-2">
                        <i class="fas fa-xmark"></i> キャンセル
                    </button>
                </div>
            </div>
        </div>
    </div>

    <!-- 詳細/編集ダイアログ -->
    <div id="detailDialog" class="fixed inset-0 flex items-center justify-center z-50 dialog-overlay hidden transition-opacity">
        <div class="bg-white w-full max-w-lg max-h-[90vh] overflow-y-auto mx-4 rounded-2xl shadow-2xl dialog-content p-6 relative">
            <!-- 閉じるボタン -->
            <button id="closeDialogBtn" class="absolute top-3 right-4 text-gray-400 hover:text-gray-700 transition text-xl">
                <i class="fas fa-times"></i>
            </button>

            <h2 class="text-xl font-semibold text-gray-800 mb-4 flex items-center gap-2">
                <i class="fas fa-pen-to-square text-blue-500"></i> TODO詳細 / 編集
            </h2>

            <!-- 編集フォーム -->
            <div class="space-y-4">
                <!-- タイトル -->
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">タイトル</label>
                    <input type="text" id="editTitle" class="w-full px-4 py-2 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-400" />
                </div>

                <!-- ステータス (ラベル) 選択 -->
                <div class="d-none">
                    <label class="block text-sm font-medium text-gray-700 mb-1">ステータス</label>
                    <select id="editStatus" class="w-full px-4 py-2 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-400 bg-white">
                        <option value="未着手"># 未着手</option>
                        <option value="作業中"># 作業中</option>
                        <option value="緊急"># 緊急</option>
                        <option value="完了"># 完了</option>
                    </select>
                </div>

                <!-- 詳細メモ -->
                <div class="d-none">
                    <label class="block text-sm font-medium text-gray-700 mb-1">詳細メモ</label>
                    <textarea id="editMemo" rows="3" class="w-full px-4 py-2 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-400 resize-none" placeholder="詳細な説明を入力..."></textarea>
                </div>

                <!-- 内容 -->
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">内容</label>
                    <input type="text" id="editContent" placeholder="内容を入力..." class="w-full px-4 py-2 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-400" />
                </div>

                <!-- 公開設定 -->
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">公開設定</label>
                    <div class="flex items-center gap-6">
                        <label class="flex items-center gap-2 text-sm text-gray-700 cursor-pointer">
                            <input type="radio" name="editPublic" value="公開" checked class="accent-blue-600" /> 公開
                        </label>
                        <label class="flex items-center gap-2 text-sm text-gray-700 cursor-pointer">
                            <input type="radio" name="editPublic" value="非公開" class="accent-blue-600" /> 非公開
                        </label>
                    </div>
                </div>

                <!-- 食材 -->
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">食材</label>
                    <div class="flex flex-wrap gap-4">
                        <label class="flex items-center gap-2 text-sm text-gray-700 cursor-pointer">
                            <input type="checkbox" id="editFoodOrange" class="accent-orange-500" /> オレンジ
                        </label>
                        <label class="flex items-center gap-2 text-sm text-gray-700 cursor-pointer">
                            <input type="checkbox" id="editFoodApple" class="accent-red-500" /> りんご
                        </label>
                        <label class="flex items-center gap-2 text-sm text-gray-700 cursor-pointer">
                            <input type="checkbox" id="editFoodBanana" class="accent-yellow-500" /> バナナ
                        </label>
                    </div>
                </div>

                <!-- 公開日 -->
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">公開日</label>
                    <input type="date" id="editPubDate" class="w-full px-4 py-2 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-400 bg-white" />
                </div>

                <!-- 数量 -->
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">数量</label>
                    <div class="grid grid-cols-3 gap-3">
                        <input type="text" id="editQty1" placeholder="数量1" class="w-full px-3 py-2 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-400" />
                        <input type="text" id="editQty2" placeholder="数量2" class="w-full px-3 py-2 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-400" />
                        <input type="text" id="editQty3" placeholder="数量3" class="w-full px-3 py-2 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-400" />
                    </div>
                </div>

                <!-- アクションボタン -->
                <div class="flex flex-col sm:flex-row gap-3 pt-2">

                    <button id="deleteTodoBtn" class="flex-1 bg-red-500 hover:bg-red-600 text-white font-medium py-2.5 rounded-xl transition shadow-sm flex items-center justify-center gap-2">
                        <i class="fas fa-trash-can"></i> 削除
                    </button>
                </div>
            </div>
        </div>
    </div>

    <script src="/js/client.js"></script>
</body>
</html>
  "##
    .to_string();

    return ssr_htm;
}

fn render_todo_list(todos: &[TodoItem]) -> String {
    let todo_items = todos
        .iter()
        .map(|todo| {
            let checked = if todo.completed { "checked" } else { "" };
            let title_class = if todo.completed {
                "line-through text-stone-400"
            } else {
                ""
            };
            format!(
                r##"
          <li class="group flex items-center justify-between p-4 hover:bg-stone-50 cursor-pointer transition-colors"
              hx-get="/api/todo/get/{id}"
              hx-target="#dialog-container"
              hx-swap="innerHTML">
            <div class="flex items-center gap-3">
              <span class="text-stone-800 font-medium {title_class}">{title}</span>
            </div>
            <form class="mt-4 flex gap-2" hx-post="/api/todo/delete" hx-target="#todo-container" hx-swap="outerHTML">
              <input type="hidden" name="id"  value="{id}" />
              <button class="text-stone-400 hover:text-red-500 opacity-0 group-hover:opacity-100 transition-opacity p-2"
                      onclick="event.stopPropagation()">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" />
                </svg>
              </button>
            </form>
          </li>"##,
                id = todo.id,
                title = todo.title,
                title_class = title_class,
            )
        })
        .collect::<Vec<_>>()
        .join("");

    let empty_state = if todos.is_empty() {
        r##"<li class="p-8 text-center text-stone-500">No tasks yet. Add one above!</li>"##
    } else {
        ""
    };

    format!(
    r##"
    <div class="bg-white w-3xl rounded-xl shadow-sm border border-stone-200 overflow-hidden" id="todo-container">
        <div>
            <a href="/" class="font-bold ms-4" >Home</a>
            <a href="/about" class="ms-4" >[ about ]</a>
            <hr class="my-2" />
        </div>
 
      <div class="p-6 border-b border-stone-200">
        <h1 class="text-2xl font-semibold text-stone-800">Todo List</h1>
        <form class="mt-4 flex gap-2" hx-post="/api/todo/create" hx-target="#todo-container" hx-swap="outerHTML">
          <input type="text" name="title" required placeholder="Add a new task..." class="flex-1 px-4 py-2 border border-stone-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-stone-500 focus:border-transparent" />
          <button type="submit" class="px-4 py-2 bg-stone-800 text-white rounded-lg hover:bg-stone-700 transition-colors font-medium">Add</button>
        </form>
      </div>
      <ul class="divide-y divide-stone-100">
{items}{empty_state}
      </ul>
      <div id="dialog-container"></div>
    </div>
  "##,
        items = todo_items,
        empty_state = empty_state,
    )
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
    //.expect("write! to String cannot fail");

    html
}
