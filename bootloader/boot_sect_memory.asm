mystring:
  db 'Hello, World!', 0


print:
  mov ah, 0x0e
  int 0x10
  jmp endprint
  
