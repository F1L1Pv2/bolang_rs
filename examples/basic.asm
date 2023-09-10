BITS 64
section .text
global _start
main:
    mov rdi, 69
    ret
    ret
_start:
    call main
    mov rax, 60
    syscall
