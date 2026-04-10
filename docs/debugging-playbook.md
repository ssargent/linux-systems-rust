# Debugging Playbook

1. Reproduce with the smallest input.
2. Add tracing and stderr diagnostics.
3. Use `strace`, `gdb`, or `lldb` for syscall/process-level inspection.
4. Write a regression test after fixing the issue.
