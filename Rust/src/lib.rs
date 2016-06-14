// Copyright (c) 2015-2016 Abitvin <foss@abitvin.net>
// Licensed under the MIT license <LICENSE.md or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

mod abitvin
{
    use std::str::Chars;

    // TODO What is the best way to store the branch closure?
    // http://stackoverflow.com/questions/27831944/how-do-i-store-a-closure-in-rust
    // TODO Simplify `Option<Box<Fn<...>>>` if we can.
    // TODO I think we can do `Fn(Vec<T>, &str)`.
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
        AllExcept(Vec<char>),
        Alter(Vec<(&'static str, &'static str)>),
        AnyOf(Vec<&'a Rule<'a, T>>),
        AnyOfOwned(Vec<Rule<'a, T>>),
        CharIn(char, char),
        Eof,
        Literal(&'static str),
        Placeholder,
        Not(&'a Rule<'a, T>),
        NotOwned(Rule<'a, T>),
        Range(u64, u64, &'a Rule<'a, T>),
        RangeOwned(u64, u64, Rule<'a, T>)
    }

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

        pub fn all_except(&mut self, exclude: Vec<char>) -> &mut Self
        {
            if exclude.len() == 0 {
                panic!("List of excluded characters is empty.");
            }
            
            self.parts.push(ScanFn::AllExcept(exclude));
            self
        }

        pub fn alter(&mut self, list: Vec<(&'static str, &'static str)>) -> &mut Self
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
        
        pub fn any_of(&mut self, rules: Vec<&'a Rule<'a, T>>) -> &mut Self
        {
            if rules.len() == 0 {
                panic!("You must specify rules.");
            }
            
            self.parts.push(ScanFn::AnyOf(rules));
            self
        }

        pub fn any_of_owned(&mut self, rules: Vec<Rule<'a, T>>) -> &mut Self
        {
            if rules.len() == 0 {
                panic!("You must specify rules.");
            }
            
            self.parts.push(ScanFn::AnyOfOwned(rules));
            self
        }

        pub fn at_least(&mut self, count: u64, rule: &'a Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(count, u64::max_value(), &rule));
            self
        }

        pub fn at_least_owned(&mut self, count: u64, rule: Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(count, u64::max_value(), rule));
            self
        }
        
        pub fn at_most(&mut self, count: u64, rule: &'a Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(0, count, &rule));
            self
        }
        
        pub fn at_most_owned(&mut self, count: u64, rule: Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(0, count, rule));
            self
        }
        
        pub fn between(&mut self, min: u64, max: u64, rule: &'a Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(min, max, &rule));
            self
        }
        
        pub fn between_owned(&mut self, min: u64, max: u64, rule: Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(min, max, rule));
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

        pub fn exact_owned(&mut self, count: u64, rule: Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(count, count, rule));
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
        
        pub fn maybe_owned(&mut self, rule: Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(0, 1, rule));
            self
        }
        
        pub fn none_or_many(&mut self, rule: &'a Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(0, u64::max_value(), &rule));
            self
        }
        
        pub fn none_or_many_owned(&mut self, rule: Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(0, u64::max_value(), rule));
            self
        }
        
        pub fn not(&mut self, rule: &'a Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::Not(&rule));
            self
        }
        
        pub fn not_owned(&mut self, rule: Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::NotOwned(rule));
            self
        }
        
        pub fn one(&mut self, rule: &'a Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(1, 1, &rule));
            self
        }

        pub fn one_owned(&mut self, rule: Rule<'a, T>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(1, 1, rule));
            self
        }

        // TODO I think `code` needs to be a String 
        pub fn scan(&self, code: &'static str) -> Result<Vec<T>, Vec<RuleError>>
        {
            let mut ctx = ScanCtx {
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

        fn branch<'b>(&'b self, ctx: &ScanCtx<'b, T>, is_root_of_rule: bool) -> ScanCtx<T>
        {
            let new_ctx = ScanCtx {
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
        
        fn merge<'b>(&self, target: &mut ScanCtx<'b, T>, source: &mut ScanCtx<'b, T>, is_root_of_rule: bool) -> i64
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
        fn run<'b>(&'b self, mut ctx: &mut ScanCtx<'b, T>) -> i64
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
                    ScanFn::AnyOf(ref rules) => self.scan_any_of(rules, &mut new_ctx),
                    ScanFn::AnyOfOwned(ref rules) => self.scan_any_of_owned(rules, &mut new_ctx),
                    ScanFn::CharIn(min, max) => self.scan_char_in_leaf(min, max, &mut new_ctx),
                    ScanFn::Eof => self.scan_eof(&mut new_ctx),
                    ScanFn::Literal(find) => self.scan_literal_leaf(&find, &mut new_ctx),
                    ScanFn::Not(ref r) => self.scan_not(&r, &mut new_ctx),
                    ScanFn::NotOwned(ref r) => self.scan_not(&r, &mut new_ctx),
                    ScanFn::Placeholder => panic!("Error, placeholder found."),
                    ScanFn::Range(min, max, ref r) => self.scan_rule_range(min, max, &r, &mut new_ctx),
                    ScanFn::RangeOwned(min, max, ref r) => self.scan_rule_range(min, max, &r, &mut new_ctx),
                };
                
                if r == -1 {
                    return -1;
                }
            }
            
            self.merge(&mut ctx, &mut new_ctx, true)
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
        
        fn scan_any_of<'b>(&'b self, rules: &Vec<&'a Rule<T>>, mut ctx: &mut ScanCtx<'b, T>) -> i64
        {
            for r in rules {
                let mut new_ctx = self.branch(&ctx, false);

                if r.run(&mut new_ctx) != -1 {
                    return self.merge(&mut ctx, &mut new_ctx, false);
                }
            }
            
            -1
        }

        fn scan_any_of_owned<'b>(&'b self, rules: &'b Vec<Rule<T>>, mut ctx: &mut ScanCtx<'b, T>) -> i64
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
            if rule.run(&mut self.branch(&ctx, false)) == -1 {
                0
            }
            else {
                -1
            }
        }
        
        fn scan_rule_range<'b>(&'b self, min: u64, max: u64, rule: &'a Rule<T>, mut ctx: &mut ScanCtx<'b, T>) -> i64
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

    /////////////
    // Grammer //
    /////////////
    
    // TODO Put Grammer in its own file/container.
    /*
    use std::collections::BTreeMap;
    
    enum RangeType
    {
        NoRangeType = 0,
        AtLeast,
        AtMost,
        Between,
        Exact,
        Not
    }
    
    struct ParseContext<'a, T>
    {
        arg1: u64,
        arg2: u64,
        arg3: &'static str,
        range_type: RangeType,
        rule: Box<Rule<'a, T>>
    }
    
    // TODO TMeta class R<TB, TM> extends Rule<IParseContext<TB, TM>, IEmpty> {}
    type R<'a, T> = Rule<'a, ParseContext<'a, T>>;
    
    // TODO Note: This was IRule, remove this comment later after porting.
    struct RuleExpr<'a, T>
    {
        id: &'static str,
        is_defined: bool,
        rule: Rule<'a, T>
    }
    
    pub struct Grammer<'a, T> /* <TBranch, TMeta> */
    {
        grammer: R<'a, T>,
        rule_exps: BTreeMap<&'static str, RuleExpr<'a, T>>,
        ws: Rule<'a, T>,
    }
    
    impl<'a, T> Grammer<'a, T>
    {
        pub fn new() -> Self
        {


            Grammer {
                // TODO this._grammer = new R<TBranch, TMeta>().noneOrMany(statement);
                grammer: R::new(None), 
                rule_exps: BTreeMap::new(),
                ws: Rule::new(None),
            }
            
            /*
            this._ws = new Rule<TBranch, TMeta>().anyOf(" ", "\t", "\n", "\r");
            
            const statementFn = (b) =>
            {
                if (b[0].rangeType !== RangeType.Not)
                    return b;
                  
                return { 
                    arg1: null,
                    arg2: null,
                    arg3: null,
                    rangeType: RangeType.NoRangeType,
                    rule: new Rule<TBranch, TMeta>().not(b[1].rule) 
                };
            };
            
            const ranges = new R<TBranch, TMeta>();
            const statement = new R<TBranch, TMeta>(statementFn);
            
            const escapedCtrlChars = new R<TBranch, TMeta>().alter(
                "\\<", "<", 
                "\\>", ">", 
                "\\{", "{", 
                "\\}", "}", 
                "\\(", "(", 
                "\\)", ")", 
                "\\[", "[", 
                "\\]", "]", 
                "\\^", "^",
                "\\~", "~",
                "\\-", "-",
                "\\,", ",",
                "\\|", "|",
                "\\+", "+", 
                "\\?", "?", 
                "\\*", "*", 
                "\\.", ".", 
                "\\$", "$",
                "\\ ", " ", 
                "\\_", "_",
                "\\!", "!"
            );
            
            // Integer
            const integerFn = (b, l) => ({
                arg1: parseInt(l), 
                arg2: null,
                arg3: null, 
                rangeType: RangeType.NoRangeType, 
                rule: null 
            });
            
            const digit = new R<TBranch, TMeta>().between("0", "9");
            const integer = new R<TBranch, TMeta>(integerFn).atLeast(1, digit);
            
            // Literal
            const literalTextFn = (b, l) => ({
                arg1: null, 
                arg2: null,
                arg3: l, 
                rangeType: RangeType.NoRangeType, 
                rule: null 
            });
            
            const literalAllExcept = new R<TBranch, TMeta>().allExcept("<", "{", "(", ")", "|", "[", "+", "?", "*", ".", "$", " ", "_", "!");
            const literalChar = new R<TBranch, TMeta>().anyOf(escapedCtrlChars, literalAllExcept);
            const literalText = new R<TBranch, TMeta>(literalTextFn).atLeast(1, literalChar);
            
            const literalFn = (b, l) =>
            {
                const text = b[0].arg3;
                let rule = new Rule<TBranch, TMeta>().literal(text);
                
                if (b.length === 2)
                    rule = this.addRange(rule, b[1]);
                   
                return { 
                    arg1: null,
                    arg2: null,
                    arg3: null,
                    rangeType: RangeType.NoRangeType,
                    rule: rule 
                };
            };
            
            const literal = new R<TBranch, TMeta>(literalFn).one(literalText).maybe(ranges);
            
            // Any char
            const anyCharFn = (b, l) =>
            {
                let rule = new Rule<TBranch, TMeta>().all();
                
                if (b.length === 1)
                    rule = this.addRange(rule, b[0]);
                
                return {
                    arg1: null,
                    arg2: null,
                    arg3: null,
                    rangeType: RangeType.NoRangeType,
                    rule: rule 
                };
            };
             
            const anyChar = new R<TBranch, TMeta>(anyCharFn).literal(".").maybe(ranges);
            
            // All except
            const allExceptCharsFn = (b, l) => ({
                arg1: null,
                arg2: null,
                arg3: l,
                rangeType: RangeType.NoRangeType,
                rule: null
            });
            
            const allExceptFn = (b, l) =>
            {
                let rule = new Rule<TBranch, TMeta>().allExcept(b[0].arg3.split(""));
                const last = b[b.length - 1];
                
                if (last.rangeType !== RangeType.NoRangeType)
                    rule = this.addRange(rule, last);
                
                return { 
                    arg1: null,
                    arg2: null,
                    arg3: null,
                    rangeType: RangeType.NoRangeType,
                    rule: rule 
                };
            };
            
            const allExceptAnyOther = new R<TBranch, TMeta>().allExcept("]");
            const allExceptChar = new R<TBranch, TMeta>().anyOf(escapedCtrlChars, allExceptAnyOther);
            const allExceptChars = new R<TBranch, TMeta>(allExceptCharsFn).atLeast(1, allExceptChar);
            const allExcept = new R<TBranch, TMeta>(allExceptFn).literal("[^").one(allExceptChars).literal("]").maybe(ranges);
            
            // Match character range
            const charRangeFn = (b, l) => l.split("-").map(c => ({
                arg1: null,
                arg2: null,
                arg3: c,
                rangeType: RangeType.NoRangeType,
                rule: null
            }));
            
            const charRangesFn = (b, l) =>
            {
                let rule = new Rule<TBranch, TMeta>();
                
                if (b.length > 2)
                {
                    const ranges: Rule<TBranch, TMeta>[] = [];
                    
                    for (let i: number = 0; i < b.length - 1; i += 2)
                        ranges.push(new Rule<TBranch, TMeta>().between(b[i].arg3, b[i + 1].arg3));
                    
                    rule.anyOf(ranges);
                }
                else
                {
                    rule.between(b[0].arg3, b[1].arg3);
                }
                
                const last = b[b.length - 1];
                
                if (last.rangeType !== RangeType.NoRangeType)
                    rule = this.addRange(rule, last);
                
                return {
                    arg1: null,
                    arg2: null,
                    arg3: null,
                    rangeType: RangeType.NoRangeType,
                    rule: rule 
                };
            };
            
            const charRangeAllExcept = new R<TBranch, TMeta>().allExcept("-", "]");
            const charRangeChar = new R<TBranch, TMeta>().anyOf(escapedCtrlChars, charRangeAllExcept);
            const charRange = new R<TBranch, TMeta>(charRangeFn).one(charRangeChar).literal("-").one(charRangeChar);
            const charRanges = new R<TBranch, TMeta>(charRangesFn).literal("[").atLeast(1, charRange).literal("]").maybe(ranges);
            
            // EOF
            const eofFn = (b, l) => ({
                arg1: null,
                arg2: null,
                arg3: null,
                rangeType: RangeType.NoRangeType,
                rule: new Rule<TBranch, TMeta>().eof()
            });
            
            const eof = new R<TBranch, TMeta>(eofFn).literal("$");
            
            // One rule
            const ruleNameFn = (b, l) => ({
                arg1: null, 
                arg2: null,
                arg3: l, 
                rangeType: RangeType.NoRangeType, 
                rule: null 
            });
            
            const ruleNameAllExcept = new R<TBranch, TMeta>().allExcept(">");
            const ruleNameChar = new R<TBranch, TMeta>().anyOf(escapedCtrlChars, ruleNameAllExcept);
            const ruleName = new R<TBranch, TMeta>(ruleNameFn).atLeast(1, ruleNameChar);
            
            const ruleFn = (b, l) =>
            {
                const id = b[0].arg3;
                const r = this._rulexps[id];
                
                if (r == null)
                    throw new Error(`Rule "${id}" not found.`);
                
                let rule = null;
                
                if (b.length === 1)
                    rule = new Rule<TBranch, TMeta>().one(r.rule);
                else
                    rule = this.addRange(r.rule, b[1]);
                
                return {
                    arg1: null,
                    arg2: null,
                    arg3: null,
                    rangeType: RangeType.NoRangeType,
                    rule: rule 
                };
            };
             
            const rule = new R<TBranch, TMeta>(ruleFn).literal("<").one(ruleName).literal(">").maybe(ranges);
            
            // At least
            const atLeastFn = (b, l) => ({
                arg1: b[0].arg1,
                arg2: null,
                arg3: null,
                rangeType: RangeType.AtLeast,
                rule: null
            });
            
            const atLeast = new R<TBranch, TMeta>(atLeastFn).literal("{").one(integer).literal(",}");
            
            // At least one
            const atLeastOneFn = (b, l) => ({
                arg1: 1,
                arg2: null,
                arg3: null,
                rangeType: RangeType.AtLeast,
                rule: null
            });
            
            const atLeastOne = new R<TBranch, TMeta>(atLeastOneFn).literal("+");
            
            // At most
            const atMostFn = (b, l) => ({
                arg1: b[0].arg1,
                arg2: null,
                arg3: null,
                rangeType: RangeType.AtMost,
                rule: null
            });
            
            const atMost = new R<TBranch, TMeta>(atMostFn).literal("{,").one(integer).literal("}");
            
            // Between
            const betweenFn = (b, l) => ({
                arg1: b[0].arg1,
                arg2: b[1].arg1,
                arg3: null,
                rangeType: RangeType.Between,
                rule: null
            });
            
            const between = new R<TBranch, TMeta>(betweenFn).literal("{").one(integer).literal(",").one(integer).literal("}");
            
            // Exact
            const exactFn = (b, l) => ({
                arg1: b[0].arg1,
                arg2: null,
                arg3: null,
                rangeType: RangeType.Exact,
                rule: null
            });
            
            const exact = new R<TBranch, TMeta>(exactFn).literal("{").one(integer).literal("}");
            
            // Maybe
            const maybeFn = (b, l) => ({
                arg1: 0,
                arg2: 1,
                arg3: null,
                rangeType: RangeType.Between,
                rule: null
            });
            
            const maybe = new R<TBranch, TMeta>(maybeFn).literal("?");
            
            // None or many
            const noneOrManyFn = (b, l) => ({
                arg1: 0,
                arg2: null,
                arg3: null,
                rangeType: RangeType.AtLeast,
                rule: null
            });
            
            const noneOrMany = new R<TBranch, TMeta>(noneOrManyFn).literal("*");
            
            // Not
            const notFn = () => ({
                arg1: 0,
                arg2: null,
                arg3: null,
                rangeType: RangeType.Not,
                rule: null
            });
            
            const not = new R<TBranch, TMeta>(notFn).literal("!");
            
            // Any of
            const anyOfFn = (b, l) =>
            {
                const last = b[b.length - 1];
                let rules = b.map(r => r.rule);
                
                if (last.rangeType !== RangeType.NoRangeType)
                    rules = rules.slice(0, -1);
                    
                const rule = this.addRange(new Rule<TBranch, TMeta>().anyOf(rules), last);
                
                return {
                    arg1: null,
                    arg2: null,
                    arg3: null,
                    rangeType: RangeType.NoRangeType,
                    rule: rule
                };
            }
            
            const statementsFn = (b, l) =>
            {
                if (b.length === 1)
                   return b; 
                
                const rule = new Rule<TBranch, TMeta>();
                
                for (const pc of b)
                    rule.one(pc.rule);
                 
                return {
                    arg1: null,
                    arg2: null,
                    arg3: null,
                    rangeType: RangeType.NoRangeType,
                    rule: rule
                };
            };
            
            const statements = new R<TBranch, TMeta>(statementsFn).atLeast(1, statement); 
            const more = new R<TBranch, TMeta>().literal("|").one(statements);
            const anyOf = new R<TBranch, TMeta>(anyOfFn).literal("(").one(statements).noneOrMany(more).literal(")").maybe(ranges);
            
            // Alter
            const alterFn = (b, l) => 
            {
                const last = b[b.length - 1];
                
                if (last.rangeType !== RangeType.NoRangeType)
                    b = b.slice(0, -1);
                
                return {
                    arg1: null,
                    arg2: null,
                    arg3: null,
                    rangeType: RangeType.NoRangeType,
                    rule: this.addRange(new Rule<TBranch, TMeta>().alter(b.map(i => i.arg3)), last)
                };
            };
            
            const alterTextFn = (b, l) => ({
                arg1: null,
                arg2: null,
                arg3: l,
                rangeType: RangeType.NoRangeType,
                rule: null
            });
            
            const alterAllExceptLeftChar = new R<TBranch, TMeta>().allExcept(",");
            const alterLeftChar = new R<TBranch, TMeta>().anyOf(escapedCtrlChars, alterAllExceptLeftChar);
            const alterLeftText = new R<TBranch, TMeta>(alterTextFn).atLeast(1, alterLeftChar);
            const alterAllExceptRightChar = new R<TBranch, TMeta>().allExcept("|", ")");
            const alterRightChar = new R<TBranch, TMeta>().anyOf(escapedCtrlChars, alterAllExceptRightChar);
            const alterRightText = new R<TBranch, TMeta>(alterTextFn).atLeast(1, alterRightChar);
            const alterText = new R<TBranch, TMeta>().one(alterLeftText).literal(",").one(alterRightText);
            const alterMore = new R<TBranch, TMeta>().literal("|").one(alterText);
            const alter = new R<TBranch, TMeta>(alterFn).literal("(~").one(alterText).noneOrMany(alterMore).literal(")").maybe(ranges);
            
            // Whitespace
            const atLeastOneWsFn = () => ({
                arg1: null,
                arg2: null,
                arg3: null,
                rangeType: RangeType.NoRangeType,
                rule: new Rule<TBranch, TMeta>().atLeast(1, this._ws)
            });
            
            const noneOrManyWsFs = () => ({
                arg1: null,
                arg2: null,
                arg3: null,
                rangeType: RangeType.NoRangeType,
                rule: new Rule<TBranch, TMeta>().noneOrMany(this._ws)
            });
            
            const atLeastOneWs = new R<TBranch, TMeta>(atLeastOneWsFn).literal("_");
            const noneOrManyWs = new R<TBranch, TMeta>(noneOrManyWsFs).literal(" ");
            
            // Ranges and statements definitions
            ranges.anyOf(atLeast, atLeastOne, atMost, between, exact, maybe, noneOrMany);
            statement.maybe(not).anyOf(anyChar, noneOrManyWs, atLeastOneWs, eof, alter, allExcept, charRanges, rule, anyOf, literal);
            
            this._grammer = new R<TBranch, TMeta>().noneOrMany(statement);
            this._rulexps = {};
            */
        }
        
        /*
        public add(id: string, expr: string, branchFn: BranchFn<TBranch> = null, meta: TMeta = null): void
        {
            const rulexp: IRule<TBranch, TMeta> = this._rulexps[id];
            
            if (rulexp != null && rulexp.isDefined)
                throw new Error(`The rule "${id}" already used.`);
            
            const result = this._grammer.scan(expr);
                    
            // TODO Show nice errors.
            if (!result.isSuccess)
                throw new Error("Error compiling rule expression.");
            
            if (rulexp == null)
            {
                const compiled = result.branches[0].rule;
                
                for (let i: number = 1; i < result.branches.length; i++)
                    compiled.one(result.branches[i].rule);
                
                compiled.branchFn = branchFn;
                compiled.meta = meta;
                
                this._rulexps[id] = {
                    id: id,
                    isDefined: true,
                    rule: compiled
                };
            }
            else
            {
                rulexp.isDefined = true;
                
                for (const r of result.branches)
                    rulexp.rule.one(r.rule);
                    
                rulexp.rule.branchFn = branchFn;
                rulexp.rule.meta = meta;
            }
        }
        */
        
        pub fn declare(&mut self, ids: Vec<&'static str>)
        {
            for id in ids {
                if self.rule_exps.contains_key(id) {
                    panic!("The rule \"{}\" already used.", id);
                }
                
                self.rule_exps.insert(id, RuleExpr {
                    id: id,
                    is_defined: false,
                    rule: Rule::new(None),
                });
            }
        }
        
        // TODO Make a type of this return type.
        pub fn scan(&self, root_id: &'static str, code: &'static str) -> Result<Vec<T>, Vec<RuleError>>
        {
            match self.rule_exps.get(root_id) {
                Some(r) => r.rule.scan(code),
                None => panic!("Rule with id \"{}\" not found.", root_id),
            }
        }
        
        pub fn ws(&mut self, expr: &'static str) 
        {
            // TODO
            panic!("Not implemented yet.");
            /*
            match self.grammer.scan(expr) {
                Ok(ref b) => {
                    let r = b[0].rule;
                    self.ws.clear();
                    self.ws.one(&r);
                },
                Err(_) => panic!("Error compiling rule expression."),
            }
            */
            /*
            const result = this._grammer.scan(expr);
            
            // TODO Show nice errors.
            if (!result.isSuccess)
                throw new Error("Error compiling rule expression.");
                
            */
            //self.ws.clear();
            /*
            this._ws.one(result.branches[0].rule);
            */
        }
        
        /*
        private addRange(rule: Rule<TBranch, TMeta>, context: IParseContext<TBranch, TBranch>): Rule<TBranch, TMeta>
        {
            switch (context.rangeType)
            {
                case RangeType.AtLeast:
                    return new Rule<TBranch, TMeta>().atLeast(context.arg1, rule);
                
                case RangeType.AtMost:
                    return new Rule<TBranch, TMeta>().atMost(context.arg1, rule);
                
                case RangeType.Between:
                    return new Rule<TBranch, TMeta>().between(context.arg1, context.arg2, rule);
                
                case RangeType.Exact:
                    return new Rule<TBranch, TMeta>().exact(context.arg1, rule);
                
                case RangeType.NoRangeType:
                    return rule;
                
                default:
                    throw new Error("Not implemented.");
            }
        }
        */
    }*/
}

#[cfg(test)]
mod tests 
{
    use abitvin::Rule;
    use std::rc::Rc;
    
    #[test]
    fn all()
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
    fn all_except()
    {
        let code = "abc";
        
        let f = |_: &Vec<u32>, l: &str| {
            assert_eq!(l, "abc");
            vec![0u32, 1u32, 2u32, 3u32]
        };
        
        let mut c: Rule<u32> = Rule::new(None);
        c.all_except(vec!['A', 'B', 'C', 'D']);
        
        let mut r: Rule<u32> = Rule::new(Some(Box::new(f)));
        r.exact_owned(3, c);
        
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
    fn alter()
    {
        let code = "aaabbbccc";
        
        let alterations = vec![
            ("aaa", "AAA"),
            ("bbb", "BBB"),
            ("ccc", "CCC"),
        ];
        
        let mut a: Rule<i32> = Rule::new(None);
        a.alter(alterations);

        let f = |_: &Vec<i32>, l: &str| {
            assert_eq!(l, "AAABBBCCC");
            vec![111, 222]
        }; 
        
        let mut r: Rule<i32> = Rule::new(Some(Box::new(f)));
        r.exact_owned(3, a);
        
        if let Ok(branches) = r.scan(&code) {
            assert_eq!(branches[0], 111);
            assert_eq!(branches[1], 222);
        }
        else {
            assert!(false);
        }
    }
    
    #[test]
    fn any_of()
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
        
        let mut any_of_these: Rule<i32> = Rule::new(None);
        any_of_these.any_of_owned(vec![aaa, bbb, ccc]);
        
        let mut root: Rule<i32> = Rule::new(None);
        root.exact_owned(3, any_of_these);
        
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
    fn at_least()
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
    fn at_most()
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
    fn between()
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
    fn char_in()
    {
        let mut digit: Rule<u32> = Rule::new(Some(Box::new(|_, l| vec![(l.chars().next().unwrap() as u32) - 48])));
        digit.char_in('0', '9');
        
        let mut af: Rule<u32> = Rule::new(Some(Box::new(|_, l| vec![(l.chars().next().unwrap() as u32) - 55])));
        af.char_in('A', 'F');

        let mut hex: Rule<u32> = Rule::new(None);
        hex.any_of_owned(vec![digit, af]);
                
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
        parse.between_owned(1, 8, hex);
        
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
    fn clear()
    {
        let code = "Ello'";
        
        let mut r: Rule<char> = Rule::new(None);
        r.literal("Ello'");
        r.clear();
        r.scan(&code);   // Panic! We cleared the rule.
    }
    
    #[test]
    fn eof()
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
    fn exact()
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
    fn literal()
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
    fn maybe()
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
        root.maybe(&dots).one_owned(xxx).maybe(&dots);
        
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
    fn none_or_many()
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
    fn not()
    {
        let mut not_this: Rule<i32> = Rule::new(None);
        not_this.literal("not this");
        
        let mut r: Rule<i32> = Rule::new(None);
        r.literal("aaa").not_owned(not_this).literal("bbb").literal("ccc");
        
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
    fn one()
    {
        let code = "onetwothree";
        
        let mut one: Rule<i32> = Rule::new(Some(Box::new(|_, _| vec![1] )));
        one.literal("one");
        
        let mut two: Rule<i32> = Rule::new(Some(Box::new(|_, _| vec![2] )));
        two.literal("two");
        
        let mut three: Rule<i32> = Rule::new(Some(Box::new(|_, _| vec![3] )));
        three.literal("three");
        
        let mut root: Rule<i32> = Rule::new(None);
        root.one_owned(one).one_owned(two).one_owned(three);
        
        if let Ok(branches) = root.scan(&code) {
            assert_eq!(branches[0], 1);
            assert_eq!(branches[1], 2);
            assert_eq!(branches[2], 3);
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn misc_calc()
    {
        /*
        const calc = new Grammer<number, IEmpty>();
        calc.declare("add", "expr", "mul");
        calc.add("num", "[0-9]+", (b, l) => parseInt(l));
        calc.add("brackets", "\\(<expr>\\)");   // Identity function
        calc.add("mul", "(<num>|<brackets>)(\\*<mul>)?", b => b.length === 1 ? b : b[0] * b[1]);
        calc.add("add", "<mul>(\\+<add>)?", b => b.length === 1 ? b : b[0] + b[1]);
        calc.add("expr", "(<add>|<brackets>)");
        
        console.log(calc.scan("expr", "2*(3*4*5)")); // 120
        console.log(calc.scan("expr", "2*(3+4)*5")); // 70
        console.log(calc.scan("expr", "((2+3*4+5))")); // 19
        */

        /*
        // Predeclare add, expr and mul.
        let mut add: Rule<f64> = Rule::new(Some(Box::new(|b, _| if b.len() == 1 { vec![b[0]] } else { vec![b[0] + b[1]] } )));
        let mut expr: Rule<f64> = Rule::new(None);
        let mut mul: Rule<f64> = Rule::new(Some(Box::new(|b, _| if b.len() == 1 { vec![b[0]] } else { vec![b[0] * b[1]] } )));

        let mut digit: Rule<f64> = Rule::new(None);
        digit.char_in('0', '9');

        let mut num: Rule<f64> = Rule::new(Some(Box::new(|_, l| vec![l.parse().unwrap()])));
        num.at_least(1, &digit);

        let mut brackets: Rule<f64> = Rule::new(None);
        brackets.literal("(").one(&expr).literal(")");

        let mut mul_right: Rule<f64> = Rule::new(None);
        mul_right.literal("*").one(&mul);
        mul.any_of(vec![&num, &brackets]).maybe(&mul_right);

        let mut add_right: Rule<f64> = Rule::new(None);
        add_right.literal("+").one(&add);
        add.one(&mul).maybe(&add_right);

        expr.any_of(vec![&add, &brackets]);

        if let Ok(branches) = expr.scan("2*(3*4*5)") {
            assert_eq!(branches[0], 120f64);
        }
        else {
            assert!(false);
        }

        if let Ok(branches) = expr.scan("2*(3+4)*5") {
            assert_eq!(branches[0], 70f64);
        }
        else {
            assert!(false);
        }

        if let Ok(branches) = expr.scan("((2+3*4+5))") {
            assert_eq!(branches[0], 19f64);
        }
        else {
            assert!(false);
        }
        */

        /*
        let expr_index_1: usize = 0;

        let mut digit: Rule<i32> = Rule::new(None);
        digit.char_in('0', '9');
        
        let mut num: Rule<i32> = Rule::new(None);
        num.at_least(1, &digit.into_rc());

        let mut brackets_expr_index: usize = 0;
        let mut brackets: Rule<i32> = Rule::new(None);
        brackets.literal("(").placeholder(&brackets_expr_index).literal(")");
        let mut brackets = brackets.into_rc();

        let mut mul_num_index: usize = 0;
        let mut mul_mul_index: usize = 0;
        let mut mul: Rule<i32> = Rule::new(None);
        //mul.any_of(vec![])
        */

        //let mut aaa_index: usize = 0;
        //let mut aaa: Rule<i32> = Rule::new(None);
        //aaa.literal("bla").placeholder(&mut aaa_index).literal("bla");
        //let aaa_rc = aaa.into_rc();

        //let mut bbb: Rule<i32> = Rule::new(None);
        //aaa.at(aaa_index, &bbb.into_rc());

        //let xxx = Rc::new(Cell::new(123u64));
        //xxx.borrow_mut().set(1234u64);

        //let xxx = Cell::new(123u64);
        //xxx.set(1234u64);

        //let yyy = Rc::new(Cell::new(123u64));
        //yyy.set(1234u64);

        //let mut aaa_index: usize = 0;
        //let mut aaa: Rule<i32> = Rule::new(None);
        //aaa.literal("bla").placeholder(&mut aaa_index).literal("bla");

        //let aaa = Rc::new(RefCell::new(aaa));
        //aaa.borrow_mut().literal("asdasd");

        //let aaa_rc = aaa.into_rc();
    }
}

/*


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

*/