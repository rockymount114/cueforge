# Security Policy

## Supported Versions

CueForge is currently pre-1.0 and in active foundational development.
Until a 1.0 release, only the `main` branch is supported with security
fixes.

| Version | Supported |
|---|---|
| main (pre-1.0) | ✅ |

## Reporting a Vulnerability

Please **do not** open a public issue for security vulnerabilities.

Instead, report privately via GitHub's private vulnerability reporting
feature on this repository, or via the security contact listed in the
repository's GitHub metadata once it is published. Include:

- A description of the vulnerability and its impact
- Steps to reproduce
- Affected crate(s) and version/commit

You should expect an initial response within a few business days.

## Scope

Security issues of particular interest for CueForge include:

- Networking (`crates/networking`) — desync exploits, malformed packet
  handling, cheating vectors in multiplayer sync
- Replay/save file parsing (`crates/replay`) — malicious file handling
- Plugin system (`docs/architecture/PluginSystem.md`) — sandboxing and
  capability boundaries for third-party plugins

Physics correctness bugs (a shot behaving unrealistically) are **not**
security issues — please file those as regular bugs using
`.github/ISSUE_TEMPLATE/physics.yml`.
