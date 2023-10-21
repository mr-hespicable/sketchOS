TODO list for now:

- [x] create delete_byte func in vga buffer (10/17)
- [x] implement backspaces in interrupts.rs (10/18)
- [x] make backspace work across rows (10/19)
- [ ] make the typing move letters to the next line (move chars fn)
    - [ ] implement moving across rows (column of character. if column = buffer width - 1, move to next line. then, go to next row and continue.)
    - [ ] implement backspaces moving characters left.
- [ ] make arrow keys work across rows
- [ ] add blinking cursor
- [ ] add a proper interface (name@env:/path$)


TODO for the future:
- [ ] file management system
- [ ] add cli