# This project is highly unstable on latest macOS versions and may have unstability on windows 10 and above. It is uncompiled and can lead to build errors. If you have any issues, feel free to file a ticket in the issues tab. 
# QuickClip

QuickClip is a fast, local clipboard manager built with Tauri. It captures text, links, and images instantly, stores history on disk, and never sends data off your machine.

## Key features
- Local-only clipboard history
- Searchable entries
- Global hotkey support
- Lightweight and low-latency
- Persistent storage via SQLite

## Getting started
1. Install dependencies
   ```bash
   npm install
   ```
2. Run in development mode
   ```bash
   npm run dev
   ```
3. Build for production
   ```bash
   npm run build
   ```

## Notes
- macOS hotkey: `Cmd+Shift+V`
- Close/hide: `Esc` or `Cmd/Ctrl+W`
- Data is stored locally and not synced externally

## License
MIT
