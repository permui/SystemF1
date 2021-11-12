use std::error::Error;
use std::io::stdin;
use f1::base::*;
use f1::rules;

struct Derivation {
    a: Vec<Judgement>
}

impl Derivation {
    fn new() -> Self {
        Derivation { a: Vec::new() }
    }
    fn push(&mut self, j: Judgement) {
        self.a.push(j);
    }
    fn print(&self) {
        for (i, j) in self.a.iter().enumerate() {
            println!("{}:\t {}", i, j);
        }
    }
    fn get(&self, i: usize) -> Result<&Judgement, String> {
        self.a.get(i).ok_or("judgement number not exists".to_string())
    }
}

fn command(js: &mut Derivation, com: &str) -> Result<Option<Judgement>, Box<dyn Error>> {
    let sv: Vec<&str> = com.split_whitespace().collect();
    let j = match *sv.get(0).ok_or("argument not enough")? {
        "env_empty" => {
            rules::env_empty()
        },
        "env_x" => {
            let i = sv.get(1).ok_or("argument not enough")?.parse::<usize>()?;
            let v = Variable(sv.get(2).ok_or("argument not enough")?.to_string());
            let j = rules::env_x(js.get(i)?, v);
            j
        },
        "type_const" => {
            let i = sv.get(1).ok_or("argument not enough")?.parse::<usize>()?;
            let k = BasicType::get(sv.get(2).ok_or("argument not enough")?).ok_or(format!("`{}` is not a basic type", sv.get(2).ok_or("argument not enough")?))?;
            let j = rules::type_const(js.get(i)?, Type::Basic(k));
            j
        },
        "type_arrow" => {
            let i1 = sv.get(1).ok_or("argument not enough")?.parse::<usize>()?;
            let i2 = sv.get(2).ok_or("argument not enough")?.parse::<usize>()?;
            let j = rules::type_arrow(js.get(i1)?, js.get(i2)?);
            j
        },
        "val_x" => {
            let i = sv.get(1).ok_or("argument not enough")?.parse::<usize>()?;
            let x = Variable(sv.get(2).ok_or("argument not enough")?.to_string());
            let j = rules::val_x(js.get(i)?, x);
            j
        },
        "val_func" => {
            let i = sv.get(1).ok_or("argument not enough")?.parse::<usize>()?;
            let j = rules::val_func(js.get(i)?);
            j
        },
        "val_appl" => {
            let i1 = sv.get(1).ok_or("argument not enough")?.parse::<usize>()?;
            let i2 = sv.get(2).ok_or("argument not enough")?.parse::<usize>()?;
            let j = rules::val_appl(js.get(i1)?, js.get(i2)?);
            j
        },
        "exit" => {
            return Ok(None);
        },
        _ => {
            Err("invalid rule".to_string())?
        }
    };
    Ok(Some(j?))
}

fn main() {
    let mut js = Derivation::new();
    loop {
        let mut buf = String::new();
        let res = stdin().read_line(&mut buf);
        if let Ok(0) = res {
            break;
        }
        let j = command(&mut js, &buf);
        match j {
            Ok(Some(jug)) => js.push(jug),
            Ok(None) => break,
            Err(s) => {
                eprintln!("Error: {}", s);
            }
        };
        js.print();
    }
}