# Always Running Mode

This app is designed to keep running in the background unless you explicitly quit it. Closing the window or pressing escape hides the UI — it doesn't stop the clipboard monitor.

## How it behaves

When the app stays running:
- Close the window (X) → window hides, app keeps running
- Press `ESC` → window hides, app keeps running
- Press `Cmd+W` / `Ctrl+W` → window hides, app keeps running
- Press `Cmd+Shift+V` / `Ctrl+Shift+V` to toggle the window off → window hides, monitor continues
- Click an item to copy → item copies and the window auto-hides
- Your computer sleeps or you switch apps → the monitor keeps running

When the app quits:
- Right-click the tray icon → choose "Quit Clipboard Manager"
- Or use system force-quit (not recommended)

## Visual cues

App running
- You’ll see a tray/menu-bar icon (📋). If the icon is visible, the app is running.

App not running
- No tray icon
- Hotkey won’t open the window
- Clipboard isn’t being monitored

## Why this design

Traditional apps often quit when you close their window. For a clipboard manager that would be annoying — you’d lose history or have to wait for it to restart. Keeping the app running in the background means:
- clipboard history is always recorded
- the app is instantly ready when you press the hotkey
- closing the window just hides the UI, it doesn’t stop the service

Flow:
Start app → runs in background → close window = hide UI → press hotkey = show UI

## Typical usage

Morning
- Boot computer (app auto-starts if configured)
- Tray icon appears — app is running

During the day
- Copy text / code / URLs — they’re saved automatically
- Need something from history? Press `Cmd+Shift+V` → pick the item → it’s copied and the window hides

Evening
- Close laptop — app stops when the system shuts down
- Next day, it can auto-start and your history is preserved

## Window vs App

The app = background process that monitors the clipboard, stores history, and responds to a global hotkey.

The window = a UI to search/view/copy items. Showing or hiding the window doesn’t affect the background process.

## Quick technical note

Instead of exiting when the window is closed, we prevent the exit and hide the window.

Rust example:
```rust
.run(|app_handle, event| {
    if let RunEvent::ExitRequested { api, .. } = event {
        api.prevent_exit();  // keep running
        window.hide();       // just hide the window
    }
})
```

Frontend example:
```javascript
currentWindow.onCloseRequested((event) => {
  event.preventDefault(); // don't quit
  currentWindow.hide();   // hide the UI
});
```

## Benefits

- You don’t lose clipboard history
- Instant access, no startup delay
- Low CPU and memory usage
- Seamless experience — behaves like part of the OS

## How to check if it’s running

Quick: look for the tray/menu-bar icon.

Terminal/Task Manager:
- macOS / Linux:
  ```
  ps aux | grep clipboard-manager
  ```
- Windows: open Task Manager → Details → look for clipboard-manager.exe

## How to fully quit

Recommended: Right-click the tray icon → choose "Quit Clipboard Manager".

Force quit only if necessary:
- macOS: Cmd+Option+Esc → Force Quit
- Windows: Ctrl+Shift+Esc → Task Manager → End Task
- Linux: `pkill clipboard-manager`

## Summary

- Closing the window hides the UI but keeps the app monitoring the clipboard.
- To stop the app completely, use the tray menu or the system’s process manager.
- Let it run — it’s lightweight and keeps your clipboard history available.

That's it — the app is meant to run quietly in the background so you can rely on it whenever you need something from your clipboard history.
