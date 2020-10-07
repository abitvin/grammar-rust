extern crate grammar;
use grammar::Grammar;
use std::env;

fn main()
{
    let mut g: Grammar<f64> = Grammar::new();

    g.rule("digit", "[0-9]");
    g.map("num", "(\\.<digit>+|<digit>+(\\.<digit>+)?)", |_, l| l.parse::<f64>().unwrap());
    
    g.rule("expr", " <add> ");
    g.map("add", "<sub> (\\+ <sub>)*", |b, _| if b.len() == 1 { b[0] } else { b.iter().fold(0f64, |acc, &n| acc + n) });
    g.map("sub", "<mul> (- <mul>)*", |b, _| if b.len() == 1 { b[0] } else { b.iter().fold(b[0] * 2f64, |acc, &n| acc - n) });
    g.map("mul", "<div> (\\* <div>)*", |b, _| if b.len() == 1 { b[0] } else { b.iter().fold(1f64, |acc, &n| acc * n) });
    g.map("div", "<pow> (/ <pow>)*", |b, _| if b.len() == 1 { b[0] } else { b.iter().fold(b[0] * b[0], |acc, &n| acc / n) });
    g.map("pow", "<neg> (\\^ <neg>)?", |b, _| if b.len() == 1 { b[0] } else { b[0].powf(b[1]) });
    
    g.map("neg-apply", "-<base>", |b, _| -b[0]);
    g.rule("neg", "(<neg-apply>|<base>)");

    g.map("abs", "abs\\(<expr>\\)", |b, _| b[0].abs());
    g.map("acos", "acos\\(<expr>\\)", |b, _| b[0].acos());
    g.map("acosh", "acosh\\(<expr>\\)", |b, _| b[0].acosh());
    g.map("asin", "asin\\(<expr>\\)", |b, _| b[0].asin());
    g.map("asinh", "asinh\\(<expr>\\)", |b, _| b[0].asinh());
    g.map("atan", "atan\\(<expr>\\)", |b, _| b[0].atan());
    g.map("atanh", "atanh\\(<expr>\\)", |b, _| b[0].atanh());
    g.map("ceil", "ceil\\(<expr>\\)", |b, _| b[0].ceil());
    g.map("cos", "cos\\(<expr>\\)", |b, _| b[0].cos());
    g.map("cosh", "cosh\\(<expr>\\)", |b, _| b[0].cosh());
    g.map("exp", "exp\\(<expr>\\)", |b, _| b[0].exp());
    g.map("floor", "floor\\(<expr>\\)", |b, _| b[0].floor());
    g.map("fract", "fract\\(<expr>\\)", |b, _| b[0].fract());
    g.map("ln", "ln\\(<expr>\\)", |b, _| b[0].ln());
    g.map("log", "log\\(<expr>,<expr>\\)", |b, _| b[0].log(b[1]));
    g.map("recip", "recip\\(<expr>\\)", |b, _| b[0].recip());
    g.map("round", "round\\(<expr>\\)", |b, _| b[0].round());
    g.map("signum", "signum\\(<expr>\\)", |b, _| b[0].signum());
    g.map("sin", "sin\\(<expr>\\)", |b, _| b[0].sin());
    g.map("sinh", "sinh\\(<expr>\\)", |b, _| b[0].sinh());
    g.map("sqrt", "sqrt\\(<expr>\\)", |b, _| b[0].sqrt());
    g.map("tan", "tan\\(<expr>\\)", |b, _| b[0].tan());
    g.map("tanh", "tanh\\(<expr>\\)", |b, _| b[0].tanh());
    g.map("trunc", "trunc\\(<expr>\\)", |b, _| b[0].trunc());
    g.rule("func", "(<abs>|<acos>|<acosh>|<asin>|<asinh>|<atan>|<atanh>|<ceil>|<cos>|<cosh>|<exp>|<floor>|<fract>|<ln>|<log>|<recip>|<round>|<signum>|<sin>|<sinh>|<sqrt>|<tan>|<tanh>|<trunc>)");
    
    g.rule("base", "(\\(<expr>\\)|<num>|<func>)");

    let c = g.compile().unwrap();
    
    // Combine the command line arguments into one String.
    let collected: Vec<String> = env::args().collect();
    let expr = &collected[1..].join("");

    match c.scan("expr", expr) {
        Ok(branches) => println!("{}", branches[0]),
        Err(_) => println!("Not a valid expression."),
    }
}