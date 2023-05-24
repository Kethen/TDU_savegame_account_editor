### Features
- Toggle on/offline
- Edit nickname, email and password
- Import progress from other saves (playersave files more specifically)

** you are advised to manually back up your saves at `%USERPROFILE%/Documents/Test Drive Unlimited/savegame` as well in case of unforeseen file corruptions **

Ghidra, x64dbg, ghex and vbindiff were used to inspect save file serialization, see https://github.com/Kethen/TDU_savegame_account_editor/blob/main/offsets_and_formats.md for a summary.

iced-rs is used for the user interface, fs_extra is used for recursive file copying, rfd is used for providing a file picker, tar-rs is used for preserving bookmarks.

tdudec on rust is ported from the source release of tdudec at http://aluigi.altervista.org/papers.htm#others-file.

.exe for windows (win32 and win64), without for linux (x86_64 glibc).

Alternatively the project can be built using cargo like other rust projects, it should build in MacOS as well.
