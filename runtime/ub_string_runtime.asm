format ELF64

section '.text' executable

public ub_str_concat
public ub_str_eq

; rdi: const char* a
; rsi: const char* b
; returns rax = 1 if equal, else 0
ub_str_eq:
    push rbx
    mov rbx, rdi
    mov rcx, rsi
.cmp_loop:
    mov al, [rbx]
    mov dl, [rcx]
    cmp al, dl
    jne .not_equal
    cmp al, 0
    je .equal
    inc rbx
    inc rcx
    jmp .cmp_loop
.equal:
    mov eax, 1
    pop rbx
    ret
.not_equal:
    xor eax, eax
    pop rbx
    ret

; rdi: const char* a
; rsi: const char* b
; returns rax = newly allocated null-terminated string
ub_str_concat:
    push rbx
    push r12
    push r13
    push r14
    mov r12, rdi
    mov r13, rsi

    mov rdi, r12
    call ub_strlen
    mov rbx, rax

    mov rdi, r13
    call ub_strlen
    mov r14, rax

    mov rdx, rbx
    add rdx, r14
    add rdx, 1

    ; mmap(NULL, len, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANON, -1, 0)
    xor rdi, rdi
    mov rsi, rdx
    mov rdx, 3
    mov r10, 34
    mov r8, -1
    xor r9, r9
    mov rax, 9
    syscall

    mov r8, rax
    mov rdi, rax
    mov rsi, r12
    mov rcx, rbx
    rep movsb

    mov rsi, r13
    mov rcx, r14
    rep movsb

    mov byte [rdi], 0
    mov rax, r8

    pop r14
    pop r13
    pop r12
    pop rbx
    ret

; rdi: const char* s, returns rax = length
ub_strlen:
    xor rax, rax
.len_loop:
    cmp byte [rdi + rax], 0
    je .len_done
    inc rax
    jmp .len_loop
.len_done:
    ret
