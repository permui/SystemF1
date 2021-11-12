use crate::base::*;
use crate::result::*;
use Assertion::*;


pub fn env_empty() -> Res<Judgement> {
    let empty_env = Environment { env: Vec::new() };
    Ok(Judgement {
        environment: empty_env,
        assertion: WFEnv
    })
}

pub fn env_x(j: &Judgement, x: Variable) -> Res<Judgement> {
    let Judgement { environment, assertion } = j;
    if let WFType(t) = assertion {
        if environment.defined(&x) {
            Err("variable already defined".to_string())
        } else {
            let mut e = environment.clone();
            e.env.push((x, t.clone()));
            Ok(Judgement {
                environment: e,
                assertion: WFEnv
            })
        }
    } else {
        Err("judgement given is not about type".to_string())
    }
}

pub fn type_const(j: &Judgement, k: Type) -> Res<Judgement> {
    if let WFEnv = j.assertion {
        if let Type::Basic(_) = &k {
            Ok(Judgement {
                environment: j.environment.clone(),
                assertion: WFType(k)
            })
        } else {
            Err("type given is not a Basic type".to_string())
        }
    } else {
        Err("judgement given is not about environment".to_string())
    }
}

pub fn type_arrow(fir: &Judgement, sec: &Judgement) -> Res<Judgement> {
    if fir.environment != sec.environment {
        return Err("two environments differ".to_string());
    }
    if let WFType(a) = &fir.assertion {
        if let WFType(b) = &sec.assertion {
            return Ok(Judgement {
                environment: fir.environment.clone(),
                assertion: WFType(Type::Arrow(Box::new(a.clone()), Box::new(b.clone())))
            })
        }
    }
    Err("not both type assertions".to_string())
} 

pub fn val_x(j: &Judgement, x: Variable) -> Res<Judgement> {
    if let WFEnv = j.assertion {
        let r = j.environment.get(&x);
        match r {
            Some((v, t)) => {
                return Ok(Judgement {
                    environment: j.environment.clone(),
                    assertion: WFTermType(Term::Var(v.clone()), t.clone())
                })
            },
            None => return Err(format!("variable `{}` not in environment", x.0))
        }
    }
    return Err("judgement is not about environment".to_string())
}

pub fn val_func(j: &Judgement) -> Res<Judgement> {
    if let WFTermType(m, b) = &j.assertion {
        if j.environment.env.is_empty() {
            return Err("judgement environment empty".to_string());
        }
        let mut e = j.environment.clone();
        let (x, a) = e.env.pop().unwrap();
        return Ok(Judgement {
            environment: e,
            assertion: WFTermType(
                Term::Func(x, a.clone(), Box::new(m.clone())),
                Type::Arrow(Box::new(a), Box::new(b.clone()))
            )
        });
    }
    return Err("judgement not in matching form".to_string())
}

pub fn val_appl(fir: &Judgement, sec: &Judgement) -> Res<Judgement> {
    if fir.environment != sec.environment {
        return Err("two environments not match".to_string());
    }
    if let WFTermType(m, t1) = &fir.assertion {
        if let Type::Arrow(a, b) = t1 {
            if let WFTermType(n, t2) = &sec.assertion {
                if a.as_ref() == t2 {
                    return Ok(Judgement {
                        environment: fir.environment.clone(),
                        assertion: WFTermType(
                            Term::Appl(Box::new(m.clone()), Box::new(n.clone())),
                            b.as_ref().clone()
                        )
                    });
                }
            }
        }
    }
    return Err("assertions not match to rule".to_string());
}