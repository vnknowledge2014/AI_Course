# Chapter 28 — Parser Combinators

> **Bạn sẽ học được**:
> - Parser = function: `str → ParseResult` (value + remaining)
> - Combinators: `then`, `or_else`, `many`, `map_parser`, `sep_by`, `between`
> - Building complex parsers from tiny atomic pieces
> - Functor + Monad on parsers — parsers CHÍNH LÀ Monads!
> - Practical: parse integers, CSV, JSON values
>
> **Yêu cầu trước**: Chapter 27 (Monads — bind, map), Chapter 27b (Applicative).
> **Thời gian đọc**: ~40 phút | **Level**: Advanced
> **Kết quả cuối cùng**: Hiểu "composition over configuration" — tiny parsers ghép thành complex parsers.

---

Bạn viết regex. Regex hoạt động — đến khi nó KHÔNG hoạt động. `r"(\d{1,3}\.){3}\d{1,3}"` — có đúng không? Ai biết. Regex không composable: bạn không thể lấy "regex cho số" và "regex cho dấu chấm" rồi ghép logic thành "regex cho IP address" một cách TYPE-SAFE.

**Parser combinators** giải quyết vấn đề này. Mỗi parser là một FUNCTION. Functions compose. `char("a")` + `char("b")` = `then(char("a"), char("b"))` = parser cho "ab". Từ `char` → `digit` → `integer` → `json_value` — từ đơn giản đến phức tạp, mỗi bước composable và testable.

Và bí mật: parsers CHÍNH LÀ Monads! `then` = `bind`. `map_parser` = `map`. Do-notation works on parsers. Tất cả kiến thức Ch26-27 áp dụng trực tiếp.

---

## Parser Combinators — Compose your way to complex parsing

Small pieces, loosely joined. Each parser: `str → ParseResult`. Combine: `then`, `or_else`, `many`, `map_parser`. Result: complex parsers from atoms.


## 28.1 — Parser Type

### Parser = function from string to result

```python
# filename: parser_type.py
from dataclasses import dataclass
from typing import Callable, Union, TypeVar

T = TypeVar("T")

@dataclass(frozen=True)
class ParseOk:
    """Successful parse: value + remaining unconsumed input."""
    value: object
    remaining: str

@dataclass(frozen=True)
class ParseErr:
    """Failed parse: error message + position."""
    message: str
    position: int

ParseResult = Union[ParseOk, ParseErr]
Parser = Callable[[str], ParseResult]

# ── Atomic parsers ──

def char(expected: str) -> Parser:
    """Parse exactly one specific character."""
    def parse(input: str) -> ParseResult:
        if input and input[0] == expected:
            return ParseOk(input[0], input[1:])
        actual = repr(input[0]) if input else "EOF"
        return ParseErr(f"Expected '{expected}', got {actual}", 0)
    return parse

def digit() -> Parser:
    """Parse a single digit (0-9)."""
    def parse(input: str) -> ParseResult:
        if input and input[0].isdigit():
            return ParseOk(int(input[0]), input[1:])
        actual = repr(input[0]) if input else "EOF"
        return ParseErr(f"Expected digit, got {actual}", 0)
    return parse

def letter() -> Parser:
    """Parse a single letter (a-z, A-Z)."""
    def parse(input: str) -> ParseResult:
        if input and input[0].isalpha():
            return ParseOk(input[0], input[1:])
        actual = repr(input[0]) if input else "EOF"
        return ParseErr(f"Expected letter, got {actual}", 0)
    return parse

def satisfy(predicate: Callable[[str], bool], name: str = "char") -> Parser:
    """Parse a char satisfying predicate."""
    def parse(input: str) -> ParseResult:
        if input and predicate(input[0]):
            return ParseOk(input[0], input[1:])
        actual = repr(input[0]) if input else "EOF"
        return ParseErr(f"Expected {name}, got {actual}", 0)
    return parse

# ── Tests ──
assert char("a")("abc") == ParseOk("a", "bc")
assert isinstance(char("a")("xyz"), ParseErr)
assert char("a")("") == ParseErr("Expected 'a', got EOF", 0)

assert digit()("42x") == ParseOk(4, "2x")
assert isinstance(digit()("abc"), ParseErr)

assert letter()("hello") == ParseOk("h", "ello")
assert isinstance(letter()("123"), ParseErr)

assert satisfy(str.isupper, "uppercase")("Hello") == ParseOk("H", "ello")

print("Parser type OK ✅")
```

---

## 28.2 — Combinators: Compose Parsers

### `then` — sequence: p1 rồi p2

```python
# filename: combinators.py
from dataclasses import dataclass
from typing import Callable, Union

@dataclass(frozen=True)
class ParseOk:
    value: object
    remaining: str
@dataclass(frozen=True)
class ParseErr:
    message: str
    position: int
ParseResult = Union[ParseOk, ParseErr]
Parser = Callable[[str], ParseResult]

def char(expected: str) -> Parser:
    def parse(input: str) -> ParseResult:
        if input and input[0] == expected:
            return ParseOk(input[0], input[1:])
        return ParseErr(f"Expected '{expected}'", 0)
    return parse

def digit() -> Parser:
    def parse(input: str) -> ParseResult:
        if input and input[0].isdigit():
            return ParseOk(int(input[0]), input[1:])
        return ParseErr("Expected digit", 0)
    return parse

# ═══ COMBINATORS ═══

def then(p1: Parser, p2: Parser) -> Parser:
    """Sequence: p1 then p2. Returns tuple of both results."""
    def parse(input: str) -> ParseResult:
        match p1(input):
            case ParseOk(v1, rest1):
                match p2(rest1):
                    case ParseOk(v2, rest2):
                        return ParseOk((v1, v2), rest2)
                    case err:
                        return err
            case err:
                return err
    return parse

def or_else(p1: Parser, p2: Parser) -> Parser:
    """Alternative: try p1, if fails try p2."""
    def parse(input: str) -> ParseResult:
        result = p1(input)
        if isinstance(result, ParseOk):
            return result
        return p2(input)  # backtrack: try p2 on ORIGINAL input
    return parse
```

#### Tiếp tục phân tích...

```python
def many(p: Parser) -> Parser:
    """Zero or more: p*. Always succeeds."""
    def parse(input: str) -> ParseResult:
        values = []
        remaining = input
        while True:
            result = p(remaining)
            if isinstance(result, ParseErr):
                break
            values.append(result.value)
            remaining = result.remaining
        return ParseOk(values, remaining)
    return parse

def many1(p: Parser) -> Parser:
    """One or more: p+. Fails if zero matches."""
    def parse(input: str) -> ParseResult:
        result = many(p)(input)
        match result:
            case ParseOk(value=values) if values:
                return result
            case _:
                return ParseErr("Expected at least one match", 0)
    return parse

def map_parser(p: Parser, fn: Callable) -> Parser:
    """Functor map on parser! Transform the parsed value."""
    def parse(input: str) -> ParseResult:
        match p(input):
            case ParseOk(value=v, remaining=r):
                return ParseOk(fn(v), r)
            case err:
                return err
    return parse

def optional(p: Parser, default=None) -> Parser:
    """Try parser, return default if fails."""
    def parse(input: str) -> ParseResult:
        result = p(input)
        if isinstance(result, ParseOk):
            return result
        return ParseOk(default, input)  # succeed with default, don't consume
    return parse

```

#### Tests

```python

# then: sequence
ab = then(char("a"), char("b"))
assert ab("abc") == ParseOk(("a", "b"), "c")
assert isinstance(ab("axc"), ParseErr)

# or_else: alternative
a_or_b = or_else(char("a"), char("b"))
assert a_or_b("abc") == ParseOk("a", "bc")
assert a_or_b("bcd") == ParseOk("b", "cd")
assert isinstance(a_or_b("xyz"), ParseErr)

# many: zero or more
digits = many(digit())
assert digits("123abc") == ParseOk([1, 2, 3], "abc")
assert digits("abc") == ParseOk([], "abc")  # zero matches = OK!

# many1: one or more
digits1 = many1(digit())
assert digits1("123abc") == ParseOk([1, 2, 3], "abc")
assert isinstance(digits1("abc"), ParseErr)

# map_parser: transform result
upper_char = map_parser(char("a"), str.upper)
assert upper_char("abc") == ParseOk("A", "bc")

# optional: try or default
maybe_a = optional(char("a"), "x")
assert maybe_a("abc") == ParseOk("a", "bc")
assert maybe_a("bc") == ParseOk("x", "bc")  # default, no consumption

print("Combinators OK ✅")
```

---

## ✅ Checkpoint 28.1-28.2

> Đến đây bạn phải hiểu:
> 1. **Parser** = `str → ParseResult`. Atomic: `char`, `digit`, `letter`
> 2. **`then`** = sequence. Parse A then B, return tuple
> 3. **`or_else`** = alternative. Try A, if fails try B (backtrack)
> 4. **`many`/`many1`** = repetition. Zero-or-more / one-or-more
> 5. **`map_parser`** = Functor on parsers! Transform parsed value
>
> **Test nhanh**: `many(char("a"))("aaab")` = ?
> <details><summary>Đáp án</summary>`ParseOk(["a", "a", "a"], "b")`. Three "a"s consumed, "b" remains.</details>

---

## 28.3 — Building Complex Parsers

### Integer parser

Để xây dựng parser phức tạp, ta sẽ chia nhỏ thành các bước. Mỗi bước chỉ là ghép các parser nhỏ lại với nhau.

#### Bước 1: Base Types & Atomic Parsers

```python
# filename: src/parser/complex_step1.py
from dataclasses import dataclass
from typing import Callable, Union

@dataclass(frozen=True)
class ParseOk:
    value: object
    remaining: str
@dataclass(frozen=True)
class ParseErr:
    message: str
    position: int
ParseResult = Union[ParseOk, ParseErr]
Parser = Callable[[str], ParseResult]

def char(expected): return lambda input: ParseOk(input[0], input[1:]) if input and input[0] == expected else ParseErr(f"Expected '{expected}'", 0)
def digit(): return lambda input: ParseOk(int(input[0]), input[1:]) if input and input[0].isdigit() else ParseErr("Expected digit", 0)
```

#### Bước 2: Combinators & String Utilities

```python
# filename: src/parser/complex_step2.py
# (Giả sử base types & atomics đã được import)
def then(p1, p2):
    def parse(input):
        match p1(input):
            case ParseOk(v1, r1):
                match p2(r1):
                    case ParseOk(v2, r2): return ParseOk((v1, v2), r2)
                    case err: return err
            case err: return err
    return parse

def or_else(p1, p2):
    return lambda input: p1(input) if isinstance(p1(input), ParseOk) else p2(input)

def many(p):
    def parse(input):
        values, remaining = [], input
        while True:
            result = p(remaining)
            if isinstance(result, ParseErr): break
            values.append(result.value)
            remaining = result.remaining
        return ParseOk(values, remaining)
    return parse

def many1(p):
    def parse(input):
        result = many(p)(input)
        return result if isinstance(result, ParseOk) and result.value else ParseErr("Expected 1+", 0)
    return parse

def map_parser(p, fn):
    def parse(input):
        match p(input):
            case ParseOk(v, r): return ParseOk(fn(v), r)
            case err: return err
    return parse

def optional(p, default=None):
    return lambda input: p(input) if isinstance(p(input), ParseOk) else ParseOk(default, input)

# ── String parser ──
def string(expected: str) -> Parser:
    """Parse exact string."""
    def parse(input: str) -> ParseResult:
        if input.startswith(expected):
            return ParseOk(expected, input[len(expected):])
        return ParseErr(f"Expected '{expected}'", 0)
    return parse

# ── Whitespace ──
def whitespace() -> Parser:
    return many(or_else(char(" "), or_else(char("\t"), or_else(char("\n"), char("\r")))))
```

#### Bước 3: Structure Combinators & Integer/CSV

```python
# filename: src/parser/complex_step3.py
# ── sep_by: parser separated by delimiter ──
def sep_by(p: Parser, sep: Parser) -> Parser:
    """Parse p separated by sep. Returns list. Zero matches = []."""
    def parse(input: str) -> ParseResult:
        first = p(input)
        if isinstance(first, ParseErr):
            return ParseOk([], input)

        values = [first.value]
        remaining = first.remaining

        while True:
            sep_result = sep(remaining)
            if isinstance(sep_result, ParseErr):
                break
            next_result = p(sep_result.remaining)
            if isinstance(next_result, ParseErr):
                break
            values.append(next_result.value)
            remaining = next_result.remaining

        return ParseOk(values, remaining)
    return parse

# ── between: parser wrapped in delimiters ──
def between(open_p: Parser, p: Parser, close_p: Parser) -> Parser:
    """Parse p between open and close delimiters."""
    def parse(input: str) -> ParseResult:
        match open_p(input):
            case ParseOk(_, r1):
                match p(r1):
                    case ParseOk(v, r2):
                        match close_p(r2):
                            case ParseOk(_, r3): return ParseOk(v, r3)
                            case err: return err
                    case err: return err
            case err: return err
    return parse

# ═══ INTEGER PARSER ═══
def integer() -> Parser:
    """Parse integer: optional '-' + digits."""
    def parse(input: str) -> ParseResult:
        sign_result = optional(char("-"), None)(input)
        rest = sign_result.remaining
        has_minus = sign_result.value == "-"

        digits_result = many1(digit())(rest)
        match digits_result:
            case ParseOk(value=ds, remaining=r):
                number = int("".join(str(d) for d in ds))
                return ParseOk(-number if has_minus else number, r)
            case err:
                return err
    return parse

assert integer()("42abc") == ParseOk(42, "abc")
assert integer()("-7xyz") == ParseOk(-7, "xyz")

# ═══ CSV PARSER ═══
def csv_line() -> Parser:
    """Parse comma-separated integers."""
    return sep_by(integer(), char(","))

assert csv_line()("1,2,3,4,5") == ParseOk([1, 2, 3, 4, 5], "")
```

#### Bước 4: Bracketed List & String Literal

```python
# filename: src/parser/complex_step4.py
# ═══ BRACKETED LIST ═══
def bracketed_list() -> Parser:
    """Parse [1,2,3] — list of integers in brackets."""
    return between(char("["), sep_by(integer(), char(",")), char("]"))

assert bracketed_list()("[1,2,3]") == ParseOk([1, 2, 3], "")
assert bracketed_list()("[42]") == ParseOk([42], "")
assert bracketed_list()("[]") == ParseOk([], "")

# ═══ STRING LITERAL PARSER ═══
def string_literal() -> Parser:
    """Parse "hello" — quoted string."""
    def non_quote(input: str) -> ParseResult:
        if input and input[0] != '"':
            return ParseOk(input[0], input[1:])
        return ParseErr("Expected non-quote", 0)

    return map_parser(
        between(char('"'), many(non_quote), char('"')),
        lambda chars: "".join(chars),
    )

assert string_literal()('"hello"rest') == ParseOk("hello", "rest")
assert string_literal()('"" rest') == ParseOk("", " rest")

print("Complex parsers OK ✅")
```

> **💡 Composition power**: `integer()` = `optional(char("-"))` + `many1(digit())` + `map`. `csv_line()` = `sep_by(integer(), char(","))`. `bracketed_list()` = `between(char("["), csv_line(), char("]"))`. Small pieces → complex behavior. No regex needed.

---

## 28.4 — Parser is a Monad!

### bind for parsers = dependent parsing

```python
# filename: parser_monad.py
from dataclasses import dataclass
from typing import Callable, Union

@dataclass(frozen=True)
class ParseOk:
    value: object
    remaining: str
@dataclass(frozen=True)
class ParseErr:
    message: str
    position: int
ParseResult = Union[ParseOk, ParseErr]
Parser = Callable[[str], ParseResult]

def char(expected):
    return lambda input: ParseOk(input[0], input[1:]) if input and input[0] == expected else ParseErr(f"Expected '{expected}'", 0)

def digit():
    return lambda input: ParseOk(int(input[0]), input[1:]) if input and input[0].isdigit() else ParseErr("Expected digit", 0)

def many(p):
    def parse(input):
        values, remaining = [], input
        while True:
            result = p(remaining)
            if isinstance(result, ParseErr): break
            values.append(result.value)
            remaining = result.remaining
        return ParseOk(values, remaining)
    return parse

def map_parser(p, fn):
    def parse(input):
        match p(input):
            case ParseOk(v, r): return ParseOk(fn(v), r)
            case err: return err
    return parse

```

#### Parser Monad operations

```python

def pure(value) -> Parser:
    """unit/return: succeed without consuming input."""
    return lambda input: ParseOk(value, input)

def bind_parser(p: Parser, fn: Callable) -> Parser:
    """Monad bind for parsers. fn receives parsed value, returns NEW parser."""
    def parse(input: str) -> ParseResult:
        match p(input):
            case ParseOk(value=v, remaining=r):
                next_parser = fn(v)  # fn returns a Parser!
                return next_parser(r)
            case err:
                return err
    return parse

```

#### Example: parse "3:abc" → repeat count + content

```python
# Parse "N:chars" where N digits are followed by exactly N characters

def counted_string() -> Parser:
    """Parse format: N:content where N = number of chars to read."""
    count_parser = map_parser(many(digit()), lambda ds: int("".join(str(d) for d in ds)) if ds else 0)

    def after_colon(n: int) -> Parser:
        """Read exactly n characters."""
        def parse(input: str) -> ParseResult:
            if len(input) >= n:
                return ParseOk(input[:n], input[n:])
            return ParseErr(f"Expected {n} chars, got {len(input)}", 0)
        return parse

    return bind_parser(
        count_parser,
        lambda n: bind_parser(
            char(":"),
            lambda _: after_colon(n)
        )
    )

assert counted_string()("3:abcrest") == ParseOk("abc", "rest")
assert counted_string()("5:helloworld") == ParseOk("hello", "world")
assert counted_string()("0:rest") == ParseOk("", "rest")

```

#### This is a MONAD!

```python
# bind_parser = flatMap on parsers
# map_parser = map/fmap on parsers (Functor)
# pure = unit/return
# Parser IS a Monad. All Ch27 concepts apply.

# Verify Monad laws:
# Left identity: bind(pure(a), f) == f(a)
f = lambda x: pure(x * 2)
a = 5
result1 = bind_parser(pure(a), f)("test")
result2 = f(a)("test")
assert result1 == result2  # Left Identity ✅

# Right identity: bind(m, pure) == m
m = digit()
result1 = bind_parser(m, pure)("5abc")
result2 = m("5abc")
assert result1 == result2  # Right Identity ✅

print("Parser Monad OK ✅")
```

> **💡 Parser is a Monad**: `map_parser` = Functor. `bind_parser` = Monad bind. `pure` = unit. This means EVERYTHING from Ch26-27 applies to parsers. Do-notation? Works for parsers too. The power of abstraction.

---

## ✅ Checkpoint 28.3-28.4

> Đến đây bạn phải hiểu:
> 1. **Complex parsers** from simple: `integer`, `csv_line`, `bracketed_list`
> 2. **`sep_by`**: parse items separated by delimiter
> 3. **`between`**: parse content between delimiters
> 4. **Parser is a Monad**: `bind_parser` = dependent parsing (counted strings)
> 5. **Monad laws hold** for parsers
>
> **Test nhanh**: `sep_by(integer(), char(","))("1,2,3")` = ?
> <details><summary>Đáp án</summary>`ParseOk([1, 2, 3], "")`. Three integers separated by commas, all consumed.</details>

---

## 28.5 — So sánh với Parser Libraries

### Python parser ecosystem

| Approach | Pros | Cons |
|---|---|---|
| **Hand-written combinators** (this chapter) | Full understanding, no deps | Verbose, no error recovery |
| **`lark`** | EBNF grammar, fast, popular | Grammar file separate from code |
| **`parsy`** | Combinator-based, Pythonic | Similar to our code but polished |
| **`pyparsing`** | Mature, OOP style | Verbose, heavyweight |
| **Regex** | Built-in, fast | Not composable, cryptic |

```python
# Our hand-written vs parsy (conceptual comparison):

# ── Our code ──
# integer = map_parser(many1(digit()), lambda ds: int("".join(str(d) for d in ds)))
# csv = sep_by(integer, char(","))

# ── parsy equivalent ──
# from parsy import regex, string
# integer = regex(r'-?[0-9]+').map(int)
# csv = integer.sep_by(string(','))

# Same CONCEPT — parsy just has nicer syntax + better error messages.
# Understanding our version = understanding ANY combinator library.
```

---

## 🏋️ Bài tập

**Bài 1** (5 phút): Parse a key=value pair

```python
# Parse "name=Minh" → ("name", "Minh")
# Key = letters, Value = letters+digits
```

<details><summary>✅ Lời giải Bài 1</summary>

```python
from dataclasses import dataclass
from typing import Union, Callable

@dataclass(frozen=True)
class ParseOk:
    value: object
    remaining: str
@dataclass(frozen=True)
class ParseErr:
    message: str
    position: int

def char(e): return lambda i: ParseOk(i[0], i[1:]) if i and i[0] == e else ParseErr(f"Expected '{e}'", 0)
def satisfy(pred, name=""): return lambda i: ParseOk(i[0], i[1:]) if i and pred(i[0]) else ParseErr(f"Expected {name}", 0)
def many1(p):
    def parse(i):
        values, r = [], i
        while True:
            result = p(r)
            if isinstance(result, ParseErr): break
            values.append(result.value)
            r = result.remaining
        return ParseOk(values, r) if values else ParseErr("Expected 1+", 0)
    return parse
def map_parser(p, fn):
    def parse(i):
        match p(i):
            case ParseOk(v, r): return ParseOk(fn(v), r)
            case err: return err
    return parse

key = map_parser(many1(satisfy(str.isalpha, "letter")), lambda cs: "".join(cs))
value = map_parser(many1(satisfy(str.isalnum, "alnum")), lambda cs: "".join(cs))

def key_value():
    def parse(input):
        match key(input):
            case ParseOk(k, r1):
                match char("=")(r1):
                    case ParseOk(_, r2):
                        match value(r2):
                            case ParseOk(v, r3): return ParseOk((k, v), r3)
                            case err: return err
                    case err: return err
            case err: return err
    return parse

assert key_value()("name=Minh rest") == ParseOk(("name", "Minh"), " rest")
assert key_value()("port=8080") == ParseOk(("port", "8080"), "")
```

</details>

---

**Bài 2** (10 phút): Parse arithmetic expression

```python
# Parse "3+5" → ("add", 3, 5)
# Parse "10*2" → ("mul", 10, 2)
# Parse "7-1" → ("sub", 7, 1)
# Only single operation (no precedence needed)
```

<details><summary>✅ Lời giải Bài 2</summary>

```python
from dataclasses import dataclass
from typing import Union

@dataclass(frozen=True)
class ParseOk:
    value: object
    remaining: str
@dataclass(frozen=True)
class ParseErr:
    message: str
    position: int

def digit(): return lambda i: ParseOk(int(i[0]), i[1:]) if i and i[0].isdigit() else ParseErr("digit", 0)
def char(e): return lambda i: ParseOk(i[0], i[1:]) if i and i[0] == e else ParseErr(f"'{e}'", 0)
def or_else(p1, p2): return lambda i: p1(i) if isinstance(p1(i), ParseOk) else p2(i)
def many1(p):
    def parse(i):
        vals, r = [], i
        while True:
            result = p(r)
            if isinstance(result, ParseErr): break
            vals.append(result.value)
            r = result.remaining
        return ParseOk(vals, r) if vals else ParseErr("1+", 0)
    return parse
def map_parser(p, fn):
    def parse(i):
        match p(i):
            case ParseOk(v, r): return ParseOk(fn(v), r)
            case err: return err
    return parse

integer = map_parser(many1(digit()), lambda ds: int("".join(str(d) for d in ds)))
op = or_else(char("+"), or_else(char("-"), char("*")))
op_name = {"+" : "add", "-": "sub", "*": "mul"}

def expr():
    def parse(input):
        match integer(input):
            case ParseOk(left, r1):
                match op(r1):
                    case ParseOk(operator, r2):
                        match integer(r2):
                            case ParseOk(right, r3):
                                return ParseOk((op_name[operator], left, right), r3)
                            case err: return err
                    case err: return err
            case err: return err
    return parse

assert expr()("3+5") == ParseOk(("add", 3, 5), "")
assert expr()("10*2") == ParseOk(("mul", 10, 2), "")
assert expr()("7-1") == ParseOk(("sub", 7, 1), "")
```

</details>

---

**Bài 3** (15 phút): Parse a JSON-like value

```python
# Parse: 42, "hello", [1,2,3], true, false, null
# Return Python equivalents
```

<details><summary>✅ Lời giải Bài 3</summary>

```python
from dataclasses import dataclass
from typing import Union, Callable

@dataclass(frozen=True)
class ParseOk:
    value: object
    remaining: str
@dataclass(frozen=True)
class ParseErr:
    message: str
    position: int

def char(e): return lambda i: ParseOk(i[0], i[1:]) if i and i[0] == e else ParseErr(f"'{e}'", 0)
def string(s): return lambda i: ParseOk(s, i[len(s):]) if i.startswith(s) else ParseErr(f"'{s}'", 0)
def digit(): return lambda i: ParseOk(int(i[0]), i[1:]) if i and i[0].isdigit() else ParseErr("digit", 0)
def or_else(p1, p2): return lambda i: p1(i) if isinstance(p1(i), ParseOk) else p2(i)
def many(p):
    def parse(i):
        vals, r = [], i
        while True:
            result = p(r)
            if isinstance(result, ParseErr): break
            vals.append(result.value)
            r = result.remaining
        return ParseOk(vals, r)
    return parse
def many1(p):
    def parse(i):
        r = many(p)(i)
        return r if r.value else ParseErr("1+", 0)
    return parse
def map_parser(p, fn):
    return lambda i: (lambda r: ParseOk(fn(r.value), r.remaining) if isinstance(r, ParseOk) else r)(p(i))

# Primitives
json_null = map_parser(string("null"), lambda _: None)
json_true = map_parser(string("true"), lambda _: True)
json_false = map_parser(string("false"), lambda _: False)
json_int = map_parser(many1(digit()), lambda ds: int("".join(str(d) for d in ds)))

def json_string():
    def non_quote(i):
        if i and i[0] != '"': return ParseOk(i[0], i[1:])
        return ParseErr("non-quote", 0)
    def parse(i):
        match char('"')(i):
            case ParseOk(_, r1):
                match many(non_quote)(r1):
                    case ParseOk(chars, r2):
                        match char('"')(r2):
                            case ParseOk(_, r3): return ParseOk("".join(chars), r3)
                            case err: return err
            case err: return err
    return parse

json_value = or_else(json_null,
             or_else(json_true,
             or_else(json_false,
             or_else(json_int,
             json_string()))))

assert json_value("42 rest") == ParseOk(42, " rest")
assert json_value('"hello" r') == ParseOk("hello", " r")
assert json_value("true x") == ParseOk(True, " x")
assert json_value("false x") == ParseOk(False, " x")
assert json_value("null x") == ParseOk(None, " x")
```

</details>

---

## 🔧 Troubleshooting

| Vấn đề | Nguyên nhân | Cách sửa |
|---|---|---|
| Infinite loop in `many` | Parser succeeds without consuming | Ensure parser consumes ≥1 char on success |
| Wrong backtracking | `or_else` doesn't reset position | `or_else` should use ORIGINAL input for p2 |
| Nested tuples from `then` | Multiple `then` chains | Use `map_parser` to flatten: `map_parser(then(a,b), lambda t: ...)` |
| Poor error messages | Using generic errors | Add context: `f"Expected {name} at position {pos}"` |
| Left recursion | Grammar like `expr = expr + term` | Rewrite as `expr = term ('+' term)*` using `many` |

---

## Tóm tắt

- ✅ **Parser** = `str → ParseResult`. Simple building block.
- ✅ **Combinators**: `then`, `or_else`, `many`, `many1`, `sep_by`, `between`, `optional`.
- ✅ **`map_parser`** = Functor on parsers. Transform parsed value.
- ✅ **`bind_parser`** = Monad bind. Dependent parsing (counted strings).
- ✅ **Complex from simple**: `char` → `digit` → `integer` → `csv` → `json_value`.
- ✅ **Parser = Monad**: All Ch26-27 concepts apply directly.
- ✅ **Composition over configuration**: no regex, no grammar files, just functions.

## Tiếp theo

→ Chapter 28b: **Traverse & Sequence** — flip containers. `list[Optional[A]]` → `Optional[list[A]]`. `asyncio.gather` chính là `sequence`!
