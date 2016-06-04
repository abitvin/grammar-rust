// Copyright (c) 2015-2016 Abitvin <foss@abitvin.net>
// Licensed under the MIT license <LICENSE.md or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

mod abitvin
{
    // TODO What is the best way to store the branch closure?
    // http://stackoverflow.com/questions/27831944/how-do-i-store-a-closure-in-rust
    // TODO Simplify `Option<Box<Fn<...>>>` if we can.
    pub type BranchFn<T> = Option<Box<Fn(&Vec<T>, &str) -> Vec<T>>>;
    
    pub struct Rule<'a, T: 'a> {
        branch_fn: BranchFn<T>,
        parts: Vec<ScanFn<'a, T>>,
    }
    
    pub struct RuleError {
        pub index: i64,
        pub msg: String,
        // TODO trail: TMeta[]
    }
    
    impl Clone for RuleError 
    {
        fn clone(&self) -> Self
        {
            RuleError {
                index: self.index,
                msg: self.msg.clone(),
            }
        }
    }
    
    enum ScanFn<'a, T: 'a> {
        All,
        AllExcept(&'a Vec<char>),
        Alter(&'a Vec<(&'static str, &'static str)>),
        AnyOf(&'a Vec<&'a Rule<'a, T>>),
        CharIn(char, char),
        Eof,
        Literal(&'static str),
        Not(&'a Rule<'a, T>),
        Range(u64, u64, &'a Rule<'a, T>)
    }
    
    use std::str::Chars;
    
    struct ScanCtx<'a, T> {
        branches: Vec<T>,
        code_iter: Chars<'a>,
        errors: Vec<RuleError>,
        index: i64,    // TODO Change to usize? No, because we use an iterator now. Or yes if we don't use Chars.
        lexeme: String,
        // TODO metaPushed: number;
        // TODO trail: TMeta[];
    }
    
    impl<'a, T> Rule<'a, T>
    {
        pub fn new(branch_fn: BranchFn<T>) -> Self
        {
            Rule { 
                branch_fn: branch_fn,
                parts: Vec::new(),
            }
        }
        
        pub fn all(&mut self) -> &mut Self
        {
            self.parts.push(ScanFn::All);
            self
        }
        
        pub fn all_except(&mut self, exclude: &'a Vec<char>) -> &mut Self
        {
            if exclude.len() == 0 {
                panic!("List of excluded characters is empty.");
            }
            
            self.parts.push(ScanFn::AllExcept(&exclude));
            self
        }
        
        pub fn alter(&mut self, list: &'a Vec<(&'static str, &'static str)>) -> &mut Self
        {
            if list.len() == 0 {
                panic!("List is empty.");
            }
            
            if !list.iter().any(|&t| { t.0.len() > 0 && t.1.len() > 1 }) {
                panic!("The strings in the list must be minimal one character long.");
            }
            
            self.parts.push(ScanFn::Alter(list));
            self
        }
        
        pub fn any_of(&mut self, rules: &'a Vec<&Rule<T>>) -> &mut Self
        {
            if rules.len() == 0 {
                panic!("You must specify rules.");
            }
            
            self.parts.push(ScanFn::AnyOf(&rules));
            self
        }
        
        pub fn at_least(&mut self, count: u64, rule: &'a Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(count, u64::max_value(), &rule));
            self
        }
        
        pub fn at_most(&mut self, count: u64, rule: &'a Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(0, count, &rule));
            self
        }
        
        pub fn between(&mut self, min: u64, max: u64, rule: &'a Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(min, max, &rule));
            self
        }
        
        pub fn char_in(&mut self, min: char, max: char) -> &mut Self
        {
            self.parts.push(ScanFn::CharIn(min, max));
            self
        }
        
        pub fn clear(&mut self) -> &mut Self
        {
            self.parts.clear();
            self
        }
        
        pub fn eof(&mut self) -> &mut Self
        {
            self.parts.push(ScanFn::Eof);
            self
        }
        
        pub fn exact(&mut self, count: u64, rule: &'a Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(count, count, &rule));
            self
        }
        
        pub fn literal(&mut self, text: &'static str) -> &mut Self
        {
            if text.len() < 1 {
                panic!("Literal text must at least 1 character long.");
            }
                
            self.parts.push(ScanFn::Literal(&text));
            self
        }
        
        pub fn maybe(&mut self, rule: &'a Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(0, 1, &rule));
            self
        }
        
        pub fn none_or_many(&mut self, rule: &'a Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(0, u64::max_value(), &rule));
            self
        }
        
        pub fn not(&mut self, rule: &'a Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::Not(&rule));
            self
        }
        
        pub fn one(&mut self, rule: &'a Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(1, 1, &rule));
            self
        }
        
        // TODO I think `code` needs to be a String 
        pub fn scan(&self, code: &'static str) -> Result<Vec<T>, Vec<RuleError>>
        {
            let mut ctx: ScanCtx<T> = ScanCtx {
                branches: Vec::new(),
                code_iter: code.chars(),
                errors: Vec::new(),
                index: 0,
                lexeme: String::new(),
            };
            
            if self.run(&mut ctx) == -1 {
                return Err(ctx.errors);
            }
            
            if let Some(_) = ctx.code_iter.next() {
                /*
                TODO Do we need these checks?
                if ctx.has_eof {
                    ctx.index -= 1;
                }
                
                if (ctx.index !== ctx.code.length)
                    return RuleResult.failed<TBranch, TMeta>(ctx.errors);
                
                */
                
                Err(ctx.errors)
            }
            else {
                Ok(ctx.branches)
            }
        }
        
        // Private functions
        
        fn branch(&'a self, ctx: &ScanCtx<'a, T>, is_root_of_rule: bool) -> ScanCtx<T>
        {
            let new_ctx: ScanCtx<T> = ScanCtx {
                branches: Vec::new(),
                code_iter: ctx.code_iter.clone(),
                errors: ctx.errors.clone(),
                index: ctx.index,
                lexeme: String::new(),
                // TODO metaPushed: isRootOfRule ? 0 : ctx.metaPushed,
                // TODO trail: ctx.trail.slice(0)
            };
            
            /* TODO
            if (isRootOfRule && this._meta)
            {
                newCtx.metaPushed++;
                newCtx.trail.push(this._meta);
            }
            */
            
            new_ctx
        }
        
        fn merge(&'a self, target: &mut ScanCtx<'a, T>, source: &mut ScanCtx<'a, T>, is_root_of_rule: bool) -> i64
        {
            /* TODO
            if (isRootOfRule)
                while (source.metaPushed-- > 0)
                    source.trail.pop();
            */
            
            let step = source.index - target.index;
            
			target.code_iter = source.code_iter.clone();
            target.errors = source.errors.clone();
            target.index = source.index;
            target.lexeme.push_str(&source.lexeme.to_string());
            // TODO target.metaPushed = 0;
            // TODO target.trail = source.trail;
           
            match self.branch_fn {
                Some(ref f) if is_root_of_rule => {
                    target.branches.append(&mut f(&source.branches, &source.lexeme));
                },
                _ => {
                    target.branches.append(&mut source.branches);
                }
            }
            
            step
        }
        
        // TODO Why I need `mut ctx` here: 
        // https://www.reddit.com/r/rust/comments/2sjicp/use_of_mut_in_function_signature/
        fn run(&'a self, mut ctx: &mut ScanCtx<'a, T>) -> i64
        {
            if self.parts.len() == 0 {
                panic!("Rule is not defined.");
            }
            
            let mut new_ctx = self.branch(&ctx, true);
            
            for p in &self.parts {
                let r = match *p {
                    ScanFn::All => self.scan_all_leaf(&mut new_ctx),
                    ScanFn::AllExcept(ref exclude) => self.scan_all_except_leaf(&exclude, &mut new_ctx),
                    ScanFn::Alter(ref alter) => self.scan_alter_leaf(&alter, &mut new_ctx),
                    ScanFn::AnyOf(ref rules) => self.scan_any_of(&rules, &mut new_ctx),
                    ScanFn::CharIn(min, max) => self.scan_char_in_leaf(min, max, &mut new_ctx),
                    ScanFn::Eof => self.scan_eof(&mut new_ctx),
                    ScanFn::Literal(find) => self.scan_literal_leaf(&find, &mut new_ctx),
                    ScanFn::Not(ref r) => self.scan_not(&r, &mut new_ctx),
                    ScanFn::Range(min, max, ref r) => self.scan_rule_range(min, max, &r, &mut new_ctx),
                };
                
                if r == -1 {
                    return -1;
                }
            }
            
            self.merge(&mut ctx, &mut new_ctx, true)
        }
        
        // TODO What about a char with more codepoints?
        fn scan_all_leaf(&self, mut ctx: &mut ScanCtx<T>) -> i64
        {
            let n = ctx.code_iter.next();
            
            if let Some(c) = n {
                ctx.lexeme.push(c);
                ctx.index += 1;
                1
            } 
            else {
                self.update_error(&mut ctx, String::from("End of code while checking for not allowed character."))
            }
        }
        
        // TODO What about a char with more codepoints?
        fn scan_all_except_leaf(&self, exclude: &Vec<char>, mut ctx: &mut ScanCtx<T>) -> i64
        {
            let n = ctx.code_iter.next();
            
            if let Some(c) = n {
                if exclude.contains(&c) {
                    return self.update_error(&mut ctx, format!("Character '{}' is not allowed here.", c));
                }
                
                ctx.lexeme.push(c);
                ctx.index += 1;
                1
            } 
            else {
                self.update_error(&mut ctx, String::from("End of code while checking for not allowed character."))
            }
        }
        
        fn scan_alter_leaf(&self, list: &Vec<(&'static str, &'static str)>, mut ctx: &mut ScanCtx<T>) -> i64
        {
            // TODO Is there a nice native substr compare function?
            for find in list {
                let mut found = true;
                let mut len = 0;
                let mut iter = ctx.code_iter.clone();
                
                for f in find.0.chars() {
                    len += 1;
                    
                    let c = iter.next();
                    
                    match c {
                        Some(c) => {
                            if c != f {
                                found = false;
                                break;
                            }
                        },
                        None => {
                            found = false;
                            break; 
                        }
                    }
                }
                
                if found {
                    ctx.code_iter = iter;
                    ctx.lexeme.push_str(find.1);
                    ctx.index += len;
                    return len;
                }
            }
            
            self.update_error(&mut ctx, String::from("Alter characters not found on this position."))
        }
        
        fn scan_any_of<'b>(&'b self, rules: &Vec<&'b Rule<T>>, mut ctx: &mut ScanCtx<'b, T>) -> i64
        {
            for r in rules {
                let mut new_ctx = self.branch(&ctx, false);
                
                if r.run(&mut new_ctx) != -1 {
                    return self.merge(&mut ctx, &mut new_ctx, false);
                }
            }
            
            -1
        }
        
        fn scan_char_in_leaf(&self, min: char, max: char, mut ctx: &mut ScanCtx<T>) -> i64
        {
            let c = ctx.code_iter.next();
            
            match c {
                Some(c) => {
                    if c < min || c > max {
                       self.update_error(&mut ctx, format!("Expected a character between '{}' and '{}'; got a {}.", min, max, c))
                    }
                    else {
                        ctx.lexeme.push(c);
                        ctx.index += 1;
                        1
                    }
                },
                None => {
                    self.update_error(&mut ctx, format!("End of code. Expected a character between '{}' and '{}'.", min, max))
                }
            }
        }
        
        fn scan_eof(&self, mut ctx: &mut ScanCtx<T>) -> i64
        {
            if let None = ctx.code_iter.next() {
                ctx.index += 1;
                1
            }
            else {
                self.update_error(&mut ctx, String::from("No EOF on this position."))
            }
        }
        
        fn scan_literal_leaf(&self, find: &'static str, mut ctx: &mut ScanCtx<T>) -> i64 
        {
            let iter = find.chars();
            let mut step = 0;
            
            for i in iter {
                let n = ctx.code_iter.next();
                
                if let Some(c) = n {
                    if i != c {
                        return self.update_error(&mut ctx, format!("The literal '{}' not found.", find));
                    }
                    
                    ctx.index += 1;
                    step += 1;
                }
                else {
                    return self.update_error(&mut ctx, format!("End of code. The literal '{}' not found.", find));
                }
            }
            
            ctx.lexeme.push_str(find);
            step
        }
        
        fn scan_not(&self, rule: &Rule<T>, mut ctx: &mut ScanCtx<T>) -> i64
        {
            if self.run(&mut self.branch(&ctx, false)) == -1 {
                0
            }
            else {
                -1
            }
        }
        
        fn scan_rule_range<'b>(&'b self, min: u64, max: u64, rule: &'b Rule<T>, mut ctx: &mut ScanCtx<'b, T>) -> i64
        {
            let mut new_ctx = self.branch(&ctx, false);
            let mut count = 0u64;
            
            loop {
                let progress = rule.run(&mut new_ctx);
                
                if progress == -1 {
                    break;
                }
                
                if progress == 0 {
                    return 0;
                }
                
                count += 1;
                
                if count == max {
                    break;
                }
            }
            
            if count >= min && count <= max {
                self.merge(&mut ctx, &mut new_ctx, false)
            }
            else {
                -1
            }
        }
        
        fn update_error(&self, mut new_ctx: &mut ScanCtx<T>, error_msg: String) -> i64
        {
            let errors = &mut new_ctx.errors;
            
            if errors.len() != 0 {
                let err_idx = errors[0].index;
                 
                if new_ctx.index < err_idx {
                    return -1;
                }
                
                if new_ctx.index > err_idx {
                    errors.clear();
                }
            }
            
            errors.push(RuleError {
                index: new_ctx.index,
                msg: error_msg,
                // TODO trail: newCtx.trail.slice(0)
            });
            
            -1
        }
    }
}

#[cfg(test)]
mod tests 
{
    use abitvin::Rule;
    
    #[test]
    fn test_all()
    {
        let code = "abcdefg";
        
        let f = |_: &Vec<bool>, l: &str| {
            assert_eq!(l, "abcdefg");
            vec![true, false, false, true]
        };
        
        let mut r: Rule<bool> = Rule::new(Some(Box::new(f)));
        r.all().all().all().all().all().all().all();
        
        if let Ok(branches) = r.scan(&code) {
            assert_eq!(branches[0], true);
            assert_eq!(branches[1], false);
            assert_eq!(branches[2], false);
            assert_eq!(branches[3], true);
        }
        else {
            assert!(false);
        }
    }
    
    #[test]
    fn test_all_except()
    {
        let code = "abc";
        
        let f = |_: &Vec<u32>, l: &str| {
            assert_eq!(l, "abc");
            vec![0u32, 1u32, 2u32, 3u32]
        };
        
        let excluding = vec!['A', 'B', 'C', 'D']; 
        
        let mut r: Rule<u32> = Rule::new(Some(Box::new(f)));
        r.all_except(&excluding).all_except(&excluding).all_except(&excluding);
        
        if let Ok(branches) = r.scan(&code) {
            assert_eq!(branches[0], 0u32);
            assert_eq!(branches[1], 1u32);
            assert_eq!(branches[2], 2u32);
            assert_eq!(branches[3], 3u32);
        }
        else {
            assert!(false);
        }
    }
    
    #[test]
    fn test_alter()
    {
        let code = "aaabbbccc";
        
        let alterations = vec![
            ("aaa", "AAA"),
            ("bbb", "BBB"),
            ("ccc", "CCC"),
        ];
        
        let f = |_: &Vec<i32>, l: &str| {
            assert_eq!(l, "AAABBBCCC");
            vec![111, 222]
        }; 
        
        let mut r: Rule<i32> = Rule::new(Some(Box::new(f)));
        r.alter(&alterations).alter(&alterations).alter(&alterations);
        
        if let Ok(branches) = r.scan(&code) {
            assert_eq!(branches[0], 111);
            assert_eq!(branches[1], 222);
        }
        else {
            assert!(false);
        }
    }
    
    #[test]
    fn test_any_of()
    {
        let code = "aaabbbccc";
        
        let aaa_fn = |_: &Vec<i32>, l: &str| {
            assert_eq!(l, "aaa");
            vec![111]
        }; 
        
        let bbb_fn = |_: &Vec<i32>, l: &str| {
            assert_eq!(l, "bbb");
            vec![222]
        };
        
        let ccc_fn = |_: &Vec<i32>, l: &str| {
            assert_eq!(l, "ccc");
            vec![333]
        };
        
        let mut aaa: Rule<i32> = Rule::new(Some(Box::new(aaa_fn)));
        aaa.literal("aaa");
        
        let mut bbb: Rule<i32> = Rule::new(Some(Box::new(bbb_fn)));
        bbb.literal("bbb");
        
        let mut ccc: Rule<i32> = Rule::new(Some(Box::new(ccc_fn)));
        ccc.literal("ccc");
        
        let these = &vec![&aaa, &bbb, &ccc];
        
        let mut root: Rule<i32> = Rule::new(None);
        root.any_of(&these).any_of(&these).any_of(&these);
        
        if let Ok(branches) = root.scan(&code) {
            assert_eq!(branches[0], 111);
            assert_eq!(branches[1], 222);
            assert_eq!(branches[2], 333);
        }
        else {
            assert!(false);
        }
    }
    
    #[test]
    fn test_at_least()
    {
        let code = "xxxx";
        
        let mut x: Rule<i32> = Rule::new(Some(Box::new(|_, _| vec![10] )));
        x.literal("x");
       
        let mut root: Rule<i32> = Rule::new(None);
        
        if let Ok(branches) = root.at_least(3, &x).scan(&code) {
            assert_eq!(branches[0], 10);
            assert_eq!(branches[1], 10);
            assert_eq!(branches[2], 10);
            assert_eq!(branches[3], 10);
        }
        else {
            assert!(false);
        }
        
        if let Ok(branches) = root.clear().at_least(4, &x).scan(&code) {
            assert_eq!(branches[0], 10);
            assert_eq!(branches[1], 10);
            assert_eq!(branches[2], 10);
            assert_eq!(branches[3], 10);
        }
        else {
            assert!(false);
        }
        
        if let Ok(branches) = root.clear().at_least(5, &x).scan(&code) {
            assert!(false);
        }
        else {
            assert!(true);
        }
    }
    
    #[test]
    fn test_at_most()
    {
        let code = "yyy";
        
        let mut y: Rule<i32> = Rule::new(Some(Box::new(|_, _| vec![14] )));
        y.literal("y");
       
        let mut root: Rule<i32> = Rule::new(None);
        
        if let Ok(branches) = root.at_most(2, &y).scan(&code) {
            assert!(false);
        }
        else {
            assert!(true);
        }
        
        if let Ok(branches) = root.clear().at_most(3, &y).scan(&code) {
            assert_eq!(branches[0], 14);
            assert_eq!(branches[1], 14);
            assert_eq!(branches[2], 14);
        }
        else {
            assert!(false);
        }
        
        if let Ok(branches) = root.clear().at_most(4, &y).scan(&code) {
            assert_eq!(branches[0], 14);
            assert_eq!(branches[1], 14);
            assert_eq!(branches[2], 14);
        }
        else {
            assert!(false);
        }
    }
    
    #[test]
    fn test_between()
    {
        let code = "zzz";
        
        let mut z: Rule<i32> = Rule::new(Some(Box::new(|_, _| vec![34])));
        z.literal("z");
       
        let mut root: Rule<i32> = Rule::new(None);
        
        if let Ok(branches) = root.between(1, 3, &z).scan(&code) {
            assert_eq!(branches[0], 34);
            assert_eq!(branches[1], 34);
            assert_eq!(branches[2], 34);
        }
        else {
            assert!(true);
        }
        
        if let Ok(branches) = root.clear().between(0, 10, &z).scan(&code) {
            assert_eq!(branches[0], 34);
            assert_eq!(branches[1], 34);
            assert_eq!(branches[2], 34);
        }
        else {
            assert!(false);
        }
        
        if let Ok(branches) = root.clear().between(4, 5, &z).scan(&code) {
            assert!(false);
        }
        else {
            assert!(true);
        }
    }
    
    #[test]
    fn test_char_in()
    {
        let mut digit: Rule<u32> = Rule::new(Some(Box::new(|_, l| vec![(l.chars().next().unwrap() as u32) - 48])));
        digit.char_in('0', '9');
        
        let mut af: Rule<u32> = Rule::new(Some(Box::new(|_, l| vec![(l.chars().next().unwrap() as u32) - 55])));
        af.char_in('A', 'F');
        
        let one_of_these = vec![&digit, &af];
        
        let mut hex: Rule<u32> = Rule::new(None);
        hex.any_of(&one_of_these);
        
        let mut parse: Rule<u32> = Rule::new(Some(Box::new(|b, _| 
        {
            let mut m = 1u32;
            let mut n = 0u32;
            
            for i in b.iter().rev() {
                n += i * m;
                m <<= 4;
            }
            
            vec![n]
        })));
        parse.between(1, 8, &hex);
        
        if let Ok(branches) = parse.scan("A") {
            assert_eq!(branches[0], 10);
        }
        else {
            assert!(false);
        }
        
        if let Ok(branches) = parse.scan("12345678") {
            assert_eq!(branches[0], 305419896);
        }
        else {
            assert!(false);
        }
        
        if let Ok(branches) = parse.scan("FF") {
            assert_eq!(branches[0], 255);
        }
        else {
            assert!(false);
        }
        
        if let Ok(branches) = parse.scan("FFFFFFFF") {
            assert_eq!(branches[0], u32::max_value());
        }
        else {
            assert!(false);
        }
        
        if let Ok(branches) = parse.scan("FFFFFFFFF") {
            assert!(false);
        }
        else {
            assert!(true);
        }
        
        if let Ok(branches) = parse.scan("FFxFF") {
            assert!(false);
        }
        else {
            assert!(true);
        }
        
        if let Ok(branches) = parse.scan("") {
            assert!(false);
        }
        else {
            assert!(true);
        }
    }
    
    #[test]
    #[should_panic]
    fn test_clear()
    {
        let code = "Ello'";
        
        let mut r: Rule<char> = Rule::new(None);
        r.literal("Ello'");
        r.clear();
        r.scan(&code);   // Panic! We cleared the rule.
    }
    
    #[test]
    fn test_eof()
    {
        let code = "123";
        
        let mut r: Rule<char> = Rule::new(Some(Box::new(|_, _| vec!['A', 'B'] )));
        r.literal("123").eof();
        
        if let Ok(branches) = r.scan(&code) {
            assert_eq!(branches[0], 'A');
            assert_eq!(branches[1], 'B');
        }
        else {
            assert!(false);
        }
    }
    
    #[test]
    fn test_exact()
    {
        let code = "..........";
        
        let mut dot: Rule<char> = Rule::new(Some(Box::new(|_, _| vec!['.'] )));
        dot.literal(".");
        
        let mut nope: Rule<char> = Rule::new(Some(Box::new(|_, _| vec!['x'] )));
        nope.literal("nope");
        
        let mut root: Rule<char> = Rule::new(None);
        
        if let Ok(branches) = root.exact(10, &dot).scan(&code) {
            assert!(branches.len() == 10 && branches.into_iter().any(|c| c == '.'));
        }
        else {
            assert!(false);
        }
        
        if let Ok(branches) = root.clear().exact(9, &dot).scan(&code) {
            assert!(false);
        }
        else {
            assert!(true);
        }
        
        if let Ok(branches) = root.clear().exact(11, &dot).scan(&code) {
            assert!(false);
        }
        else {
            assert!(true);
        }
        
        if let Ok(branches) = root.clear().exact(0, &nope).exact(10, &dot).exact(0, &nope).scan(&code) {
            assert!(branches.len() == 10 && branches.into_iter().any(|c| c == '.'));
        }
        else {
            assert!(false);
        }
    }
    
    #[test]
    fn test_literal()
    {
        let code = "y̆y̆y̆x̆";
        
        let mut r: Rule<u64> = Rule::new(Some(Box::new(|_, l| 
        {
            assert_eq!(l, "y̆y̆y̆x̆");
            vec![7777u64, 8888u64, 9999u64]
        })));
        
        r.literal("y̆y̆").literal("y̆").literal("x̆");
        
        if let Ok(branches) = r.scan(&code) {
            assert_eq!(branches[0], 7777u64);
            assert_eq!(branches[1], 8888u64);
            assert_eq!(branches[2], 9999u64);
        }
        else {
            assert!(false);
        }
    }
    
    #[test]
    fn test_maybe()
    {
        let codes = vec![
            "xxx",
            "...xxx",
            "xxx...",
            "...xxx...",
        ];
        
        let mut dots: Rule<char> = Rule::new(None);
        dots.literal("...");
        
        let mut xxx: Rule<char> = Rule::new(Some(Box::new(|_, _| vec!['x'] )));
        xxx.literal("xxx");
        
        let mut root: Rule<char> = Rule::new(None);
        root.maybe(&dots).one(&xxx).maybe(&dots);
        
        for c in codes {
            if let Ok(branches) = root.scan(&c) {
                assert!(branches.len() == 1 && branches[0] == 'x');
            }
            else {
                assert!(false);
            }
        }
    }
    
    #[test]
    fn test_none_or_many()
    {
        let mut dot: Rule<bool> = Rule::new(Some(Box::new(|b, l| vec![true])));
        dot.literal(".");
        
        let mut x: Rule<bool> = Rule::new(Some(Box::new(|b, l| vec![false])));
        x.literal("x");
        
        let mut code1: Rule<bool> = Rule::new(Some(Box::new(|b, l|
        {
            assert_eq!(b.len(), 0);
            assert_eq!(l, "");
            Vec::new()
        })));
        
        let mut code2: Rule<bool> = Rule::new(Some(Box::new(|b, l|
        {
            assert_eq!(b.len(), 1);
            assert_eq!(b[0], false);
            assert_eq!(l, "x");
            Vec::new()
        })));
        
        let mut code3: Rule<bool> = Rule::new(Some(Box::new(|b, l|
        {
            assert_eq!(b.len(), 2);
            assert_eq!(b[0], true);
            assert_eq!(b[1], true);
            assert_eq!(l, "..");
            Vec::new()
        })));
        
        let mut code4: Rule<bool> = Rule::new(Some(Box::new(|b, l|
        {
            assert_eq!(b.len(), 3);
            assert_eq!(b[0], false);
            assert_eq!(b[1], false);
            assert_eq!(b[2], true);
            assert_eq!(l, "xx.");
            Vec::new()
        })));
        
        let mut code5: Rule<bool> = Rule::new(Some(Box::new(|b, l|
        {
            assert_eq!(b.len(), 4);
            assert_eq!(b[0], true);
            assert_eq!(b[1], true);
            assert_eq!(b[2], false);
            assert_eq!(b[3], false);
            assert_eq!(l, "..xx");
            Vec::new()
        })));
        
        if let Err(_) = code1.none_or_many(&dot).none_or_many(&x).none_or_many(&dot).scan("") {
            assert!(false);
        }
        
        if let Err(_) = code2.none_or_many(&dot).none_or_many(&x).none_or_many(&dot).scan("x") {
            assert!(false);
        }
        
        if let Err(_) = code3.none_or_many(&dot).none_or_many(&x).none_or_many(&dot).scan("..") {
            assert!(false);
        }
        
        if let Err(_) = code4.none_or_many(&dot).none_or_many(&x).none_or_many(&dot).scan("xx.") {
            assert!(false);
        }
        
        if let Err(_) = code5.none_or_many(&dot).none_or_many(&x).none_or_many(&dot).scan("..xx") {
            assert!(false);
        }
    }
    
    #[test]
    fn test_not()
    {
        let mut not_this: Rule<i32> = Rule::new(None);
        not_this.literal("not this");
        
        let mut r: Rule<i32> = Rule::new(None);
        r.literal("aaa").not(&not_this).literal("bbb").literal("ccc");
        
        if let Ok(_) = r.scan("aaabbbccc") {
            assert!(true);
        }
        else {
            assert!(false);
        }
        
        if let Ok(_) = r.scan("aaanot thisbbbccc") {
            assert!(false);
        }
        else {
            assert!(true);
        }
    }
    
    #[test]
    fn test_one()
    {
        let code = "onetwothree";
        
        let mut one: Rule<i32> = Rule::new(Some(Box::new(|_, _| vec![1] )));
        one.literal("one");
        
        let mut two: Rule<i32> = Rule::new(Some(Box::new(|_, _| vec![2] )));
        two.literal("two");
        
        let mut three: Rule<i32> = Rule::new(Some(Box::new(|_, _| vec![3] )));
        three.literal("three");
        
        let mut root: Rule<i32> = Rule::new(None);
        root.one(&one).one(&two).one(&three);
        
        if let Ok(branches) = root.scan(&code) {
            assert_eq!(branches[0], 1);
            assert_eq!(branches[1], 2);
            assert_eq!(branches[2], 3);
        }
        else {
            assert!(false);
        }
    }
}


//#![feature(test)]

//extern crate test;

/*
https://medium.com/@jreem/advanced-rust-using-traits-for-argument-overloading-c6a6c8ba2e17#.d0sagshgi
impl<R> IntoReader for R where R: Reader {
    type OutReader = R;
    fn into_reader(self) -> R {
        self
    }
}
*/

/*
// http://stackoverflow.com/questions/30253422/how-to-print-structs-and-arrays
impl std::fmt::Display for Vector
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        write!(f, "Vector {{ x: {}, y: {} }}", self.x, self.y)
    }
}
*/

/*

namespace Abitvin
{
    export class RuleResult<TBranch, TMeta>
    {
        constructor(
            public branches: TBranch[],
            public errors: IRuleError<TMeta>[],
            public isSuccess: boolean
        ) {}
        
        public static failed<TBranch, TMeta>(errors: IRuleError<TMeta>[]): RuleResult<TBranch, TMeta>
        {
            return new RuleResult<TBranch, TMeta>(null, errors, false);
        }
        
        public static success<TBranch, TMeta>(branches: TBranch[]): RuleResult<TBranch, TMeta>
        {
            return new RuleResult<TBranch, TMeta>(branches, null, true);
        }
    }
    
	
    export class Rule<TBranch, TMeta>
	{
        private _branchFn: BranchFn<TBranch>;
		private _meta: TMeta;
        private _parts: {(ctx): number}[];
        
		constructor(branchFn: BranchFn<TBranch> = null, meta: TMeta = null)
        {
			this._branchFn = branchFn;
            this._meta = meta;
            this._parts = [];
		}
        
        public set branchFn(value: BranchFn<TBranch>) { this._branchFn = value; }
        public get meta(): TMeta { return this._meta; }
        public set meta(value: TMeta) { this._meta = value; }
       



		
        
        
        
        

		
        
        
        
		private showCode(text: string, position: number): void
        {
            console.error(text.substr(position, 40));
        }
	}
}
*/