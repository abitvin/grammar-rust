// Copyright (c) 2015-2016 Abitvin <foss@abitvin.net>
// Licensed under the MIT license <LICENSE.md or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

pub mod abitvin
{
    use std::str::Chars;

    // TODO What is the best way to store the branch closure?
    // http://stackoverflow.com/questions/27831944/how-do-i-store-a-closure-in-rust
    // TODO Simplify `Option<Box<Fn<...>>>` if we can.
    pub type BranchFn<T, S> = Option<Box<Fn(&Vec<T>, &str, &mut S) -> Vec<T>>>;
    
    // TODO Better name
    enum Progress<'a, T: 'a> {
        Some(usize, ScanCtx<'a, T>),
        No(ScanCtx<'a, T>), // TODO We can do without the ScanCtx but we need to clone the errors.
    }

    pub struct Rule<'a, T: 'a, S: 'a> {
        branch_fn: BranchFn<T, S>,
        parts: Vec<ScanFn<'a, T, S>>,
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
    
    enum ScanFn<'a, T: 'a, S: 'a> {
        All,
        AllExcept(Vec<char>),
        Alter(Vec<(&'static str, &'static str)>),
        AnyOf(Vec<&'a Rule<'a, T, S>>),
        AnyOfOwned(Vec<Rule<'a, T, S>>),
        AnyOfRaw(Vec<*const Rule<'a, T, S>>),
        CharIn(char, char),
        Eof,
        Literal(&'static str),
        LiteralString(String),
        Not(&'a Rule<'a, T, S>),
        NotOwned(Rule<'a, T, S>),
        NotRaw(*const Rule<'a, T, S>),
        Range(u64, u64, &'a Rule<'a, T, S>),
        RangeOwned(u64, u64, Rule<'a, T, S>),
        RangeRaw(u64, u64, *const Rule<'a, T, S>),
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

    impl<'a, T, S> Rule<'a, T, S>
    {
        pub fn new(branch_fn: BranchFn<T, S>) -> Self
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
        
        pub fn any_of(&mut self, rules: Vec<&'a Rule<'a, T, S>>) -> &mut Self
        {
            if rules.len() == 0 {
                panic!("You must specify rules.");
            }
            
            self.parts.push(ScanFn::AnyOf(rules));
            self
        }

        pub fn any_of_owned(&mut self, rules: Vec<Rule<'a, T, S>>) -> &mut Self
        {
            if rules.len() == 0 {
                panic!("You must specify rules.");
            }
            
            self.parts.push(ScanFn::AnyOfOwned(rules));
            self
        }

        pub unsafe fn any_of_raw(&mut self, rules: Vec<*const Rule<'a, T, S>>) -> &mut Self
        {
            if rules.len() == 0 {
                panic!("You must specify rules.");
            }
            
            self.parts.push(ScanFn::AnyOfRaw(rules));
            self
        }
        
        pub fn at_least(&mut self, count: u64, rule: &'a Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(count, u64::max_value(), &rule));
            self
        }

        pub fn at_least_owned(&mut self, count: u64, rule: Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(count, u64::max_value(), rule));
            self
        }
        
        pub unsafe fn at_least_raw(&mut self, count: u64, rule: *const Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeRaw(count, u64::max_value(), rule));
            self
        }
        
        pub fn at_most(&mut self, count: u64, rule: &'a Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(0, count, &rule));
            self
        }
        
        pub fn at_most_owned(&mut self, count: u64, rule: Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(0, count, rule));
            self
        }
        
        pub unsafe fn at_most_raw(&mut self, count: u64, rule: *const Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeRaw(0, count, rule));
            self
        }
        
        pub fn between(&mut self, min: u64, max: u64, rule: &'a Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(min, max, &rule));
            self
        }
        
        pub fn between_owned(&mut self, min: u64, max: u64, rule: Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(min, max, rule));
            self
        }
        
        pub unsafe fn between_raw(&mut self, min: u64, max: u64, rule: *const Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeRaw(min, max, rule));
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
        
        pub fn exact(&mut self, count: u64, rule: &'a Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(count, count, &rule));
            self
        }

        pub fn exact_owned(&mut self, count: u64, rule: Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(count, count, rule));
            self
        }

        pub unsafe fn exact_raw(&mut self, count: u64, rule: *const Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeRaw(count, count, rule));
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

        pub fn literal_string(&mut self, text: String) -> &mut Self
        {
            if text.len() < 1 {
                panic!("Literal text must at least 1 character long.");
            }
                
            self.parts.push(ScanFn::LiteralString(text));
            self
        }

        pub fn maybe(&mut self, rule: &'a Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(0, 1, &rule));
            self
        }
        
        pub fn maybe_owned(&mut self, rule: Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(0, 1, rule));
            self
        }
        
        pub unsafe fn maybe_raw(&mut self, rule: *const Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeRaw(0, 1, rule));
            self
        }
        
        pub fn none_or_many(&mut self, rule: &'a Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(0, u64::max_value(), &rule));
            self
        }
        
        pub fn none_or_many_owned(&mut self, rule: Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(0, u64::max_value(), rule));
            self
        }
        
        pub unsafe fn none_or_many_raw(&mut self, rule: *const Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeRaw(0, u64::max_value(), rule));
            self
        }
        
        pub fn not(&mut self, rule: &'a Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::Not(&rule));
            self
        }
        
        pub fn not_owned(&mut self, rule: Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::NotOwned(rule));
            self
        }

        pub unsafe fn not_raw(&mut self, rule: *const Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::NotRaw(rule));
            self
        }
        
        pub fn one(&mut self, rule: &'a Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::Range(1, 1, &rule));
            self
        }
        
        pub fn one_owned(&mut self, rule: Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(1, 1, rule));
            self
        }

        pub unsafe fn one_raw(&mut self, rule: *const Rule<'a, T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeRaw(1, 1, rule));
            self
        }

        // TODO I think `code` needs to be a String 
        pub fn scan(&self, code: &'static str, mut shared: &mut S) -> Result<Vec<T>, Vec<RuleError>>
        {
            let mut ctx = ScanCtx {
                branches: Vec::new(),
                code_iter: code.chars(),
                errors: Vec::new(),
                index: 0,
                lexeme: String::new(),
            };
            
            match self.run(ctx, &mut shared) {
                Progress::Some(_, new_ctx) => ctx = new_ctx,
                Progress::No(new_ctx) => return Err(new_ctx.errors),
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

        #[allow(unused_variables)]   // TODO This warning is for `is_root_of_rule` which we must implement.
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
        
        fn merge<'b>(&self, mut target: ScanCtx<'b, T>, mut source: ScanCtx<'b, T>, is_root_of_rule: bool, mut state: &mut S) -> Progress<'b, T>
        {
            /* TODO
            if (isRootOfRule)
                while (source.metaPushed-- > 0)
                    source.trail.pop();
            */
                
            let step = source.index - target.index;
                
            target.code_iter = source.code_iter;
            target.errors = source.errors;
            target.index = source.index;
            target.lexeme.push_str(&source.lexeme.to_string());
            // TODO target.metaPushed = 0;
            // TODO target.trail = source.trail;
            
            match self.branch_fn {
                Some(ref f) if is_root_of_rule => {
                    target.branches.append(&mut f(&source.branches, &source.lexeme, &mut state));
                },
                _ => {
                    target.branches.append(&mut source.branches);
                }
            }
                
            Progress::Some(step as usize, target)
        }
        
        fn run<'b>(&'b self, ctx: ScanCtx<'b, T>, mut shared: &mut S) -> Progress<T>
        {
            if self.parts.len() == 0 {
                panic!("Rule is not defined.");
            }
            
            let mut new_ctx = self.branch(&ctx, true);
            
            for p in &self.parts {
                let progress = match *p {
                    ScanFn::All => self.scan_all_leaf(new_ctx),
                    ScanFn::AllExcept(ref exclude) => self.scan_all_except_leaf(&exclude, new_ctx),
                    ScanFn::Alter(ref alter) => self.scan_alter_leaf(&alter, new_ctx),
                    ScanFn::AnyOf(ref rules) => self.scan_any_of(rules, new_ctx, &mut shared),
                    ScanFn::AnyOfOwned(ref rules) => self.scan_any_of_owned(rules, new_ctx, &mut shared),
                    ScanFn::AnyOfRaw(ref rules) => self.scan_any_of_raw(rules, new_ctx, &mut shared),
                    ScanFn::CharIn(min, max) => self.scan_char_in_leaf(min, max, new_ctx),
                    ScanFn::Eof => self.scan_eof_leaf(new_ctx),
                    ScanFn::Literal(find) => self.scan_literal_leaf(&find, new_ctx),
                    ScanFn::LiteralString(ref text) => self.scan_literal_leaf(&text, new_ctx),
                    ScanFn::Not(r) => self.scan_not(r as *const Rule<T, S>, new_ctx, &mut shared),
                    ScanFn::NotOwned(ref r) => self.scan_not(r as *const Rule<T, S>, new_ctx, &mut shared),
                    ScanFn::NotRaw(r) => self.scan_not(r, new_ctx, &mut shared),
                    ScanFn::Range(min, max, r) => self.scan_rule_range(min, max, r as *const Rule<T, S>, new_ctx, &mut shared),
                    ScanFn::RangeOwned(min, max, ref r) => self.scan_rule_range(min, max, r as *const Rule<T, S>, new_ctx, &mut shared),
                    ScanFn::RangeRaw(min, max, r) => self.scan_rule_range(min, max, r, new_ctx, &mut shared),
                };
                
                match progress {
                    Progress::Some(_, newer_ctx) => new_ctx = newer_ctx,
                    Progress::No(_) => return Progress::No(ctx),
                }
            }
            
            self.merge(ctx, new_ctx, true, &mut shared)
        }

        // TODO What about a char with more codepoints?
        fn scan_all_except_leaf<'b>(&'b self, exclude: &Vec<char>, mut ctx: ScanCtx<'b, T>) -> Progress<T>
        {
            let n = ctx.code_iter.next();
            
            if let Some(c) = n {
                if exclude.contains(&c) {
                    return self.update_error(ctx, format!("Character '{}' is not allowed here.", c));
                }
                
                ctx.lexeme.push(c);
                ctx.index += 1;
                Progress::Some(1, ctx)
            } 
            else {
                self.update_error(ctx, String::from("End of code while checking for not allowed character."))
            }
        }

        // TODO What about a char with more codepoints?
        fn scan_all_leaf<'b>(&'b self, mut ctx: ScanCtx<'b, T>) -> Progress<T> 
        {
            let n = ctx.code_iter.next();
            
            if let Some(c) = n {
                ctx.lexeme.push(c);
                ctx.index += 1;
                Progress::Some(1, ctx)
            } 
            else {
                self.update_error(ctx, String::from("End of code while checking for not allowed character."))
            }
        }
        
        fn scan_alter_leaf<'b>(&'b self, list: &Vec<(&'static str, &'static str)>, mut ctx: ScanCtx<'b, T>) -> Progress<T>
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
                    return Progress::Some(len as usize, ctx);
                }
            }
            
            self.update_error(ctx, String::from("Alter characters not found on this position."))
        }
        
        fn scan_any_of<'b>(&'b self, rules: &Vec<&'a Rule<T, S>>, mut ctx: ScanCtx<'b, T>, mut state: &mut S) -> Progress<T>
        {
            for r in rules {
                let mut new_ctx = self.branch(&ctx, false);

                if let Progress::Some(progress, new_ctx) = unsafe { (*r).run(new_ctx, &mut state) } {
                    return self.merge(ctx, new_ctx, false, &mut state);
                } 
            }

            Progress::No(ctx)
        }
        
        fn scan_any_of_owned<'b>(&'b self, rules: &'b Vec<Rule<T, S>>, mut ctx: ScanCtx<'b, T>, mut state: &mut S) -> Progress<T>
        {
            for r in rules {
                let mut new_ctx = self.branch(&ctx, false);

                if let Progress::Some(progress, new_ctx) = unsafe { r.run(new_ctx, &mut state) } {
                    return self.merge(ctx, new_ctx, false, &mut state);
                } 
            }

            Progress::No(ctx)
        }

        fn scan_any_of_raw<'b>(&'b self, rules: &Vec<*const Rule<'b, T, S>>, mut ctx: ScanCtx<'b, T>, mut state: &mut S) -> Progress<T>
        {
            for r in rules {
                let mut new_ctx = self.branch(&ctx, false);

                if let Progress::Some(progress, new_ctx) = unsafe { (**r).run(new_ctx, &mut state) } {
                    return self.merge(ctx, new_ctx, false, &mut state);
                } 
            }

            Progress::No(ctx)
        }
        
        fn scan_char_in_leaf<'b>(&'b self, min: char, max: char, mut ctx: ScanCtx<'b, T>) -> Progress<T>
        {
            let c = ctx.code_iter.next();
            
            match c {
                Some(c) => {
                    if c < min || c > max {
                       self.update_error(ctx, format!("Expected a character between '{}' and '{}'; got a {}.", min, max, c))
                    }
                    else {
                        ctx.lexeme.push(c);
                        ctx.index += 1;
                        Progress::Some(1, ctx)
                    }
                },
                None => {
                    self.update_error(ctx, format!("End of code. Expected a character between '{}' and '{}'.", min, max))
                }
            }
        }
        
        fn scan_eof_leaf<'b>(&'b self, mut ctx: ScanCtx<'b, T>) -> Progress<T>
        {
            if let None = ctx.code_iter.next() {
                ctx.index += 1;
                Progress::Some(1, ctx)
            }
            else {
                self.update_error(ctx, String::from("No EOF on this position."))
            }
        }
        
        fn scan_literal_leaf<'b>(&'b self, find: &str, mut ctx: ScanCtx<'b, T>) -> Progress<T> 
        {
            let iter = find.chars();
            let mut step = 0;
                
            for i in iter {
                let n = ctx.code_iter.next();
                    
                if let Some(c) = n {
                    if i != c {
                        return self.update_error(ctx, format!("The literal '{}' not found.", find));
                    }
                        
                    ctx.index += 1;
                    step += 1;
                }
                else {
                    return self.update_error(ctx, format!("End of code. The literal '{}' not found.", find));
                }
            }
                
            ctx.lexeme.push_str(find);
            Progress::Some(step as usize, ctx)
        }
        
        fn scan_not<'b>(&'b self, rule: *const Rule<'b, T, S>, mut ctx: ScanCtx<'b, T>, mut state: &mut S) -> Progress<T>
        {
            match unsafe { (*rule).run(self.branch(&ctx, false), &mut state) } {
                Progress::Some(_, _) => Progress::No(ctx),
                Progress::No(_) => Progress::Some(0, ctx),
            }
        }
        
        fn scan_rule_range<'b>(&'b self, min: u64, max: u64, rule: *const Rule<'a, T, S>, ctx: ScanCtx<'b, T>, mut state: &mut S) -> Progress<T>
        {
            let mut new_ctx = self.branch(&ctx, false);
            let mut count = 0u64;
            
            loop {
                match unsafe { (*rule).run(new_ctx, &mut state) } {
                    Progress::Some(progress, ctx) => {
                        if progress == 0 {
                            return Progress::Some(0, ctx);
                        }

                        new_ctx = ctx;
                        count += 1;

                        if count == max {
                            break;
                        }
                    },
                    Progress::No(ctx) => {
                        new_ctx = ctx;
                        break;
                    }
                }
            }

            if count >= min && count <= max {
                self.merge(ctx, new_ctx, false, &mut state)
            }
            else {
                Progress::No(ctx)
            }
        }

        fn update_error<'b>(&'b self, mut ctx: ScanCtx<'b, T>, error_msg: String) -> Progress<T>
        {
            if ctx.errors.len() != 0 {
                let err_idx = ctx.errors[0].index;
                    
                if ctx.index < err_idx {
                    return Progress::No(ctx);
                }
                    
                if ctx.index > err_idx {
                    ctx.errors.clear();
                }
            }
                
            ctx.errors.push(RuleError {
                index: ctx.index,
                msg: error_msg,
                // TODO trail: newCtx.trail.slice(0)
            });
                
            Progress::No(ctx)
        }
    }

    /////////////
    // Grammer //
    /////////////
    
    // TODO Put Grammer in its own file/container.
    
    use std::collections::BTreeMap;
    
    enum RangeType {
        NoRangeType = 0,
        AtLeast,
        AtMost,
        Between,
        Exact,
        Not
    }
    
    pub struct NoShared {} 

    struct ParseContext<'a, T: 'a> {
        arg1: u64,
        arg2: u64,
        arg3: Option<String>,
        range_type: RangeType,
        rule: Option<Box<Rule<'a, T, NoShared>>>,
    }

    type RuleExprMap<'a, T> = BTreeMap<&'static str, RuleExpr<'a, T>>;

    // TODO TMeta class R<TB, TM> extends Rule<IParseContext<TB, TM>, IEmpty> {}
    type R<'a, T> = Rule<'a, ParseContext<'a, T>, RuleExprMap<'a, T>>;
    
    // TODO Note: This was IRule, remove this comment later after porting.
    struct RuleExpr<'a, T: 'a> {
        id: &'static str,
        is_defined: bool,
        rule: Rule<'a, T, NoShared>
    }
    
    pub struct Grammer<'a, T: 'a> /* <TBranch, TMeta> */
    {
        grammer: R<'a, T>,
        rule_exps: RuleExprMap<'a, T>,
        ws: Rule<'a, T, NoShared>,
    }

    impl<'a, T> Grammer<'a, T>
    {
        pub fn new() -> Self
        {
            let rule_exps = RuleExprMap::new();

            let mut space = Rule::new(None); space.literal(" ");
            let mut tab = Rule::new(None); space.literal("\t");
            let mut new_line = Rule::new(None); space.literal("\n");
            let mut carriage_return = Rule::new(None); space.literal("\r");
            let mut ws = Rule::new(None); ws.any_of_owned(vec![space, tab, new_line, carriage_return]);

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
            */

            let statement_fn = |b: &Vec<ParseContext<T>>, _: &str, _: &mut RuleExprMap<T>|
            {   
                // TODO
                panic!("TODO");

                /*match b[0].range_type {
                    RangeType::Not => {
                        b
                    },
                    _ => {
                        let mut r: Rule<'a, T, NoShared> = Rule::new(None);
                        r.not_owned(b[1].rule.unwrap().into_raw());

                        vec![ParseContext{ 
                            arg1: 0,
                            arg2: 0,
                            arg3: None,
                            range_type: RangeType::NoRangeType,
                            rule: Some(Box::new(r)) 
                        }]
                    }
                } */
                
                /*
                if b[0].range_type != RangeType::Not {
                    return b;
                }
                    
                let mut r = Rule::new();
                r.not(b[1].rule);

                vec![ParseContext{ 
                    arg1: 0,
                    arg2: 0,
                    arg3: None,
                    range_type: RangeType::NoRangeType,
                    rule: Some(Box::new(r)) 
                }]
                */
            };

            // TODO let statement = R::new(Some(Box(statement_fn)));
            let statement = R::new(None);

            let mut escaped_ctrl_chars: R<'a, T> = Rule::new(None);
            escaped_ctrl_chars.alter(vec![
                ("\\<", "<"), 
                ("\\>", ">"), 
                ("\\{", "{"),
                ("\\}", "}"), 
                ("\\(", "("), 
                ("\\)", ")"), 
                ("\\[", "["), 
                ("\\]", "]"), 
                ("\\^", "^"),
                ("\\~", "~"),
                ("\\-", "-"),
                ("\\,", ","),
                ("\\|", "|"),
                ("\\+", "+"), 
                ("\\?", "?"), 
                ("\\*", "*"), 
                ("\\.", "."), 
                ("\\$", "$"),
                ("\\ ", " "), 
                ("\\_", "_"),
                ("\\!", "!"),
            ]);

            /*
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
            */
            
            let literal_text_fn = |_: &Vec<ParseContext<T>>, l: &str, _: &mut RuleExprMap<T>|
            {
                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: Some(String::from(l)),
                    range_type: RangeType::NoRangeType,
                    rule: None, 
                }]
            };

            let mut literal_all_except: R<'a, T> = Rule::new(None);
            literal_all_except.all_except(vec!['<', '{', '(', ')', '|', '[', '+', '?', '*', '.', '$', ' ', '_', '!']);

            let mut literal_char: R<'a, T> = Rule::new(None);
            literal_char.any_of_owned(vec![escaped_ctrl_chars, literal_all_except]);
            
            let mut literal_text: R<'a, T> = Rule::new(Some(Box::new(literal_text_fn)));
            literal_text.at_least_owned(1, literal_char);

            let literal_fn = |b: &Vec<ParseContext<T>>, l: &str, _: &mut RuleExprMap<T>|
            {
                let mut rule = Rule::new(None);
                rule.literal_string(b[0].arg3.clone().unwrap());
                
                if b.len() == 2 {
                    rule = Grammer::add_range(rule, &b[1]);
                }
                
                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: None,
                    range_type: RangeType::NoRangeType,
                    rule: Some(Box::new(rule)),
                }]
            };

            let mut literal: R<'a, T> = Rule::new(Some(Box::new(literal_fn)));
            literal.one_owned(literal_text); // TODO .maybe(&ranges);

            /*
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
            */
            
            let mut grammer = R::new(None);
            grammer.none_or_many_owned(statement);
            
            Grammer {
                grammer: grammer, 
                rule_exps: rule_exps,
                ws: ws,
            }
        }
        
        pub fn add(&mut self, id: &'static str, expr: &'static str, branch_fn: BranchFn<T, NoShared>)
        {
            {
                let rulexp = self.rule_exps.get(id);

                if rulexp.is_some() && rulexp.unwrap().is_defined {
                    panic!("The rule \"{}\" already used.", id);    // TODO Return nice error
                }
            }

            let mut result = self.grammer.scan(&expr, &mut self.rule_exps); 

            match result {
                Err(_) => {
                    panic!("Error compiling rule expression.");    // TODO Return nice error
                },
                Ok(mut branches) => {
                    let new_ruleexp = match self.rule_exps.get_mut(id) {
                        Some(rulexp) => {
                            rulexp.is_defined = true;
                            rulexp.rule.branch_fn = branch_fn;
                            // TODO rulexp.rule.meta = meta;

                            for r in branches {
                                rulexp.rule.one_owned(*r.rule.unwrap());
                            }

                            None
                        }
                        None => {
                            // TODO because I couldn't find a way to pop the first item of an vec.
                            // I reversed the vec two times, I hope we can do this better.

                            let mut reversed = Vec::new();

                            loop {
                                if let Some(r) = branches.pop() {
                                    reversed.push(r);
                                }
                                else {
                                    break;
                                }
                            }

                            let mut compiled = *reversed.pop().unwrap().rule.unwrap();
                            let mut reversed_again = Vec::new();

                            loop {
                                if let Some(r) = reversed.pop() {
                                    reversed_again.push(r);
                                }
                                else {
                                    break;
                                }
                            }

                            compiled.branch_fn = branch_fn;
                            // TODO compiled.meta = meta;

                            for r in reversed_again {
                                compiled.one_owned(*r.rule.unwrap());
                            }

                            Some(RuleExpr {
                                id: id,
                                is_defined: true,
                                rule: compiled,
                            })
                        }
                    };

                    if let Some(r) = new_ruleexp {
                        self.rule_exps.insert(id, r);
                    }
                },
            }
        }

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
            let mut dummy = NoShared {};

            match self.rule_exps.get(root_id) {
                Some(r) => r.rule.scan(code, &mut dummy),
                None => panic!("Rule with id \"{}\" not found.", root_id),
            }
        }
        
        pub fn ws(&mut self, expr: &'static str) 
        {
            match self.grammer.scan(expr, &mut self.rule_exps) {
                Ok(mut b) => {
                    if b.len() != 1 {
                        panic!("Error compiling rule expression.");
                    }
                    
                    let r = *b.pop().unwrap().rule.unwrap();
                    self.ws.clear().one_owned(r);
                },
                Err(_) => panic!("Error compiling rule expression."),
            }
        }
        
        fn add_range<'b>(rule: Rule<'b, T, NoShared>, ctx: &ParseContext<T>) -> Rule<'b, T, NoShared>
        {
            match ctx.range_type {
                RangeType::AtLeast => {
                    let mut r = Rule::new(None);
                    r.at_least_owned(ctx.arg1, rule);
                    r
                }
                _ => panic!("Not implemented!"),
            }
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
    }
}

#[cfg(test)]
mod tests 
{
    use abitvin::Rule;

    struct NoShared {}

    #[test]
    fn all()
    {
        let mut dummy = 80085; 
        let code = "abcdefg";
        
        let f = |_: &Vec<bool>, l: &str, _: &mut i32| {
            assert_eq!(l, "abcdefg");
            vec![true, false, false, true]
        };
        
        let mut r: Rule<bool, i32> = Rule::new(Some(Box::new(f)));
        r.all().all().all().all().all().all().all();
        
        if let Ok(branches) = r.scan(&code, &mut dummy) {
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
        let mut dummy = false; 
        let code = "abc";
        
        let f = |_: &Vec<u32>, l: &str, _: &mut bool| {
            assert_eq!(l, "abc");
            vec![0u32, 1u32, 2u32, 3u32]
        };
        
        let mut c = Rule::new(None);
        c.all_except(vec!['A', 'B', 'C', 'D']);
        
        let mut r: Rule<u32, bool> = Rule::new(Some(Box::new(f)));
        r.exact_owned(3, c);
        
        if let Ok(branches) = r.scan(&code, &mut dummy) {
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
        let mut dummy = 0f64;
        let code = "aaabbbccc";
        
        let alterations = vec![
            ("aaa", "AAA"),
            ("bbb", "BBB"),
            ("ccc", "CCC"),
        ];
        
        let mut a = Rule::new(None);
        a.alter(alterations);

        let f = |_: &Vec<i32>, l: &str, _: &mut f64| {
            assert_eq!(l, "AAABBBCCC");
            vec![111, 222]
        }; 
        
        let mut r: Rule<i32, f64> = Rule::new(Some(Box::new(f)));
        r.exact_owned(3, a);
        
        if let Ok(branches) = r.scan(&code, &mut dummy) {
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
        let mut dummy = false;
        let code = "aaabbbccc";
        
        let aaa_fn = |_: &Vec<i32>, l: &str, _: &mut bool| {
            assert_eq!(l, "aaa");
            vec![111]
        }; 
        
        let bbb_fn = |_: &Vec<i32>, l: &str, _: &mut bool| {
            assert_eq!(l, "bbb");
            vec![222]
        };
        
        let ccc_fn = |_: &Vec<i32>, l: &str, _: &mut bool| {
            assert_eq!(l, "ccc");
            vec![333]
        };
        
        let mut aaa = Rule::new(Some(Box::new(aaa_fn)));
        aaa.literal("aaa");
        
        let mut bbb = Rule::new(Some(Box::new(bbb_fn)));
        bbb.literal("bbb");
        
        let mut ccc = Rule::new(Some(Box::new(ccc_fn)));
        ccc.literal("ccc");
        
        let mut any_of_these = Rule::new(None);
        any_of_these.any_of_owned(vec![aaa, bbb, ccc]);
        
        let mut root: Rule<i32, bool> = Rule::new(None);
        root.exact_owned(3, any_of_these);
        
        if let Ok(branches) = root.scan(&code, &mut dummy) {
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
        let mut dummy = false;
        let code = "xxxx";
        
        let mut x = Rule::new(Some(Box::new(|_, _, _| vec![10])));
        x.literal("x");
        
        let mut root: Rule<i32, bool> = Rule::new(None);
        
        if let Ok(branches) = root.at_least(3, &x).scan(&code, &mut dummy) {
            assert_eq!(branches[0], 10);
            assert_eq!(branches[1], 10);
            assert_eq!(branches[2], 10);
            assert_eq!(branches[3], 10);
        }
        else {
            assert!(false);
        }
        
        if let Ok(branches) = root.clear().at_least(4, &x).scan(&code, &mut dummy) {
            assert_eq!(branches[0], 10);
            assert_eq!(branches[1], 10);
            assert_eq!(branches[2], 10);
            assert_eq!(branches[3], 10);
        }
        else {
            assert!(false);
        }
        
        if let Ok(_) = root.clear().at_least(5, &x).scan(&code, &mut dummy) {
            assert!(false);
        }
        else {
            assert!(true);
        }
    }
    
    #[test]
    fn at_most()
    {
        let mut dummy = 1234;
        let code = "yyy";
        
        let mut y = Rule::new(Some(Box::new(|_, _, _| vec![14] )));
        y.literal("y");
               
        let mut root: Rule<i32, i32> = Rule::new(None);
        
        if let Ok(_) = root.at_most(2, &y).scan(&code, &mut dummy) {
            assert!(false);
        }
        else {
            assert!(true);
        }
        
        if let Ok(branches) = root.clear().at_most(3, &y).scan(&code, &mut dummy) {
            assert_eq!(branches[0], 14);
            assert_eq!(branches[1], 14);
            assert_eq!(branches[2], 14);
        }
        else {
            assert!(false);
        }
        
        if let Ok(branches) = root.clear().at_most(4, &y).scan(&code, &mut dummy) {
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
        let mut dummy = 1234;
        let code = "zzz";
        
        let mut z = Rule::new(Some(Box::new(|_, _, _| vec![34])));
        z.literal("z");
               
        let mut root: Rule<i32, i32> = Rule::new(None);
        
        if let Ok(branches) = root.between(1, 3, &z).scan(&code, &mut dummy) {
            assert_eq!(branches[0], 34);
            assert_eq!(branches[1], 34);
            assert_eq!(branches[2], 34);
        }
        else {
            assert!(true);
        }
        
        if let Ok(branches) = root.clear().between(0, 10, &z).scan(&code, &mut dummy) {
            assert_eq!(branches[0], 34);
            assert_eq!(branches[1], 34);
            assert_eq!(branches[2], 34);
        }
        else {
            assert!(false);
        }
        
        if let Ok(_) = root.clear().between(4, 5, &z).scan(&code, &mut dummy) {
            assert!(false);
        }
        else {
            assert!(true);
        }
    }
    
    #[test]
    fn char_in()
    {
        let mut dummy = 1234;

        let mut digit = Rule::new(Some(Box::new(|_, l, _| vec![(l.chars().next().unwrap() as u32) - 48])));
        digit.char_in('0', '9');
        
        let mut af = Rule::new(Some(Box::new(|_, l, _| vec![(l.chars().next().unwrap() as u32) - 55])));
        af.char_in('A', 'F');

        let mut hex = Rule::new(None);
        hex.any_of_owned(vec![digit, af]);

        let mut parser: Rule<u32, i32> = Rule::new(Some(Box::new(|b, _, _| 
        {
            println!("parser len: {}", b.len());

            let mut m = 1u32;
            let mut n = 0u32;
            
            for i in b.iter().rev() {
                n += i * m;
                m <<= 4;
            }
            
            vec![n]
        })));
        parser.between_owned(1, 8, hex);
        
        if let Ok(branches) = parser.scan("A", &mut dummy) {
            assert_eq!(branches[0], 10);
        }
        else {
            assert!(false);
        }
        
        if let Ok(branches) = parser.scan("12345678", &mut dummy) {
            assert_eq!(branches[0], 305419896);
        }
        else {
            assert!(false);
        }
        
        if let Ok(branches) = parser.scan("FF", &mut dummy) {
            assert_eq!(branches[0], 255);
        }
        else {
            assert!(false);
        }
        
        if let Ok(branches) = parser.scan("FFFFFFFF", &mut dummy) {
            assert_eq!(branches[0], u32::max_value());
        }
        else {
            assert!(false);
        }
        
        if let Ok(_) = parser.scan("FFFFFFFFF", &mut dummy) {
            assert!(false);
        }
        else {
            assert!(true);
        }
        
        if let Ok(_) = parser.scan("FFxFF", &mut dummy) {
            assert!(false);
        }
        else {
            assert!(true);
        }
        
        if let Ok(_) = parser.scan("", &mut dummy) {
            assert!(false);
        }
        else {
            assert!(true);
        }
    }
    
    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn clear()
    {
        let mut dummy = 'X';
        let code = "Ello'";
        
        let mut r: Rule<char, char> = Rule::new(None);
        r.literal("Ello'");
        r.clear();
        r.scan(&code, &mut dummy);   // Panic! We cleared the rule.
    }
    
    #[test]
    fn eof()
    {
        let mut dummy = '@';
        let code = "123";
        
        let mut r: Rule<char, char> = Rule::new(Some(Box::new(|_, _, _| vec!['A', 'B'] )));
        r.literal("123").eof();
        
        if let Ok(branches) = r.scan(&code, &mut dummy) {
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
        let mut dummy = false;
        let code = "..........";
        
        let mut dot = Rule::new(Some(Box::new(|_, _, _| vec!['.'] )));
        dot.literal(".");
                
        let mut nope = Rule::new(Some(Box::new(|_, _, _| vec!['x'] )));
        nope.literal("nope");
                
        let mut root: Rule<char, bool> = Rule::new(None);
        
        if let Ok(branches) = root.exact(10, &dot).scan(&code, &mut dummy) {
            assert!(branches.len() == 10 && branches.into_iter().any(|c| c == '.'));
        }
        else {
            assert!(false);
        }
        
        if let Ok(_) = root.clear().exact(9, &dot).scan(&code, &mut dummy) {
            assert!(false);
        }
        else {
            assert!(true);
        }
        
        if let Ok(_) = root.clear().exact(11, &dot).scan(&code, &mut dummy) {
            assert!(false);
        }
        else {
            assert!(true);
        }
        
        if let Ok(branches) = root.clear().exact(0, &nope).exact(10, &dot).exact(0, &nope).scan(&code, &mut dummy) {
            assert!(branches.len() == 10 && branches.into_iter().any(|c| c == '.'));
        }
        else {
            assert!(false);
        }
    }
    
    #[test]
    fn literal()
    {
        let mut dummy = NoShared {};
        let code = "y̆y̆y̆x̆";
        
        let mut r: Rule<u64, NoShared> = Rule::new(Some(Box::new(|_, l, _| 
        {
            assert_eq!(l, "y̆y̆y̆x̆");
            vec![7777u64, 8888u64, 9999u64]
        })));
        
        r.literal("y̆y̆").literal("y̆").literal("x̆");
        
        if let Ok(branches) = r.scan(&code, &mut dummy) {
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
        let mut dummy = false;

        let codes = vec![
            "xxx",
            "...xxx",
            "xxx...",
            "...xxx...",
        ];
        
        let mut dots = Rule::new(None);
        dots.literal("...");
                
        let mut xxx = Rule::new(Some(Box::new(|_, _, _| vec!['x'] )));
        xxx.literal("xxx");
                
        let mut root: Rule<char, bool> = Rule::new(None);
        root.maybe(&dots).one_owned(xxx).maybe(&dots);
        
        for c in codes {
            if let Ok(branches) = root.scan(&c, &mut dummy) {
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
        let mut dummy = false;

        let mut dot = Rule::new(Some(Box::new(|_, _, _| vec![true])));
        dot.literal(".");
                
        let mut x = Rule::new(Some(Box::new(|_, _, _| vec![false])));
        x.literal("x");
                
        let mut code1: Rule<bool, bool> = Rule::new(Some(Box::new(|b, l, _|
        {
            assert_eq!(b.len(), 0);
            assert_eq!(l, "");
            Vec::new()
        })));
        
        let mut code2: Rule<bool, bool> = Rule::new(Some(Box::new(|b, l, _|
        {
            assert_eq!(b.len(), 1);
            assert_eq!(b[0], false);
            assert_eq!(l, "x");
            Vec::new()
        })));
        
        let mut code3: Rule<bool, bool> = Rule::new(Some(Box::new(|b, l, _|
        {
            assert_eq!(b.len(), 2);
            assert_eq!(b[0], true);
            assert_eq!(b[1], true);
            assert_eq!(l, "..");
            Vec::new()
        })));
        
        let mut code4: Rule<bool, bool> = Rule::new(Some(Box::new(|b, l, _|
        {
            assert_eq!(b.len(), 3);
            assert_eq!(b[0], false);
            assert_eq!(b[1], false);
            assert_eq!(b[2], true);
            assert_eq!(l, "xx.");
            Vec::new()
        })));
        
        let mut code5: Rule<bool, bool> = Rule::new(Some(Box::new(|b, l, _|
        {
            assert_eq!(b.len(), 4);
            assert_eq!(b[0], true);
            assert_eq!(b[1], true);
            assert_eq!(b[2], false);
            assert_eq!(b[3], false);
            assert_eq!(l, "..xx");
            Vec::new()
        })));
        
        if let Err(_) = code1.none_or_many(&dot).none_or_many(&x).none_or_many(&dot).scan("", &mut dummy) {
            assert!(false);
        }
        
        if let Err(_) = code2.none_or_many(&dot).none_or_many(&x).none_or_many(&dot).scan("x", &mut dummy) {
            assert!(false);
        }
        
        if let Err(_) = code3.none_or_many(&dot).none_or_many(&x).none_or_many(&dot).scan("..", &mut dummy) {
            assert!(false);
        }
        
        if let Err(_) = code4.none_or_many(&dot).none_or_many(&x).none_or_many(&dot).scan("xx.", &mut dummy) {
            assert!(false);
        }
        
        if let Err(_) = code5.none_or_many(&dot).none_or_many(&x).none_or_many(&dot).scan("..xx", &mut dummy) {
            assert!(false);
        }
    }
    
    #[test]
    fn not()
    {
        let mut dummy = 0;
        
        let mut not_this = Rule::new(None);
        not_this.literal("not this");
        
        let mut r: Rule<i32, i32> = Rule::new(None);
        r.literal("aaa").not_owned(not_this).literal("bbb").literal("ccc");
        
        if let Ok(_) = r.scan("aaabbbccc", &mut dummy) {
            assert!(true);
        }
        else {
            assert!(false);
        }
        
        if let Ok(_) = r.scan("aaanot thisbbbccc", &mut dummy) {
            assert!(false);
        }
        else {
            assert!(true);
        }
    }
    
    #[test]
    fn one()
    {
        let mut dummy = NoShared {};
        let code = "onetwothree";
        
        let mut one: Rule<i32, NoShared> = Rule::new(Some(Box::new(|_, _, _| vec![1] )));
        one.literal("one");
        
        let mut two: Rule<i32, NoShared> = Rule::new(Some(Box::new(|_, _, _| vec![2] )));
        two.literal("two");
        
        let mut three: Rule<i32, NoShared> = Rule::new(Some(Box::new(|_, _, _| vec![3] )));
        three.literal("three");
        
        let mut root: Rule<i32, NoShared> = Rule::new(None);
        root.one_owned(one).one_owned(two).one_owned(three);
        
        if let Ok(branches) = root.scan(&code, &mut dummy) {
            assert_eq!(branches[0], 1);
            assert_eq!(branches[1], 2);
            assert_eq!(branches[2], 3);
        }
        else {
            assert!(false);
        }
    }

    struct Shared {
        number: i32,
    }

    #[test]
    fn shared_state()
    {
        let mut shared = Shared { 
            number: 0 
        };

        let mut a: Rule<i32, Shared> = Rule::new(Some(Box::new(|_: &Vec<i32>, _: &str, s: &mut Shared| {
            s.number = 123; 
            Vec::new()
        })));
        a.literal("a");

        let mut b: Rule<i32, Shared> = Rule::new(Some(Box::new(|_: &Vec<i32>, _: &str, s: &mut Shared| {
            s.number = 456777; 
            Vec::new() 
        })));
        b.literal("b");
        
        let mut c: Rule<i32, Shared> = Rule::new(Some(Box::new(|_: &Vec<i32>, _: &str, s: &mut Shared| { 
            s.number = -999;
            Vec::new()
        })));
        c.literal("c");

        let mut failed: Rule<i32, Shared> = Rule::new(Some(Box::new(|_: &Vec<i32>, _: &str, s: &mut Shared| { 
            s.number = 123456;
            Vec::new()
        })));
        failed.literal("---");

        if let Ok(branches) = a.scan("a", &mut shared) {
            assert_eq!(shared.number, 123);
        }
        else {
            assert!(false);
        }

        if let Ok(branches) = b.scan("b", &mut shared) {
            assert_eq!(shared.number, 456777);
        }
        else {
            assert!(false);
        }

        if let Ok(branches) = c.scan("c", &mut shared) {
            assert_eq!(shared.number, -999);
        }
        else {
            assert!(false);
        }

        if let Ok(branches) = failed.scan("xxx", &mut shared) {
            assert!(false);
        }
        else {
            assert_eq!(shared.number, -999);
        }
    }
    
    #[test]
    fn misc_calc()
    {
        unsafe {
            let mut dummy = false;
            
            // Predeclare add, expr and mul.
            let mut expr: Rule<f64, bool> = Rule::new(None);
            let mut add = Rule::new(Some(Box::new(|b, _, _| if b.len() == 1 { vec![b[0]] } else { vec![b[0] + b[1]] } )));
            let mut mul = Rule::new(Some(Box::new(|b, _, _| if b.len() == 1 { vec![b[0]] } else { vec![b[0] * b[1]] } )));

            let mut digit = Rule::new(None);
            digit.char_in('0', '9');

            let mut num = Rule::new(Some(Box::new(|_, l, _| vec![l.parse().unwrap()])));
            num.at_least_owned(1, digit);

            let mut brackets = Rule::new(None);
            brackets.literal("(").one_raw(&expr).literal(")"); 

            let mut mul_right = Rule::new(None);
            mul_right.literal("*").one_raw(&mul);
            mul.any_of_raw(vec![&num, &brackets]).maybe_owned(mul_right);

            let mut add_right = Rule::new(None);
            add_right.literal("+").one_raw(&add);
            add.one_raw(&mul).maybe_owned(add_right);

            expr.any_of_raw(vec![&add, &brackets]);

            if let Ok(branches) = expr.scan("2*(3*4*5)", &mut dummy) {
                assert_eq!(branches[0], 120f64);
            }
            else {
                assert!(false);
            }

            if let Ok(branches) = expr.scan("2*(3+4)*5", &mut dummy) {
                assert_eq!(branches[0], 70f64);
            }
            else {
                assert!(false);
            }

            if let Ok(branches) = expr.scan("((2+3*4+5))", &mut dummy) {
                assert_eq!(branches[0], 19f64);
            }
            else {
                assert!(false);
            }
        }
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