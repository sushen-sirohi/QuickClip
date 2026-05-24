use arboard::Clipboard;
use chrono::Utc;
use rusqlite::{Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, State, Emitter};
use image::ImageFormat;
use std::io::Cursor;
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardEntry {
    pub id: i64,
    pub content: String,
    pub type_: String,
    pub timestamp: i64,
}

pub struct AppState {
    db: Arc<Mutex<Connection>>,
}

fn init_db() -> SqlResult<Connection> {
    let conn = Connection::open("clipboard_history.db")?;
    
    // 1. Create Base Table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT NOT NULL,
            timestamp INTEGER NOT NULL
        )",
        [],
    )?;
    
    // 2. Migration: Ensure 'type__' column exists
    let col_exists = conn.prepare("SELECT type__ FROM clipboard LIMIT 1").is_ok();
    
    if !col_exists {
        println!("Migration: Adding 'type__' column to clipboard table...");
        if let Err(e) = conn.execute("ALTER TABLE clipboard ADD COLUMN type__ TEXT DEFAULT 'text'", []) {
            eprintln!("Migration Warning: Could not add column (might exist?): {}", e);
        }
    }

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_timestamp ON clipboard(timestamp DESC)",
        [],
    )?;
    
    Ok(conn)
}

fn detect_type(content: &str) -> String {
    if content.starts_with("http://") || content.starts_with("https://") {
        return "link".to_string();
    }
    "text".to_string()
}

fn save_clipboard_entry(conn: &Connection, content: &str, entry_type: &str) -> SqlResult<()> {
    let timestamp = Utc::now().timestamp();
    
    let last_entry: Result<(String, String), _> = conn.query_row(
        "SELECT content, type__ FROM clipboard ORDER BY timestamp DESC LIMIT 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    );
    
    if let Ok((last_content, last_type)) = last_entry {
        if last_content == content && last_type == entry_type {
            return Ok(());
        }
    }
    
    conn.execute(
        "INSERT INTO clipboard (content, type__, timestamp) VALUES (?1, ?2, ?3)",
        [content, entry_type, &timestamp.to_string()],
    )?;
    
    Ok(())
}

#[tauri::command]
fn search_clipboard(query: String, state: State<AppState>) -> Result<Vec<ClipboardEntry>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    let mut stmt = db
        .prepare(
            "SELECT id, content, COALESCE(type__, 'text'), timestamp FROM clipboard 
             WHERE content LIKE ?1 AND type__ != 'image'
             ORDER BY timestamp DESC 
             LIMIT 100",
        )
        .map_err(|e| e.to_string())?;
    
    let search_pattern = format!("%{}%", query);
    
    let entries = stmt
        .query_map([search_pattern], |row| {
            Ok(ClipboardEntry {
                id: row.get(0)?,
                content: row.get(1)?,
                type_: row.get(2)?,
                timestamp: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();
    
    Ok(entries)
}

#[tauri::command]
fn get_recent_clipboard(limit: usize, offset: Option<usize>, type_filter: Option<String>, state: State<AppState>) -> Result<Vec<ClipboardEntry>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let offset_val = offset.unwrap_or(0);
    
    let base_query = "SELECT id, content, COALESCE(type__, 'text'), timestamp FROM clipboard";
    
    let (sql, params): (String, Vec<Box<dyn rusqlite::ToSql>>) = if let Some(ref t) = type_filter {
        if t == "all" {
             (format!("{} ORDER BY timestamp DESC LIMIT ?1 OFFSET ?2", base_query),
             vec![Box::new(limit), Box::new(offset_val)])
        } else {
             (format!("{} WHERE type__ = ?3 ORDER BY timestamp DESC LIMIT ?1 OFFSET ?2", base_query),
             vec![Box::new(limit), Box::new(offset_val), Box::new(t.clone())])
        }
    } else {
        (format!("{} ORDER BY timestamp DESC LIMIT ?1 OFFSET ?2", base_query),
        vec![Box::new(limit), Box::new(offset_val)])
    };

    let mut stmt = db.prepare(&sql).map_err(|e| e.to_string())?;
    
    let rows = stmt.query_map(rusqlite::params_from_iter(params.iter()), |row| {
        Ok(ClipboardEntry {
            id: row.get(0)?,
            content: row.get(1)?,
            type_: row.get(2)?,
            timestamp: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?;

    let entries = rows
        .filter_map(Result::ok)
        .collect();
    
    Ok(entries)
}

#[tauri::command]
fn copy_to_clipboard(content: String, entry_type: Option<String>) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
    let etype = entry_type.unwrap_or_else(|| "text".to_string());
    
    if etype == "image" {
        let bytes = general_purpose::STANDARD
            .decode(&content)
            .map_err(|e| e.to_string())?;
            
        let img = image::load_from_memory(&bytes).map_err(|e| e.to_string())?;
        let rgba = img.to_rgba8();
        
        let img_data = arboard::ImageData {
            width: img.width() as usize,
            height: img.height() as usize,
            bytes: std::borrow::Cow::Owned(rgba.into_vec()),
        };
        
        clipboard.set_image(img_data).map_err(|e| e.to_string())?;
    } else {
        clipboard.set_text(content).map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

#[tauri::command]
fn delete_entry(id: i64, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.execute("DELETE FROM clipboard WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn clear_history(state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.execute("DELETE FROM clipboard", [])
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn start_clipboard_monitor(app_handle: AppHandle) {
    thread::spawn(move || {
        let mut clipboard = loop {
            match Clipboard::new() {
                Ok(cb) => break cb,
                Err(e) => {
                    eprintln!("Failed to init clipboard module: {}. Retrying in 2s...", e);
                    thread::sleep(Duration::from_secs(2));
                }
            }
        };
        
        println!("Clipboard monitor active.");
        
        let mut last_text = String::new();
        let mut last_img_hash = 0u64; 
        
        loop {
            thread::sleep(Duration::from_millis(1000)); 
            
            let mut updated = false;

            match clipboard.get_text() {
                Ok(content) => {
                    if content != last_text && !content.is_empty() {
                        last_text = content.clone();
                        updated = true;
                        
                        if let Some(state) = app_handle.try_state::<AppState>() {
                            if let Ok(db) = state.db.lock() {
                                let type_ = detect_type(&content);
                                if let Err(e) = save_clipboard_entry(&db, &content, &type_) {
                                     eprintln!("Error saving text: {}", e);
                                     // Prevent loop spam if DB is flawed
                                     // But updated was true, so it will try again next text change
                                }
                            }
                        }
                    }
                },
                Err(_) => {} 
            }
            
            if !updated {
                if let Ok(img) = clipboard.get_image() {
                    use std::collections::hash_map::DefaultHasher;
                    use std::hash::{Hash, Hasher};
                    
                    let mut hasher = DefaultHasher::new();
                    img.bytes.hash(&mut hasher);
                    let hash = hasher.finish();
                    
                    if hash != last_img_hash && hash != 0 {
                        last_img_hash = hash;
                        
                        let width = img.width as u32;
                        let height = img.height as u32;
                        
                        if let Some(buf) = image::RgbaImage::from_raw(width, height, img.bytes.into_owned()) {
                             let mut bytes: Vec<u8> = Vec::new();
                             let mut cursor = Cursor::new(&mut bytes);
                             
                             if let Ok(_) = buf.write_to(&mut cursor, ImageFormat::Png) {
                                 let base64_str = general_purpose::STANDARD.encode(&bytes);
                                 
                                 if let Some(state) = app_handle.try_state::<AppState>() {
                                    if let Ok(db) = state.db.lock() {
                                        let _ = save_clipboard_entry(&db, &base64_str, "image");
                                        updated = true;
                                    }
                                }
                             }
                        }
                    }
                }
            }

            if updated {
                let _ = app_handle.emit("clipboard-updated", ());
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let conn = init_db().expect("Failed to initialize database");
    let state = AppState {
        db: Arc::new(Mutex::new(conn)),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            search_clipboard,
            get_recent_clipboard,
            copy_to_clipboard,
            delete_entry,
            clear_history,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            start_clipboard_monitor(app_handle.clone());
            
            let window = app.get_webview_window("main").expect("Failed to get main window");
            let w_clone = window.clone();
            
            window.on_window_event(move |event| {
                 match event {
                     tauri::WindowEvent::CloseRequested { api, .. } => {
                         api.prevent_close();
                         let _ = w_clone.hide();
                     }
                     _ => {}
                 }
            });
            
            let show_i = MenuItem::with_id(app, "show", "Show QuickClip", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit QuickClip", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;
            
            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "show" => { if let Some(w) = app.get_webview_window("main") { let _ = w.show(); let _ = w.set_focus(); } }
                    "quit" => std::process::exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                        let app = tray.app_handle();
                        if let Some(w) = app.get_webview_window("main") {
                            if w.is_visible().unwrap_or(false) { let _ = w.hide(); }
                            else { let _ = w.show(); let _ = w.set_focus(); }
                        }
                    }
                })
                .build(app)?;
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
