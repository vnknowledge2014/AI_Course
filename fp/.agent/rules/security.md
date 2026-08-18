---
description: Security guidelines and restrictions on tool usage.
trigger: always_on
---
# Security & Privacy

1.  **Privacy Protection**: NEVER expose API keys, passwords, personal data, or proprietary IP in code output, logs, or external models.
    _Rationale: Sub-agent outputs persist in SurrealDB — leaked secrets become permanent._
2.  **Tool Restrictions**:
    -   NEVER use `curl` or `wget` to send POST requests containing sensitive data to unknown external endpoints.
    -   Restrict memory storage to project architectural context, not credentials.
    -   Be extremely cautious running arbitrary `sh` commands retrieved from the web.
    -   The `web_search` and `web_fetch` tools have built-in InputGuard that blocks localhost, private IPs (10.x, 192.168.x, 172.16-31.x), and file:// URLs.
3.  **Input Sanitization**: `omni run` has built-in Input Guard (41 detectors across 9 categories: instruction override, role switching, jailbreak, context manipulation, encoding bypass, CJK injection, social engineering, template injection, padding detection) that automatically blocks prompt injection attacks. For additional sanitization beyond this, ensure inputs are escaped before SQL/command use.
4.  **Skill Security**: `omni skill validate` runs a 5-category Security Scanner (command injection, secret patterns, unsafe file ops, network exfil, privilege escalation). Critical findings block deployment.
