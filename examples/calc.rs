extern crate grammer;
use grammer::Grammer;
use grammer::NoShared;
use std::env;

fn main()
{
    let mut g: Grammer<f64> = Grammer::new();

    g.declare(vec!["add", "base", "div", "mul", "neg", "pow", "sub"]);
    
    g.add("digit", "[0-9]", None);
    g.add("num", "(\\.<digit>+|<digit>+(\\.<digit>+)?)", Some(Box::new(|_, l, _| vec![l.parse::<f64>().unwrap()])));
    
    g.add("expr", " <add> ", None);
    g.add("add", "<sub> (\\+ <sub>)*", Some(Box::new(|b, _, _| if b.len() == 1 { b } else { vec![b.iter().fold(0f64, |acc, &n| acc + n)] })));
    g.add("sub", "<mul> (- <mul>)*", Some(Box::new(|b, _, _| if b.len() == 1 { b } else { vec![b.iter().fold(b[0] * 2f64, |acc, &n| acc - n)] })));
    g.add("mul", "<div> (\\* <div>)*", Some(Box::new(|b, _, _| if b.len() == 1 { b } else { vec![b.iter().fold(1f64, |acc, &n| acc * n)] })));
    g.add("div", "<pow> (/ <pow>)*", Some(Box::new(|b, _, _| if b.len() == 1 { b } else { vec![b.iter().fold(b[0] * b[0], |acc, &n| acc / n)] })));
    g.add("pow", "<neg> (\\^ <neg>)?", Some(Box::new(|b, _, _| if b.len() == 1 { b } else { vec![b[0].powf(b[1])] })));
    
    g.add("neg-apply", "-<base>", Some(Box::new(|b, _, _| vec![-b[0]])));
    g.add("neg", "(<neg-apply>|<base>)", None);

    g.add("abs", "abs\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].abs()])));
    g.add("acos", "acos\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].acos()])));
    g.add("acosh", "acosh\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].acosh()])));
    g.add("asin", "asin\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].asin()])));
    g.add("asinh", "asinh\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].asinh()])));
    g.add("atan", "atan\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].atan()])));
    g.add("atanh", "atanh\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].atanh()])));
    g.add("ceil", "ceil\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].ceil()])));
    g.add("cos", "cos\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].cos()])));
    g.add("cosh", "cosh\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].cosh()])));
    g.add("exp", "exp\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].exp()])));
    g.add("floor", "floor\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].floor()])));
    g.add("fract", "fract\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].fract()])));
    g.add("ln", "ln\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].ln()])));
    g.add("log", "log\\(<expr>,<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].log(b[1])])));
    g.add("recip", "recip\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].recip()])));
    g.add("round", "round\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].round()])));
    g.add("signum", "signum\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].signum()])));
    g.add("sin", "sin\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].sin()])));
    g.add("sinh", "sinh\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].sinh()])));
    g.add("sqrt", "sqrt\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].sqrt()])));
    g.add("tan", "tan\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].tan()])));
    g.add("tanh", "tanh\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].tanh()])));
    g.add("trunc", "trunc\\(<expr>\\)", Some(Box::new(|b, _, _| vec![b[0].trunc()])));
    g.add("func", "(<abs>|<acos>|<acosh>|<asin>|<asinh>|<atan>|<atanh>|<ceil>|<cos>|<cosh>|<exp>|<floor>|<fract>|<ln>|<log>|<recip>|<round>|<signum>|<sin>|<sinh>|<sqrt>|<tan>|<tanh>|<trunc>)", None);
    
    g.add("base", "(\\(<expr>\\)|<num>|<func>)", None);
    
    // Combine the command line arguments into one String.
    let collected: Vec<String> = env::args().collect();
    let expr = &collected[1..].join("");

    match g.scan("expr", expr) {
        Ok(branches) => println!("{}", branches[0]),
        Err(_) => println!("Not a valid expression."),
    }
}