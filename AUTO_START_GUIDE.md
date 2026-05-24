# Auto-Start — QuickClip

Make QuickClip start automatically on boot.

## macOS
- System Settings: Apple menu → System Settings → General → Login Items → + → select QuickClip.
- Older macOS: System Preferences → Users & Groups → Login Items → + → add QuickClip.
- In-app: open app menu and enable "Start at Login" if available.

Tip: If using a dev build, run from your project and enable the option in the app menu.

## Windows
- Startup folder:
  1. Press Win+R → type `shell:startup` → Enter
  2. Right-click → New → Shortcut → point to QuickClip.exe
- Task Scheduler (advanced): Create Basic Task → Trigger: "When I log on" → Action: Start a program → point to QuickClip.exe

## Linux
- Autostart (most desktop environments):
  mkdir -p ~/.config/autostart
  Create `~/.config/autostart/quickclip.desktop` with:
  ```ini
  [Desktop Entry]
  Type=Application
  Name=QuickClip
  Exec=/full/path/to/quickclip
  X-GNOME-Autostart-enabled=true
  ```
- Systemd (user): create `~/.config/systemd/user/quickclip.service`, enable & start with `systemctl --user enable --now quickclip.service`.

## Verify
- Reboot or log out → log in
- Look for tray/menu-bar icon
- Use hotkey: Cmd+Shift+V (mac) or Ctrl+Shift+V (Win/Linux)

## Quick Troubleshooting
- Not listed at login: re-add via Login Items / Startup folder / ~/.config/autostart
- App launches but no tray icon: start app manually to see errors; check permissions
- Linux systemd: `journalctl --user -u quickclip` for logs

## Best Practices
- Start minimized so QuickClip runs in background
- Quit only from the tray menu so state saves properly

## Privacy
QuickClip runs locally and does not send clipboard data over the internet.
```
