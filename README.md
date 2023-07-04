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

## Testing

If you wanna test it you will need:  

- Windows Server with RDP
- RDP Client
- Set IFEO `rdpclip.exe` with `debugger` set to `path/to/clipmon_injector`
- Do random copy/paste from server to client and vice versa
- Check the logs at `path/to/clipmon_injector/clipmon.log`
