# Security

Please report security issues privately by opening a GitHub security advisory on the repository.

`mediautil` is local-first and should not upload input files or derived content. Treat regressions that unexpectedly send file data over the network as security bugs.

When adding archive extraction, shell execution, or file-writing behavior:

- validate paths before writing extracted files
- avoid shell string interpolation
- preserve explicit missing-tool errors
- add functional tests for unsafe inputs
