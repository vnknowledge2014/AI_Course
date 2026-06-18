# Chapter 28 — Parser Combinators

> **Bạn sẽ học được**:
> - Parser = function: `str → Result[(value, remaining)]`
> - Combinators: `then`, `or_else`, `many`, `map`
> - Building complex parsers from simple ones
>
> **Yêu cầu trước**: Chapter 27 (Monads)
> **Thời gian đọc**: ~30 phút | **Level**: Advanced

---

## 28.1 — Parser Type

```python
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

# Basic parsers
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

assert char("a")("abc") == ParseOk("a", "bc")
assert isinstance(char("a")("xyz"), ParseErr)
assert digit()("42x") == ParseOk(4, "2x")
```

## 28.2 — Combinators

```python
def then(p1: Parser, p2: Parser) -> Parser:
    """Sequence: p1 then p2."""
    def parse(input: str) -> ParseResult:
        match p1(input):
            case ParseOk(v1, rest):
                match p2(rest):
                    case ParseOk(v2, rest2):
                        return ParseOk((v1, v2), rest2)
                    case err:
                        return err
            case err:
                return err
    return parse

def or_else(p1: Parser, p2: Parser) -> Parser:
    """Alternative: p1 or p2."""
    def parse(input: str) -> ParseResult:
        result = p1(input)
        if isinstance(result, ParseOk):
            return result
        return p2(input)
    return parse

def many(p: Parser) -> Parser:
    """Zero or more: p*."""
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

# Combine!
ab = then(char("a"), char("b"))
assert ab("abc") == ParseOk(("a", "b"), "c")

a_or_b = or_else(char("a"), char("b"))
assert a_or_b("abc") == ParseOk("a", "bc")
assert a_or_b("bcd") == ParseOk("b", "cd")

digits = many(digit())
assert digits("123abc") == ParseOk([1, 2, 3], "abc")

# Parse integer!
def integer() -> Parser:
    def parse(input: str) -> ParseResult:
        result = many(digit())(input)
        match result:
            case ParseOk(digits, rest) if digits:
                number = int("".join(str(d) for d in digits))
                return ParseOk(number, rest)
            case _:
                return ParseErr("Expected integer", 0)
    return parse

assert integer()("42abc") == ParseOk(42, "abc")
```

---

## Tóm tắt

- ✅ **Parser**: `str → ParseResult` — function from string to result.
- ✅ **Combinators**: `then`, `or_else`, `many` — compose parsers.
- ✅ **Build up**: `char` → `digit` → `integer` → complex parsers.
- ✅ **FP power**: Small composable pieces → complex behavior.

## Tiếp theo

→ Chapter 29: **TDD with pytest** — Test-Driven Development.
