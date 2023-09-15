;infinite loop

loop:
  jmp loop

times 510-($-$$) db 0
; the epic magic number
dw 0xaa55
