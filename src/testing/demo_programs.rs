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

pub const UNIVERSAL_QUESTION: &'static str = r"fn main() -> int {
    if 0 == 0 {
        return 42;
    }
}";

pub const UNIVERSE_EQ: &'static str = r"fn main() -> bool {
    return 42 == 42;
}";
pub const UNIVERSE_EQ_NEG: &'static str = r"fn main() -> bool {
    return 42 == 4;
}";

pub const UNIVERSE_G: &'static str = r"fn main() -> bool {
    return 42 > 0;
}";
pub const TRUE: &'static str = "fn main() -> bool { return true; }";
pub const FALSE: &'static str = "fn main() -> bool { return false; }";