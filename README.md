# ClipMon

Clipboard Monitoring during RDP Session.

## TODO

- [X] Basic injection hooking `MessageBoxA`
- [X] CreateProcess with `DEBUG_PROCESS`
- [X] Proper logging system
- [X] Hook `SetClipboardData`
- [X] Hook `GetClipboardData`
- [X] Use proper lib (clipboard-win) to handle the clipboard in the hooked function
- [ ] Monitoring code in the detour functions
- [ ] Setup config file
- [ ] Update `README.md` with *installation* and *setup* instructions
