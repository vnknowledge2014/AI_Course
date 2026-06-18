# Chapter 35 — Security

> **Bạn sẽ học được**:
> - Password hashing: bcrypt, argon2
> - JWT authentication
> - Input validation = defense in depth
> - OWASP Top 10 awareness
>
> **Yêu cầu trước**: Chapter 34 (Advanced Data)
> **Thời gian đọc**: ~30 phút | **Level**: Principal

---

## 35.1 — Password Hashing

```python
import hashlib
import secrets

# ❌ NEVER: plain text or MD5
# password = "secret123"  ← stored as-is = catastrophic

# ✅ Use bcrypt or argon2
# pip install bcrypt
def hash_password(password: str) -> str:
    salt = secrets.token_hex(16)
    hashed = hashlib.pbkdf2_hmac("sha256", password.encode(), salt.encode(), 100_000)
    return f"{salt}:{hashed.hex()}"

def verify_password(password: str, stored: str) -> bool:
    salt, hash_hex = stored.split(":")
    hashed = hashlib.pbkdf2_hmac("sha256", password.encode(), salt.encode(), 100_000)
    return hashed.hex() == hash_hex

stored = hash_password("my_secret_123")
assert verify_password("my_secret_123", stored)
assert not verify_password("wrong_password", stored)
```

## 35.2 — JWT Authentication

```python
import json, base64, hmac, hashlib, time

SECRET = "super-secret-key-change-in-production"

def create_jwt(payload: dict, expiry_seconds: int = 3600) -> str:
    header = base64.urlsafe_b64encode(json.dumps({"alg": "HS256"}).encode()).decode()
    payload["exp"] = int(time.time()) + expiry_seconds
    body = base64.urlsafe_b64encode(json.dumps(payload).encode()).decode()
    signature = hmac.new(SECRET.encode(), f"{header}.{body}".encode(), hashlib.sha256).hexdigest()
    return f"{header}.{body}.{signature}"

def verify_jwt(token: str) -> dict | None:
    parts = token.split(".")
    if len(parts) != 3:
        return None
    header, body, signature = parts
    expected = hmac.new(SECRET.encode(), f"{header}.{body}".encode(), hashlib.sha256).hexdigest()
    if signature != expected:
        return None
    payload = json.loads(base64.urlsafe_b64decode(body + "=="))
    if payload.get("exp", 0) < time.time():
        return None
    return payload

token = create_jwt({"user_id": 42, "role": "admin"})
payload = verify_jwt(token)
assert payload is not None
assert payload["user_id"] == 42
```

## 35.3 — Input Validation = Security

```python
from pydantic import BaseModel, Field, field_validator
import re

class LoginRequest(BaseModel):
    email: str
    password: str = Field(min_length=8, max_length=128)

    @field_validator("email")
    @classmethod
    def validate_email(cls, v: str) -> str:
        if not re.match(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$", v):
            raise ValueError("Invalid email format")
        return v.lower()

# Pydantic validation = first line of defense
# Combined with type system = defense in depth
```

---

## Tóm tắt

- ✅ **Hashing**: bcrypt/argon2 for passwords. NEVER plain text.
- ✅ **JWT**: Stateless auth tokens. Verify signature + expiry.
- ✅ **Validation**: Pydantic = security at input boundary.
- ✅ **Types**: Domain types prevent injection by design.

## Tiếp theo

→ Chapter 36: **Application Security** — CORS, HTTPS, rate limiting, SQL injection.
