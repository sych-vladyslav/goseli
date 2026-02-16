---
name: security-audit
description: Run a security audit on the codebase - OWASP Top 10, auth vulnerabilities, injection attacks, rate limiting, CORS, CSP. Used by Security Engineer.
context: fork
agent: Explore
argument-hint: [area e.g. "auth" or "api" or "full"]
---

# Security Audit

Comprehensive security review against OWASP Top 10 and e-commerce best practices.

Area: $ARGUMENTS

## OWASP Top 10 Checks

### 1. Injection (A03:2021)
- [ ] SQL queries use parameterized statements (sqlx handles this)
- [ ] No string concatenation in queries
- [ ] User input sanitized before use in shell commands
- [ ] JSONB queries properly escape user input

### 2. Broken Authentication (A07:2021)
- [ ] Passwords hashed with bcrypt/argon2 (not MD5/SHA)
- [ ] JWT tokens have reasonable expiry (access: 15min, refresh: 7 days)
- [ ] Refresh token rotation implemented
- [ ] Failed login rate limiting
- [ ] Password complexity requirements enforced
- [ ] No credentials in code, logs, or error messages

### 3. Sensitive Data Exposure (A02:2021)
- [ ] HTTPS enforced (HSTS header)
- [ ] Sensitive headers removed (X-Powered-By)
- [ ] PCI compliance: credit card data never stored (use Stripe tokens)
- [ ] Personal data encrypted at rest where required
- [ ] API responses don't leak internal data (stack traces, DB errors)

### 4. Broken Access Control (A01:2021)
- [ ] Authorization checked on every endpoint (not just authentication)
- [ ] Users cannot access other users' data (IDOR prevention)
- [ ] Admin endpoints restricted to admin role
- [ ] API rate limiting per-user and per-IP

### 5. Security Misconfiguration (A05:2021)
- [ ] CORS properly configured (not `*` in production)
- [ ] Content-Security-Policy header set
- [ ] X-Frame-Options: DENY
- [ ] X-Content-Type-Options: nosniff
- [ ] No default credentials or debug endpoints

### 6. XSS (A03:2021)
- [ ] React's built-in escaping used (no `dangerouslySetInnerHTML`)
- [ ] User input sanitized in any server-rendered HTML
- [ ] CSP prevents inline scripts

### 7. CSRF
- [ ] State-changing requests use POST/PUT/DELETE (not GET)
- [ ] CSRF tokens on forms (or SameSite cookies)
- [ ] API endpoints check Origin/Referer headers

### E-Commerce Specific
- [ ] Price validation on server side (don't trust client prices)
- [ ] Stock checked at checkout time (not just display time)
- [ ] Order total recalculated server-side
- [ ] Coupon codes cannot be brute-forced (rate limiting)
- [ ] File upload validation (type, size, content)
- [ ] No PII in URLs or query parameters

## Output
For each vulnerability:
- **Severity**: Critical / High / Medium / Low / Informational
- **OWASP Category**: A01-A10
- **Location**: file:line
- **Description**: What's vulnerable
- **Exploit scenario**: How it could be exploited
- **Fix**: Specific code change or configuration
- **Verification**: How to verify the fix works
