import std/strformat
import winim/lean
import nclip

# subscribe to the clipboard event
if not AddClipboardFormatListener():
  quit "could not listen for clipboard event: " & $GetLastError()

# listen for clipboard event
echo "waiting for clipboard event"
var msg: MSG
while GetMessage(msg):
  if msg.message == WM_CLIPBOARDUPDATE:
    let (dtype, data) = GetClipboardData()
    if dtype == CP_TEXT:
      echo fmt"Got {data}"
