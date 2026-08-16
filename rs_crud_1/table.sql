CREATE TABLE IF NOT EXISTS todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content TEXT,
    public INTEGER NOT NULL DEFAULT 0,
    food_orange INTEGER DEFAULT 0,
    food_apple INTEGER DEFAULT 0,
    food_banana INTEGER DEFAULT 0,
    pub_date TEXT,                       
    qty1 INTEGER DEFAULT 0,              
    qty2 INTEGER DEFAULT 0,              
    qty3 INTEGER DEFAULT 0,              
    created_at TEXT DEFAULT (DATETIME('now', 'localtime'))
);