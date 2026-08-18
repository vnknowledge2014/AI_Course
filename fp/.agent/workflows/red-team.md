---
description: "Red Team Workflow: Offensive security pipeline using AI-powered agents. From OSINT recon → vulnerability discovery → exploitation → post-exploitation."
---

// turbo-all

# 🔴 Red Team Workflow — Native Omni Pipeline

> Offensive security via `omni run` DAGs. Sub-agents use `web_search`, `web_fetch`, `grep_search`,
> `write_file`, `skill_search` to perform recon, vuln analysis, and exploit planning.
>
> Sources: PentestGPT, Strix, Redamon, PentAGI, PentestAgent, HexStrike AI, AI-Infra-Guard.
>
> **Data zone:** All output goes to `.agent/security/red/`. Blue Team reads from here in Phase 2.

## Prerequisites

- `omni` CLI + Ollama + SurrealDB running
- Create `scope.md` with target, RoE, allowed techniques, time window

> ⚠️ **Legal**: Only run against targets with explicit written authorization.
>
> **Tip:** Use `security_auditor` role in DAGs for this workflow — it has full security toolkit
> (pattern_scan, http_request, dns_resolve, port_check, tls_inspect, web_crawl, sandbox_exec, web_search, web_fetch).
> Run `omni security scan` first for automated pattern-based baseline.

### Infrastructure Resilience

> **If SurrealDB or Ollama is down:**
> - `omni run` tasks will fail — sub-agents require Ollama to execute
> - **Red-team output files** (`.agent/security/red/`) are written to disk by sub-agents — they persist regardless of DB state
> - If only SurrealDB is down: skill search still works (BM25 in-memory), but memory recall and reasoning bank are unavailable
> - When services recover: run `omni index --import-fallback` then `omni index` to sync any fallback data
> - Re-run the failed phase after recovery — each phase reads its inputs from disk, so no data is lost

---

## Phase 1: OSINT RECON — 3 parallel analyzers + 1 consolidator

```bash
omni run "You are a red team lead. Read scope.md for the target.

TASK 1 — OSINT Reconnaissance: Use web_search + web_fetch + skill_search to gather:
1. WHOIS, DNS records (A/MX/NS/TXT/CNAME), subdomains (crt.sh, web sources)
2. Technology stack (web server, CMS, frameworks, JS libs, CDN)
3. Public exposure (breaches, GitHub leaks, Shodan/Censys, Google dorks)

Use skill_search for: 'osint-evals', 'shodan-reconnaissance', 'ethical-hacking-methodology',
'pentest-checklist', 'scanning-tools', 'red-team-tactics'

Write consolidated Attack Surface Map to .agent/security/red/attack-surface.md:
## Asset Inventory (domains, IPs, ports)
## Technology Stack (mark outdated versions HIGH PRIORITY)
## Potential Attack Vectors (CRITICAL/HIGH/MEDIUM/LOW)
## Prioritized Target List (Target | Tech | Risk | Rationale)

Rules: OSINT only — no active scanning. Document source URLs."
```

---

## Phase 2: VULNERABILITY ANALYSIS — CVE research + parallel vuln scanning

```bash
omni run "You are a vulnerability researcher. Read .agent/security/red/attack-surface.md.

TASK 2 — Vulnerability Discovery:
1. CVE Research: For each tech+version, use web_search for CVEs (NVD, Exploit-DB), then web_fetch for details.
   Record: CVE ID, CVSS score, public PoC availability, Metasploit modules, MITRE ATT&CK technique.
2. Web App Analysis: Check injection points, auth flaws, IDOR, SSRF, CORS, CSP, error handling.
3. Infrastructure Analysis: TLS/SSL, DNS security (SPF/DKIM/DMARC), security headers.
4. Cloud/Identity: OAuth endpoints, API docs exposure, S3/Azure blob, token security.

Use skill_search for: 'top-web-vulnerabilities', 'api-security-testing', 'sql-injection-testing',
'xss-html-injection', 'vulnerability-scanner', 'web-security-testing', 'active-directory-attacks'

Write consolidated report to .agent/security/red/vuln-report.md:
## Executive Summary (totals by severity)
## Findings Table (sorted by CVSS)
## Attack Chain Analysis (how findings chain together)
## Top 5 Exploitation Targets for Phase 3

Rules: Analysis only — no active exploitation."
```

---

## Phase 3: EXPLOITATION PLANNING — strategy + PoC generation + playbooks

```bash
omni run "You are a red team exploit developer. Read .agent/security/red/vuln-report.md and .agent/security/red/attack-surface.md.

TASK 3 — Exploitation Planning:
1. Attack Strategy: Map attack chains as Entry→Pivot→Escalation→Objective.
   For each vector: pre-conditions, MITRE technique, tool/method, expected result, fallback.
2. PoC Scripts: Write Python PoC scripts to .agent/security/red/exploits/ for top vulnerabilities.
   Each script: validates target is in scope.md, captures evidence, NON-DESTRUCTIVE, clean exit.
3. Attack Playbooks: Write playbooks to .agent/security/red/playbooks/ (web, infra, cloud, post-exploit).
   Each: objective, prerequisites, step-by-step commands, evidence collection, cleanup checklist.

Use skill_search for: 'red-team-tools', 'pentest-commands', 'proof-of-vulnerability',
'sqlmap-database-pentesting', 'ethical-hacking-methodology'

Write exploit-plan.md with Kill Chain mapping and success probabilities.

Rules: ALL scripts NON-DESTRUCTIVE. ALL targets verified against scope.md."
```

---

## Phase 3.5: POST-EXPLOITATION — Lateral movement, persistence, cleanup

> Inspired by [Redamon](https://github.com/samugit83/redamon): zero-human-intervention post-exploitation simulation.

```bash
omni run "You are a post-exploitation specialist. Read .agent/security/red/exploit-plan.md and .agent/security/red/vuln-report.md.

TASK 3.5 — Post-Exploitation Simulation:

1. LATERAL MOVEMENT: For each successfully exploited entry point, map potential pivot paths:
   - Internal network segments reachable from compromised host
   - Shared credentials / credential reuse opportunities
   - Trust relationships (domain trusts, SSH keys, API tokens)
   - Service-to-service communication paths

2. PERSISTENCE MECHANISMS: Document persistence techniques applicable to each target:
   - Scheduled tasks / cron jobs
   - Service manipulation / registry modification
   - Web shells / backdoor placement
   - Supply chain injection points

3. PRIVILEGE ESCALATION VERIFICATION: For each pivot:
   - Kernel exploits / SUID binaries / misconfigured sudoers
   - Group policy / GPO abuse
   - Token impersonation / credential delegation

4. DATA EXFILTRATION SIMULATION (NON-DESTRUCTIVE):
   - Identify sensitive data locations (DB, file shares, cloud storage)
   - Map egress channels (DNS tunneling, HTTP/S, ICMP)
   - Estimate data volume at risk

5. CLEANUP CHECKLIST: For each technique, document:
   - Artifacts created (files, logs, registry keys)
   - Cleanup commands
   - Detection signatures that would catch this activity

Use skill_search for: 'post-exploitation', 'active-directory-attacks', 'red-team-tactics',
'pentest-commands', 'ethical-hacking-methodology'

Write to .agent/security/red/post-exploit-report.md with:
## Lateral Movement Map (node graph: entry → pivot → target)
## Persistence Viable Techniques (sorted by stealth rating)
## Privilege Escalation Paths
## Data at Risk Assessment
## Cleanup Verification Checklist

Rules: ALL simulation is THEORETICAL. NO actual lateral movement. Document what COULD happen."
```

---

## Phase 4: FINAL REPORT

```bash
omni run "Read all .agent/security/red/ files (attack-surface.md, vuln-report.md, exploit-plan.md, post-exploit-report.md).
Generate .agent/security/red/pentest-report.md:
1. Executive Summary (2 paragraphs)
2. Scope & Methodology
3. Findings Summary Table (sorted by CVSS)
4. Detailed Findings (Description, Evidence, CVSS, MITRE, Remediation)
5. Attack Chain Analysis (including post-exploitation paths)
6. Risk Matrix & Business Impact (including data-at-risk from post-exploit)
7. Remediation Priorities (Critical→Low with timelines)
8. Appendix: Raw Data References

Also create .agent/security/red/executive-brief.md (1-page management brief)."
```

---

## Quick One-Shots

```bash
# Quick OSINT
omni run "Read scope.md. OSINT recon via web_search + web_fetch. Write to .agent/security/red/recon-quick.md"

# Quick CVE scan
omni run "Read .agent/security/red/attack-surface.md. Search CVEs for each tech. Write to .agent/security/red/cve-quick.md"

# Quick web vuln check
omni run "Read scope.md. Check web security (headers, CORS, CSP, cookies). Write to .agent/security/red/webapp-quick.md"
```

---

## Pipeline

```
Phase 1 RECON → Phase 2 VULN SCAN → Phase 3 EXPLOIT → Phase 3.5 POST-EXPLOIT → Phase 4 REPORT
 web_search         CVE research        PoC scripts      Lateral movement      pentest-report.md
 web_fetch          web/infra/cloud     playbooks        Persistence           executive-brief.md
 3 parallel scans   MITRE mapping       kill chain map   Cleanup checklist
```

### Data Flow

```
.agent/security/red/
├── attack-surface.md      ← Phase 1 output
├── vuln-report.md         ← Phase 2 output
├── exploit-plan.md        ← Phase 3 output
├── post-exploit-report.md ← Phase 3.5 output (NEW)
├── pentest-report.md      ← Phase 4 output
├── executive-brief.md     ← Phase 4 output
├── exploits/              ← Phase 3 PoC scripts
└── playbooks/             ← Phase 3 attack playbooks
         │
         ↓ (Blue Team reads pentest-report.md in Phase 2)
.agent/security/blue/adversarial-report.md
```

### Key Skills (auto-searched by sub-agents via `skill_search`)

| Skill | Phase |
|-------|-------|
| `red-team-tactics`, `red-team-tools` | All |
| `pentest-checklist`, `pentest-commands` | 1-3 |
| `osint-evals`, `shodan-reconnaissance` | 1 |
| `scanning-tools`, `vulnerability-scanner` | 2 |
| `top-web-vulnerabilities`, `api-security-testing` | 2 |
| `sql-injection-testing`, `xss-html-injection` | 2-3 |
| `sqlmap-database-pentesting` | 3 |
| `proof-of-vulnerability` | 3 |
| `active-directory-attacks` | 2-3.5 |
| `threat-modeling-expert`, `threat-mitigation-mapping` | 3-4 |
