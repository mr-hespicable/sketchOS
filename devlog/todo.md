TODO list for now:

- [x] create delete_byte func in vga buffer (10/17)
- [x] implement backspaces in interrupts.rs (10/18)
- [x] make backspace work across rows (10/19)
- [ ] make the typing move letters to the next line `(vga_buffer::move_chars)`
    - [ ] implement moving across rows 
    - [ ] implement backspaces moving characters left
- [ ] make arrow keys work across rows
- [ ] add blinking cursor (reverse highlight)
- [ ] add a proper interface (name@env:/path$)
    - [ ] implement `vga_buffer::move_chars` with this interface


TODO for the future:
- [ ] file management system
- [ ] add cli
- [ ] add unix stuff
