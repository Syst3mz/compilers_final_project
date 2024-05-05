pub const THE_UNIVERSE: &'static str = "fn main() -> int { return 42; }";
pub const THE_UNIVERSE_BY_ADDITION: &'static str = r"fn main() -> int {
    return 20 + 22;
}";
pub const ASSIGNED_UNIVERSE: &'static str = r"fn main() -> int {
    let a: int = 20;
    let b: int = 22;
    return a + b;
}";
pub const ASSIGNED_UNIVERSE_MUTATION: &'static str = r"fn main() -> int {
    let a: int = 20;
    a = a + 22;
    return a;
}";

pub const UNIVERSAL_NEGATION: &'static str = r"fn main() -> int {
    return 62 + -20;
}";

pub const IF_TRUE: &'static str = r"fn main() -> int {
    if 0 == 0 {
        return 42;
    }
}";

pub const UNIVERSE_EQ: &'static str = r"fn main() -> bool {
    return 42 == 42;
}";
pub const UNIVERSE_EQ_FALSE: &'static str = r"fn main() -> bool {
    return 42 == 4;
}";

pub const AND: &'static str = r"fn main() -> bool {
    return (1 > 0) && (1 > 0);
}";
pub const AND_A_FALSE: &'static str = r"fn main() -> bool {
    return (0 > 1) && (1 > 0);
}";

pub const AND_B_FALSE: &'static str = r"fn main() -> bool {
    return (1 > 0) && (0 > 1);
}";

pub const AND_FALSE: &'static str = r"fn main() -> bool {
    return (0 > 1) && (0 > 1);
}";

pub const OR: &'static str = r"fn main() -> bool {
    return (1 > 0) || (1 > 0);
}";
pub const OR_A_FALSE: &'static str = r"fn main() -> bool {
    return (0 > 1) || (1 > 0);
}";

pub const OR_B_FALSE: &'static str = r"fn main() -> bool {
    return (1 > 0) || (0 > 1);
}";

pub const OR_FALSE: &'static str = r"fn main() -> bool {
    return (0 > 1) || (0 > 1);
}";

pub const UNIVERSE_G: &'static str = r"fn main() -> bool {
    return 42 > 0;
}";

pub const UNIVERSE_G_NEG: &'static str = r"fn main() -> bool {
    return 0 > 42;
}";

pub const UNIVERSE_FUNC_CALL_NO_ARGS: &'static str = r"fn universe() -> int {
    return 42;
}

fn main() -> int {
    return universe();
}";

pub const UNIVERSE_FUNC_CALL_ARG: &'static str = r"fn universe(a:int) -> int {
    return a;
}

fn main() -> int {
    return universe(42);
}";

pub const UNIVERSE_FUNC_CALL_ARGS: &'static str = r"fn universe(a:int, b:int) -> int {
    return a + b;
}

fn main() -> int {
    return universe(20, 22);
}";

pub const UNIVERSE_INT_TO_BOOL: &'static str = r"fn main() -> bool {
    return 42;
}";
pub const TRUE: &'static str = "fn main() -> bool { return true; }";
pub const FALSE: &'static str = "fn main() -> bool { return false; }";