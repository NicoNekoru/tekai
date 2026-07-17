# Security policy

Please report suspected vulnerabilities with GitHub private vulnerability
reporting for `NicoNekoru/tekai`. Do not include exploit details in a public
issue.

Include the affected version, operating system, TeX distribution, triggering
command, and a minimal document or repository when it can be shared safely.

TeX is a programmable language. `tekai` does not enable shell escape by default;
passing `--shell-escape` allows trusted documents and packages to execute
external commands with the current user's permissions.
