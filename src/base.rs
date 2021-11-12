use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable(pub String);

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BasicType(String);

impl BasicType {
    const BASIC_TYPES: [&'static str; 2] = ["t", "K"];
    pub fn get(s: &str) -> Option<BasicType> {
        if Self::BASIC_TYPES.contains(&s) {
            Some(BasicType(String::from(s)))
        } else {
            None
        }
    }
}

impl fmt::Display for BasicType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Basic(BasicType),
    Arrow(Box<Type>, Box<Type>)
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Basic(b) => write!(f, "{}", b),
            Self::Arrow(a, b) => {
                let fir = match a.as_ref() {
                    Type::Basic(t) => format!("{}", t),
                    _ => format!("({})", a.as_ref())
                };
                let sec = match b.as_ref() {
                    Type::Basic(t) => format!("{}", t),
                    _ => format!("({})", b.as_ref())
                };
                write!(f, "{} -> {}", fir, sec)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Var(Variable),
    Func(Variable, Type, Box<Term>),
    Appl(Box<Term>, Box<Term>)
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Var(v) => write!(f, "{}", v),
            Self::Func(v, t, m) => write!(f, "λ{}:{}.{}", v, t, m.as_ref()),
            Self::Appl(m, n) => write!(f, "({} {})", m.as_ref(), n.as_ref())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Assertion {
    WFEnv,
    WFType(Type),
    WFTermType(Term, Type)
}

impl fmt::Display for Assertion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WFEnv => write!(f, "◇"),
            Self::WFType(t) => write!(f, "{}", t),
            Self::WFTermType(v, t) => write!(f, "{} : {}", v, t)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Environment {
    pub env: Vec<(Variable, Type)>
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.env.is_empty() {
            write!(f, "∅")
        } else {
            let t = self.env.iter().map(|(v, t)| format!("{} : {}", v, t)).collect::<Vec<String>>();
            let s = t.join(", ");
            write!(f, "{}", s)
        }
    }
}

impl Environment {
    pub fn defined(&self, x: &Variable) -> bool {
        for (v, _) in &self.env {
            if v == x {
                return true;
            }
        }
        return false;
    }
    pub fn get(&self, x: &Variable) -> Option<&(Variable, Type)> {
        self.env.iter().find(|&p| &p.0 == x)
    }
}

#[derive(Clone, Debug)]
pub struct Judgement {
    pub environment: Environment,
    pub assertion: Assertion
}

impl fmt::Display for Judgement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} |- {}", self.environment, self.assertion)
    }
}