# Progress

- Created a web UI capable of calling commands defined in windows.json
    - feat: create method for windows control, mode, resize using custom_protocol
    - testing: Create real nodejs server and communicate using stdin, stdout with ui
    - ✅ Creating the `CommandManager` class to manage running commands
    - ✅ Creating `CommandChild` to execute commands
    - ✅ Testing command execution via `call_command` on ResponseTools