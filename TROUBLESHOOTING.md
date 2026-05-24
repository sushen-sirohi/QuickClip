# Troubleshooting

## Global Shortcut Not Working

If `Cmd+Shift+V` doesn't work:

1. **Check if another app is using the same shortcut**
   - Some clipboard managers or screenshot tools use this combination
   - Try changing the shortcut in `src/main.js` (search for `CommandOrControl+Shift+V`)

2. **Permissions Issue (macOS)**
   - Go to System Preferences > Security & Privacy > Privacy > Accessibility
   - Add the Clipboard Manager app to the list

3. **Use the System Tray Instead**
   - Left-click the tray icon to toggle the window
   - This always works regardless of shortcut issues

## App Not Starting in Background

If you see the window on startup instead of running silently:

- Check `tauri.conf.json` - ensure `visible: false` is set
- The app should start hidden with only a tray icon visible

## Clipboard Not Being Monitored

If new clipboard items aren't showing up:

1. **Restart the app** - The clipboard monitor runs in a background thread
2. **Check permissions** - Some OS versions require clipboard access permissions
3. **Check the developer console** - Look for any error messages

## Window Loses Focus Too Quickly

If the window hides immediately when you try to interact:

- This is the "hide on blur" feature
- It's designed to be quick-access
- The 200ms delay can be adjusted in `src/main.js` (search for `onFocusChanged`)

## Building/Development Issues

### Rust Compilation Errors

Make sure you have the Rust toolchain installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Node/NPM Issues

```bash
# Clear cache and reinstall
rm -rf node_modules package-lock.json
npm install
```

### Tauri CLI Issues

```bash
# Reinstall Tauri CLI
npm uninstall @tauri-apps/cli
npm install --save-dev @tauri-apps/cli@latest
```

## Database Issues

If clipboard history is corrupted or not loading:

```bash
# Delete the database (it will be recreated)
rm clipboard_history.db
```

The database file is located in the same directory as the app executable.

## Performance Issues

If the app feels slow:

1. **Clear old history** - Click "Clear All" to remove old entries
2. **Check CPU usage** - The clipboard monitor should use minimal resources
3. **Reduce polling frequency** - Edit `src-tauri/src/lib.rs` and increase the `Duration::from_millis(500)` value

## Getting Help

If you encounter other issues:

1. Check the logs in the developer console (`Cmd+Shift+I` or `Ctrl+Shift+I`)
2. Look for error messages in the terminal when running in dev mode
3. Create an issue on GitHub with details about your system and the error
