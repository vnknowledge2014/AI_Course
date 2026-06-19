# Chapter 27b — Applicative & Validation

> **Bạn sẽ học được**:
> - **Applicative** = "apply function in container to value in container"
> - **Fail-fast vs Collect-all**: Monad chỉ bắt lỗi ĐẦU TIÊN, Applicative thu thập TẤT CẢ
> - **Validation pattern**: validate nhiều fields, gom tất cả lỗi
> - **ap** = Applicative apply: `ap(fn_in_box, value_in_box) → result_in_box`
> - Practical: form validation, API request validation, config parsing
>
> **Yêu cầu trước**: Chapter 27 (Monads — bind, map).
> **Thời gian đọc**: ~35 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Hiểu tại sao form validation cần Applicative, không phải Monad.

---

Bạn đăng ký tài khoản online. Nhập sai username, sai email, sai password — submit. Website báo lỗi: "Username phải ≥ 3 ký tự". Bạn sửa username, submit lại. "Email không hợp lệ". Sửa email, submit. "Password phải ≥ 8 ký tự". BA LẦN submit cho BA lỗi? Tệ!

Website tốt báo TẤT CẢ lỗi cùng lúc: "Username too short, Email invalid, Password too weak". Sửa hết → submit MỘT lần. Đó là Validation — và Monad KHÔNG LÀM ĐƯỢC điều này. Monad short-circuit ở lỗi đầu tiên (Ch27). Applicative là pattern cho phép "chạy tất cả, gom tất cả lỗi".

---

## Applicative — Apply in parallel, collect all errors

Monad `bind` = SEQUENTIAL: step 2 phụ thuộc kết quả step 1. Nếu step 1 fail → stop. Applicative = INDEPENDENT: validate name, validate email, validate password — BA phép kiểm tra KHÔNG phụ thuộc nhau, nên có thể chạy TẤT CẢ rồi gom kết quả.


## 27b.1 — Monad's limitation: Fail-fast

### Short-circuit chỉ bắt lỗi đầu tiên

```python
# filename: fail_fast_problem.py
from dataclasses import dataclass
from typing import Union, Callable

@dataclass(frozen=True)
class Ok:
    value: object
@dataclass(frozen=True)
class Err:
    error: str
Result = Union[Ok, Err]

def bind(r: Result, fn: Callable) -> Result:
    match r:
        case Ok(value=v): return fn(v)
        case Err(): return r

# ── Form validation (Monad — fail-fast) ──
def validate_name(name: str) -> Result:
    return Ok(name) if len(name) >= 2 else Err("Name too short")

def validate_email(email: str) -> Result:
    return Ok(email) if "@" in email else Err("Invalid email")

def validate_age(age_str: str) -> Result:
    try:
        age = int(age_str)
        return Ok(age) if 0 < age < 150 else Err(f"Age {age} out of range")
    except: return Err(f"'{age_str}' is not a number")

# Monadic chain — short-circuits at FIRST error
def validate_form_monad(name: str, email: str, age: str) -> Result:
    r1 = validate_name(name)
    r2 = bind(r1, lambda n: validate_email(email))
    r3 = bind(r2, lambda e: validate_age(age))
    return r3

# ALL three fields are invalid, but only FIRST error shown
result = validate_form_monad("M", "bad-email", "abc")
assert result == Err("Name too short")  # Only this! Others not checked ❌

# User fixes name, submits again → sees email error
result = validate_form_monad("Minh", "bad-email", "abc")
assert result == Err("Invalid email")  # Still one at a time ❌

print("Fail-fast problem shown ✅")
```

---

## 27b.2 — Validated: Collect ALL Errors

### Applicative Validation type

Giải pháp: tạo type `Validated` riêng biệt với `Result`. `Result` = Monad (fail-fast). `Validated` = Applicative (collect-all). Cùng "shape" nhưng KHÁC semantics.

```python
# filename: validated_type.py
from dataclasses import dataclass
from typing import Union, Callable, TypeVar

T = TypeVar("T")

@dataclass(frozen=True)
class Valid:
    value: object

@dataclass(frozen=True)
class Invalid:
    errors: tuple[str, ...]  # tuple = immutable, multi-error

Validated = Union[Valid, Invalid]

def valid(value) -> Validated:
    return Valid(value)

def invalid(*errors: str) -> Validated:
    return Invalid(errors)

# ── Combine errors: Semigroup on errors! ──
def combine_errors(v1: Validated, v2: Validated, fn: Callable) -> Validated:
    """
    Apply fn to values if BOTH are Valid.
    If either (or both) are Invalid, COMBINE all errors.
    """
    match (v1, v2):
        case (Valid(value=a), Valid(value=b)):
            return Valid(fn(a, b))
        case (Invalid(errors=e1), Invalid(errors=e2)):
            return Invalid(e1 + e2)  # combine ALL errors!
        case (Invalid(errors=e), _):
            return Invalid(e)
        case (_, Invalid(errors=e)):
            return Invalid(e)

# ── Tests ──
assert combine_errors(Valid(1), Valid(2), lambda a, b: a + b) == Valid(3)
assert combine_errors(
    Invalid(("error 1",)),
    Invalid(("error 2",)),
    lambda a, b: a + b,
) == Invalid(("error 1", "error 2"))  # BOTH errors collected!

assert combine_errors(
    Invalid(("error 1",)),
    Valid(42),
    lambda a, b: a + b,
) == Invalid(("error 1",))

print("Validated type OK ✅")
```

> **💡 Key difference**: Result (Monad) `bind` = sequential, fail-fast. Validated (Applicative) `combine_errors` = parallel, collect-all. Use Result when steps DEPEND on each other. Use Validated when validations are INDEPENDENT.

---

## 27b.3 — Applicative Validation in Practice

### Form validation — collect ALL errors at once

```python
# filename: form_validation.py
from dataclasses import dataclass
from typing import Union, Callable

@dataclass(frozen=True)
class Valid:
    value: object
@dataclass(frozen=True)
class Invalid:
    errors: tuple[str, ...]
Validated = Union[Valid, Invalid]

# ── Individual field validators ──
def validate_name(name: str) -> Validated:
    errors = []
    if len(name.strip()) < 2:
        errors.append("Name must be at least 2 characters")
    if len(name) > 50:
        errors.append("Name must be at most 50 characters")
    if any(c.isdigit() for c in name):
        errors.append("Name cannot contain digits")
    return Invalid(tuple(errors)) if errors else Valid(name.strip())

def validate_email(email: str) -> Validated:
    errors = []
    if "@" not in email:
        errors.append("Email must contain @")
    elif email.count("@") > 1:
        errors.append("Email must contain exactly one @")
    elif "." not in email.split("@")[1]:
        errors.append("Email domain must contain a dot")
    return Invalid(tuple(errors)) if errors else Valid(email.lower())

def validate_password(password: str) -> Validated:
    errors = []
    if len(password) < 8:
        errors.append("Password must be at least 8 characters")
    if not any(c.isupper() for c in password):
        errors.append("Password must contain an uppercase letter")
    if not any(c.isdigit() for c in password):
        errors.append("Password must contain a digit")
    return Invalid(tuple(errors)) if errors else Valid(password)

# ── Combine independent validations ──
def validate_all(*validations: Validated) -> Validated:
    """Run ALL validations, collect ALL errors."""
    errors: list[str] = []
    values: list = []

    for v in validations:
        match v:
            case Valid(value=val):
                values.append(val)
            case Invalid(errors=errs):
                errors.extend(errs)

    return Invalid(tuple(errors)) if errors else Valid(tuple(values))

# ── Form validator ──
@dataclass(frozen=True)
class UserRegistration:
    name: str
    email: str
    password: str

def validate_registration(name: str, email: str, password: str) -> Validated:
    result = validate_all(
        validate_name(name),
        validate_email(email),
        validate_password(password),
    )
    match result:
        case Valid(value=vals):
            return Valid(UserRegistration(name=vals[0], email=vals[1], password=vals[2]))
        case Invalid():
            return result

# ── Tests ──

# All valid → success
result = validate_registration("Minh", "minh@co.com", "SecureP@ss1")
assert result == Valid(UserRegistration("Minh", "minh@co.com", "securep@ss1"))

# ALL fields invalid → ALL errors collected!
result = validate_registration("M", "bad", "weak")
assert isinstance(result, Invalid)
# Should have errors from ALL three fields:
assert "Name must be at least 2 characters" in result.errors
assert "Email must contain @" in result.errors
assert "Password must be at least 8 characters" in result.errors
assert "Password must contain an uppercase letter" in result.errors
assert "Password must contain a digit" in result.errors
assert len(result.errors) == 5  # All 5 errors collected!

# Some valid, some invalid
result = validate_registration("Minh", "bad", "SecureP@ss1")
assert isinstance(result, Invalid)
assert len(result.errors) == 1
assert "Email must contain @" in result.errors

# Password multiple errors
result = validate_registration("Minh", "minh@co.com", "short")
assert isinstance(result, Invalid)
assert len(result.errors) == 3  # too short + no upper + no digit

print("Form validation OK ✅")
```

---

## ✅ Checkpoint 27b.1-27b.3

> Đến đây bạn phải hiểu:
> 1. **Monad = fail-fast**: `bind` short-circuits ở lỗi đầu tiên
> 2. **Applicative = collect-all**: chạy tất cả validations, gom tất cả errors
> 3. **`Validated` type**: `Valid(value)` or `Invalid(errors: tuple)`
> 4. **`validate_all`**: combine independent validations
> 5. **When**: validations INDEPENDENT → Applicative. Steps DEPENDENT → Monad
>
> **Test nhanh**: `validate_all(Invalid(("e1",)), Valid(42), Invalid(("e2", "e3")))` = ?
> <details><summary>Đáp án</summary>`Invalid(("e1", "e2", "e3"))`! All errors from ALL Invalid results are collected. Valid(42) contributes no errors but its value is lost.</details>

---

## 27b.4 — ap: The Applicative Operation

### Apply function in container to value in container

`ap` là tên chính thức của Applicative operation. Ý tưởng: bạn có một function TRONG container và một giá trị TRONG container. `ap` áp dụng function lên giá trị, giữ context.

```python
# filename: applicative_ap.py
from dataclasses import dataclass
from typing import Union, Callable

@dataclass(frozen=True)
class Valid:
    value: object
@dataclass(frozen=True)
class Invalid:
    errors: tuple[str, ...]
Validated = Union[Valid, Invalid]

# ap :: Validated[A → B] → Validated[A] → Validated[B]
def ap(fn_v: Validated, val_v: Validated) -> Validated:
    """Apply function-in-container to value-in-container."""
    match (fn_v, val_v):
        case (Valid(value=fn), Valid(value=val)):
            return Valid(fn(val))
        case (Invalid(errors=e1), Invalid(errors=e2)):
            return Invalid(e1 + e2)  # collect ALL errors
        case (Invalid(errors=e), _):
            return Invalid(e)
        case (_, Invalid(errors=e)):
            return Invalid(e)

# ── Build multi-argument constructor via curried ap ──
# User(name, email, age) — need 3 args

@dataclass(frozen=True)
class User:
    name: str
    email: str
    age: int

# Curry: User constructor → one arg at a time
def make_user(name: str):
    def step2(email: str):
        def step3(age: int):
            return User(name, email, age)
        return step3
    return step2

# Chain ap:
# 1. Start: Valid(make_user)                      — function in box
# 2. ap(Valid(make_user), Valid("Minh"))           — partially applied
# 3. ap(result, Valid("minh@co.com"))              — partially applied
# 4. ap(result, Valid(25))                         — fully applied = User!

result = ap(
    ap(
        ap(Valid(make_user), Valid("Minh")),
        Valid("minh@co.com"),
    ),
    Valid(25),
)
assert result == Valid(User("Minh", "minh@co.com", 25))

# With errors — ALL collected!
result = ap(
    ap(
        ap(Valid(make_user), Invalid(("name too short",))),
        Invalid(("invalid email",)),
    ),
    Invalid(("age out of range",)),
)
assert result == Invalid(("name too short", "invalid email", "age out of range"))

print("Applicative ap OK ✅")
```

### lift_n — Helper cho multi-argument functions

Currying `ap` thủ công khá verbose. `lift` functions simplify:

```python
# filename: applicative_lift.py
from dataclasses import dataclass
from typing import Union, Callable

@dataclass(frozen=True)
class Valid:
    value: object
@dataclass(frozen=True)
class Invalid:
    errors: tuple[str, ...]
Validated = Union[Valid, Invalid]

def ap(fn_v: Validated, val_v: Validated) -> Validated:
    match (fn_v, val_v):
        case (Valid(value=fn), Valid(value=val)): return Valid(fn(val))
        case (Invalid(errors=e1), Invalid(errors=e2)): return Invalid(e1 + e2)
        case (Invalid(errors=e), _): return Invalid(e)
        case (_, Invalid(errors=e)): return Invalid(e)

def lift2(fn: Callable, v1: Validated, v2: Validated) -> Validated:
    """Lift a 2-arg function into Validated context."""
    return ap(ap(Valid(lambda a: lambda b: fn(a, b)), v1), v2)

def lift3(fn: Callable, v1: Validated, v2: Validated, v3: Validated) -> Validated:
    """Lift a 3-arg function into Validated context."""
    return ap(ap(ap(Valid(lambda a: lambda b: lambda c: fn(a, b, c)), v1), v2), v3)

# ── Usage ──
@dataclass(frozen=True)
class Config:
    host: str
    port: int
    debug: bool

result = lift3(
    lambda h, p, d: Config(h, p, d),
    Valid("localhost"),
    Valid(8080),
    Valid(True),
)
assert result == Valid(Config("localhost", 8080, True))

# Errors collected
result = lift3(
    lambda h, p, d: Config(h, p, d),
    Invalid(("host required",)),
    Valid(8080),
    Invalid(("debug must be bool",)),
)
assert result == Invalid(("host required", "debug must be bool"))

print("Applicative lift OK ✅")
```

---

## 27b.5 — Monad vs Applicative: When to Use Which?

### Decision framework

```
Is validation for field B DEPENDENT on result of field A?

YES → Monad (bind)
  Example: "validate email domain" depends on parsed email format
  Chain: parse_email(raw) → bind → validate_domain(parsed)

NO → Applicative (validate_all / ap)
  Example: validate name, email, password independently
  Parallel: validate_all(check_name, check_email, check_password)
```

```python
# filename: monad_vs_applicative.py
from dataclasses import dataclass
from typing import Union, Callable

@dataclass(frozen=True)
class Ok:
    value: object
@dataclass(frozen=True)
class Err:
    error: str
Result = Union[Ok, Err]

@dataclass(frozen=True)
class Valid:
    value: object
@dataclass(frozen=True)
class Invalid:
    errors: tuple[str, ...]
Validated = Union[Valid, Invalid]

def bind(r, fn):
    match r:
        case Ok(value=v): return fn(v)
        case Err(): return r

def validate_all(*vs):
    errors, values = [], []
    for v in vs:
        match v:
            case Valid(value=val): values.append(val)
            case Invalid(errors=errs): errors.extend(errs)
    return Invalid(tuple(errors)) if errors else Valid(tuple(values))

# ═══ Monad: DEPENDENT steps ═══
# Step 2 NEEDS result of step 1

def parse_config_file(path: str) -> Result:
    """Step 1: read file (might fail)."""
    configs = {"app.toml": {"port": "8080", "host": "localhost"}}
    return Ok(configs[path]) if path in configs else Err(f"File not found: {path}")

def parse_port(config: dict) -> Result:
    """Step 2: depends on step 1's result."""
    try: return Ok(int(config["port"]))
    except: return Err("Invalid port")

# Must be sequential — step 2 needs step 1's output
result = bind(parse_config_file("app.toml"), parse_port)
assert result == Ok(8080)

# ═══ Applicative: INDEPENDENT fields ═══
# Each field validates on its own

def check_name(n: str) -> Validated:
    return Valid(n) if len(n) >= 2 else Invalid(("name too short",))

def check_email(e: str) -> Validated:
    return Valid(e) if "@" in e else Invalid(("bad email",))

def check_age(a: str) -> Validated:
    try:
        age = int(a)
        return Valid(age) if 0 < age < 150 else Invalid(("age out of range",))
    except: return Invalid(("age not a number",))

# Can validate ALL at once — independent!
result = validate_all(check_name("M"), check_email("bad"), check_age("abc"))
assert isinstance(result, Invalid)
assert len(result.errors) == 3  # All 3 errors!

print("Monad vs Applicative OK ✅")
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Classify each as Monad or Applicative

```python
# a) Validate username length AND username uniqueness (uniqueness check needs the parsed username)
# b) Validate height, weight, BMI threshold — all independent
# c) Parse JSON string → extract "id" field → fetch from database with ID
# d) Check name format, check email format, check phone format — independent
```

<details><summary>✅ Lời giải Bài 1</summary>

```python
# a) MONAD — uniqueness depends on parsed username (step 2 needs step 1)
# b) APPLICATIVE — height, weight, BMI are independent checks
# c) MONAD — each step depends on previous (parse → extract → fetch)
# d) APPLICATIVE — name, email, phone are independent validations
```

</details>

---

**Bài 2** (10 phút): Validate an `Address` with ALL errors

```python
# Validate: street (non-empty), city (non-empty), zip_code (5 digits)
# Collect ALL errors at once
```

<details><summary>✅ Lời giải Bài 2</summary>

```python
from dataclasses import dataclass
from typing import Union

@dataclass(frozen=True)
class Valid:
    value: object
@dataclass(frozen=True)
class Invalid:
    errors: tuple[str, ...]
Validated = Union[Valid, Invalid]

def validate_all(*vs):
    errors, values = [], []
    for v in vs:
        match v:
            case Valid(value=val): values.append(val)
            case Invalid(errors=errs): errors.extend(errs)
    return Invalid(tuple(errors)) if errors else Valid(tuple(values))

@dataclass(frozen=True)
class Address:
    street: str
    city: str
    zip_code: str

def check_street(s: str) -> Validated:
    return Valid(s.strip()) if s.strip() else Invalid(("Street is required",))

def check_city(c: str) -> Validated:
    return Valid(c.strip()) if c.strip() else Invalid(("City is required",))

def check_zip(z: str) -> Validated:
    clean = z.strip()
    errors = []
    if len(clean) != 5:
        errors.append("Zip code must be 5 digits")
    if not clean.isdigit():
        errors.append("Zip code must contain only digits")
    return Invalid(tuple(errors)) if errors else Valid(clean)

def validate_address(street: str, city: str, zip_code: str) -> Validated:
    result = validate_all(check_street(street), check_city(city), check_zip(zip_code))
    match result:
        case Valid(value=vals):
            return Valid(Address(street=vals[0], city=vals[1], zip_code=vals[2]))
        case Invalid():
            return result

# Tests
good = validate_address("123 Main St", "Hanoi", "10000")
assert good == Valid(Address("123 Main St", "Hanoi", "10000"))

bad = validate_address("", "", "abc")
assert isinstance(bad, Invalid)
assert len(bad.errors) == 4  # street + city + 2 zip errors
```

</details>

---

**Bài 3** (15 phút): Combine Monad AND Applicative

```python
# 1. Parse JSON config (Monad — might fail)
# 2. Extract 3 fields: host, port, debug (Applicative — independent validations)
# 3. Build Config if ALL valid
```

<details><summary>✅ Lời giải Bài 3</summary>

```python
import json
from dataclasses import dataclass
from typing import Union, Callable

@dataclass(frozen=True)
class Ok:
    value: object
@dataclass(frozen=True)
class Err:
    error: str
Result = Union[Ok, Err]

@dataclass(frozen=True)
class Valid:
    value: object
@dataclass(frozen=True)
class Invalid:
    errors: tuple[str, ...]
Validated = Union[Valid, Invalid]

def bind(r, fn):
    match r:
        case Ok(value=v): return fn(v)
        case Err(): return r

def validate_all(*vs):
    errors, values = [], []
    for v in vs:
        match v:
            case Valid(value=val): values.append(val)
            case Invalid(errors=errs): errors.extend(errs)
    return Invalid(tuple(errors)) if errors else Valid(tuple(values))

@dataclass(frozen=True)
class Config:
    host: str
    port: int
    debug: bool

# Step 1: Parse JSON (Monad — dependent)
def parse_json(raw: str) -> Result:
    try: return Ok(json.loads(raw))
    except: return Err("Invalid JSON")

# Step 2: Validate fields (Applicative — independent)
def validate_fields(data: dict) -> Result:
    def check_host(d):
        h = d.get("host", "")
        return Valid(h) if h else Invalid(("host is required",))

    def check_port(d):
        p = d.get("port")
        if p is None: return Invalid(("port is required",))
        if not isinstance(p, int): return Invalid(("port must be integer",))
        if not (1 <= p <= 65535): return Invalid(("port out of range",))
        return Valid(p)

    def check_debug(d):
        db = d.get("debug")
        if db is None: return Invalid(("debug is required",))
        if not isinstance(db, bool): return Invalid(("debug must be boolean",))
        return Valid(db)

    result = validate_all(check_host(data), check_port(data), check_debug(data))
    match result:
        case Valid(value=vals):
            return Ok(Config(host=vals[0], port=vals[1], debug=vals[2]))
        case Invalid(errors=errs):
            return Err("Validation failed: " + "; ".join(errs))

# Combined: Monad → Applicative
def load_config(raw: str) -> Result:
    return bind(parse_json(raw), validate_fields)

# Tests
assert load_config('{"host":"localhost","port":8080,"debug":true}') == Ok(Config("localhost", 8080, True))
assert load_config("bad json") == Err("Invalid JSON")
assert "host is required" in load_config('{"port":8080,"debug":true}').error
assert "port is required" in load_config('{"host":"x"}').error
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Only first error shown | Used Monad (bind) instead of Applicative | Use `validate_all` for independent checks |
| Losing valid values | `validate_all` returns tuple, not domain object | Pattern match `Valid(values)` → construct domain object |
| Errors duplicated | Multiple validators checking same thing | Each validator should check ONE concern |
| Can't use Applicative | Step B depends on step A result | Use Monad (bind) for dependent steps |
| TypeError in `ap` | Function not curried | Curry: `lambda a: lambda b: fn(a, b)` |

---

## Tóm tắt

- ✅ **Monad** = fail-fast (bind). First error stops pipeline
- ✅ **Applicative** = collect-all (ap/validate_all). ALL errors gathered
- ✅ **`Validated` type**: `Valid(value)` or `Invalid(errors: tuple)`
- ✅ **`validate_all`**: combine independent validations
- ✅ **`ap`**: apply function-in-container to value-in-container
- ✅ **`lift2`/`lift3`**: lift multi-arg functions into Validated context
- ✅ **When**: dependent steps → Monad. Independent validations → Applicative
- ✅ **Combine**: Monad for parsing, Applicative for field validation

## Tiếp theo

→ Chapter 28: **Parser Combinators** — build complex parsers from tiny pieces. `char`, `digit` → `integer` → `JSON parser`. Functor + Monad on parsers.
