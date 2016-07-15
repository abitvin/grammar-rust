// Copyright (c) 2015-2016 Abitvin <foss@abitvin.net>
// Licensed under the MIT license <LICENSE.md or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

pub mod abitvin
{
    use std::str::Chars;

    // TODO What is the best way to store the branch closure?
    // http://stackoverflow.com/questions/27831944/how-do-i-store-a-closure-in-rust
    // TODO Simplify `Option<Box<Fn<...>>>` if we can.
    pub type BranchFn<T, S> = Option<Box<Fn(Vec<T>, &str, &mut S) -> Vec<T>>>;
    
    // TODO Better name
    enum Progress<'b, T> {
        Some(usize, ScanCtx<'b, T>),
        No(ScanCtx<'b, T>), // TODO We can do without the ScanCtx but we need to clone the errors.
    }

    pub struct Rule<T, S> {
        branch_fn: BranchFn<T, S>,
        parts: Vec<ScanFn<T, S>>,
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
    
    enum ScanFn<T, S> {
        All,
        AllExcept(Vec<char>),
        Alter(Vec<(&'static str, &'static str)>),
        AlterString(Vec<(String, String)>),
        AnyOfOwned(Vec<Rule<T, S>>),
        AnyOfRaw(Vec<*const Rule<T, S>>),
        CharIn(char, char),
        Eof,
        Literal(&'static str),
        LiteralString(String),
        NotOwned(Rule<T, S>),
        NotRaw(*const Rule<T, S>),
        RangeOwned(u64, u64, Rule<T, S>),
        RangeRaw(u64, u64, *const Rule<T, S>),
    }

    impl<T, S> ScanFn<T, S> 
    {
        // TODO It's not really a shallow_clone...
        fn shallow_clone(&self) -> Self 
        {
            match *self {
                ScanFn::All => ScanFn::All,
                ScanFn::AllExcept(ref v) => ScanFn::AllExcept(v.clone()),
                ScanFn::Alter(ref v) => ScanFn::Alter(v.clone()),
                ScanFn::AlterString(ref v) => ScanFn::AlterString(v.clone()),
                ScanFn::AnyOfOwned(ref v) => ScanFn::AnyOfOwned(v.iter().map(|r| r.shallow_clone_b()).collect()),
                ScanFn::AnyOfRaw(ref v) => ScanFn::AnyOfRaw(v.clone()),
                ScanFn::CharIn(min, max) => ScanFn::CharIn(min, max),
                ScanFn::Eof => ScanFn::Eof,
                ScanFn::Literal(ref s) => ScanFn::Literal(s),
                ScanFn::LiteralString(ref s) => ScanFn::LiteralString(s.clone()),
                ScanFn::NotOwned(ref r) => ScanFn::NotOwned(r.shallow_clone_b()),
                ScanFn::NotRaw(r) => ScanFn::NotRaw(r),
                ScanFn::RangeOwned(min, max, ref rule) => ScanFn::RangeOwned(min, max, rule.shallow_clone_b()), 
                ScanFn::RangeRaw(min, max, rule) => ScanFn::RangeRaw(min, max, rule),
            }
        }
    }

    struct ScanCtx<'b, T> {
        branches: Vec<T>,
        code_iter: Chars<'b>,
        errors: Vec<RuleError>,
        index: i64,    // TODO Change to usize? No, because we use an iterator now. Or yes if we don't use Chars.
        lexeme: String,
        // TODO metaPushed: number;
        // TODO trail: TMeta[];
    }

    impl<T, S> Rule<T, S>
    {
        pub fn new(branch_fn: BranchFn<T, S>) -> Self
        {
            Rule { 
                branch_fn: branch_fn,
                parts: Vec::new(),
            }
        }

        // TODO Rename to `any_char`, also in TypeScript
        pub fn all(&mut self) -> &mut Self
        {
            self.parts.push(ScanFn::All);
            self
        }
        
        // TODO Rename to `any_char_except`, also in TypeScript
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
            
            if !list.iter().any(|t| { t.0.len() > 0 && t.1.len() > 0 }) {
                panic!("The strings in the list must be minimal one character long.");
            }
            
            self.parts.push(ScanFn::Alter(list));
            self
        }

        pub fn alter_string(&mut self, list: Vec<(String, String)>) -> &mut Self
        {
            if list.len() == 0 {
                panic!("List is empty.");
            }
            
            if !list.iter().any(|ref t| { t.0.len() > 0 && t.1.len() > 0 }) {
                panic!("The strings in the list must be minimal one character long.");
            }
            
            self.parts.push(ScanFn::AlterString(list));
            self
        }
        
        pub fn any_of(&mut self, mut rules: Vec<Rule<T, S>>) -> &mut Self
        {
            match rules.len() {
                0 => panic!("You must specify rules."),
                1 => self.parts.push(ScanFn::RangeOwned(1, 1, rules.pop().unwrap())),
                _ => self.parts.push(ScanFn::AnyOfOwned(rules)),
            };
            
            self
        }
        
        pub unsafe fn any_of_raw(&mut self, mut rules: Vec<*const Rule<T, S>>) -> &mut Self
        {
            match rules.len() {
                0 => panic!("You must specify rules."),
                1 => self.parts.push(ScanFn::RangeRaw(1, 1, rules[0])),
                _ => self.parts.push(ScanFn::AnyOfRaw(rules)),  
            };
            
            self
        }
        
        pub fn at_least(&mut self, count: u64, rule: Rule<T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(count, u64::max_value(), rule));
            self
        }
        
        pub unsafe fn at_least_raw(&mut self, count: u64, rule: *const Rule<T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeRaw(count, u64::max_value(), rule));
            self
        }
        
        pub fn at_most(&mut self, count: u64, rule: Rule<T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(0, count, rule));
            self
        }
        
        pub unsafe fn at_most_raw(&mut self, count: u64, rule: *const Rule<T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeRaw(0, count, rule));
            self
        }
        
        pub fn between(&mut self, min: u64, max: u64, rule: Rule<T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(min, max, rule));
            self
        }
        
        pub unsafe fn between_raw(&mut self, min: u64, max: u64, rule: *const Rule<T, S>) -> &mut Self
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
        
        pub fn exact(&mut self, count: u64, rule: Rule<T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(count, count, rule));
            self
        }
        
        pub unsafe fn exact_raw(&mut self, count: u64, rule: *const Rule<T, S>) -> &mut Self
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
        
        pub fn maybe(&mut self, rule: Rule<T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(0, 1, rule));
            self
        }
        
        pub unsafe fn maybe_raw(&mut self, rule: *const Rule<T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeRaw(0, 1, rule));
            self
        }
        
        pub fn none_or_many(&mut self, rule: Rule<T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(0, u64::max_value(), rule));
            self
        }
        
        pub unsafe fn none_or_many_raw(&mut self, rule: *const Rule<T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeRaw(0, u64::max_value(), rule));
            self
        }
        
        pub fn not(&mut self, rule: Rule<T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::NotOwned(rule));
            self
        }
        
        pub unsafe fn not_raw(&mut self, rule: *const Rule<T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::NotRaw(rule));
            self
        }
        
        pub fn one(&mut self, rule: Rule<T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeOwned(1, 1, rule));
            self
        }
        
        pub unsafe fn one_raw(&mut self, rule: *const Rule<T, S>) -> &mut Self
        {
            self.parts.push(ScanFn::RangeRaw(1, 1, rule));
            self
        }
        
        // TODO If we can add raw Rules then by defenition this `scan` function is unsafe.
        pub fn scan(&self, code: &str, mut shared: &mut S) -> Result<Vec<T>, Vec<RuleError>>
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

        // TODO It's not really a shallow_clone...
        pub unsafe fn shallow_clone(&self, branch_fn: BranchFn<T, S>) -> Self
        {
            Rule {
                branch_fn: branch_fn,
                parts: self.parts.iter().map(|p| p.shallow_clone()).collect(),
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
                    target.branches.append(&mut f(source.branches, &source.lexeme, &mut state));
                },
                _ => {
                    target.branches.append(&mut source.branches);
                },
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
                    ScanFn::AlterString(ref alter) => self.scan_alter_string_leaf(&alter, new_ctx),
                    ScanFn::AnyOfOwned(ref rules) => self.scan_any_of_owned(rules, new_ctx, &mut shared),
                    ScanFn::AnyOfRaw(ref rules) => self.scan_any_of_raw(rules, new_ctx, &mut shared),
                    ScanFn::CharIn(min, max) => self.scan_char_in_leaf(min, max, new_ctx),
                    ScanFn::Eof => self.scan_eof_leaf(new_ctx),
                    ScanFn::Literal(find) => self.scan_literal_leaf(&find, new_ctx),
                    ScanFn::LiteralString(ref text) => self.scan_literal_leaf(&text, new_ctx),
                    ScanFn::NotOwned(ref r) => self.scan_not(r as *const Rule<T, S>, new_ctx, &mut shared),
                    ScanFn::NotRaw(r) => self.scan_not(r, new_ctx, &mut shared),
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
            for alter in list {
                let find = alter.0;
                let len = find.chars().count();
                let compare: String = ctx.code_iter.clone().take(len).collect();

                if find == compare {
                    ctx.code_iter.nth(len - 1);
                    ctx.lexeme.push_str(alter.1);
                    ctx.index += len as i64;    // TODO As usize instead of i64
                    return Progress::Some(len as usize, ctx);
                }
            }

            self.update_error(ctx, String::from("Alter characters not found on this position."))
        }

        fn scan_alter_string_leaf<'b>(&'b self, list: &Vec<(String, String)>, mut ctx: ScanCtx<'b, T>) -> Progress<T>
        {
            for alter in list {
                let find = &alter.0;
                let len = find.chars().count();
                let compare: String = ctx.code_iter.clone().take(len).collect();

                if *find == compare {
                    ctx.code_iter.nth(len - 1);
                    ctx.lexeme.push_str(&alter.1);
                    ctx.index += len as i64;    // TODO As usize instead of i64
                    return Progress::Some(len as usize, ctx);
                }
            }

            self.update_error(ctx, String::from("Alter characters not found on this position."))
        }
        
        fn scan_any_of_owned<'b>(&'b self, rules: &'b Vec<Rule<T, S>>, ctx: ScanCtx<'b, T>, mut state: &mut S) -> Progress<T>
        {
            for r in rules {
                let new_ctx = self.branch(&ctx, false);

                if let Progress::Some(_, new_ctx) = r.run(new_ctx, &mut state) {
                    return self.merge(ctx, new_ctx, false, &mut state);
                } 
            }

            Progress::No(ctx)
        }
        
        fn scan_any_of_raw<'b>(&'b self, rules: &Vec<*const Rule<T, S>>, ctx: ScanCtx<'b, T>, mut state: &mut S) -> Progress<T>
        {
            for r in rules {
                let new_ctx = self.branch(&ctx, false);

                if let Progress::Some(_, new_ctx) = unsafe { (**r).run(new_ctx, &mut state) } {
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
        
        fn scan_not<'b>(&'b self, rule: *const Rule<T, S>, ctx: ScanCtx<'b, T>, mut state: &mut S) -> Progress<T>
        {
            match unsafe { (*rule).run(self.branch(&ctx, false), &mut state) } {
                Progress::Some(_, _) => Progress::No(ctx),
                Progress::No(_) => Progress::Some(0, ctx),
            }
        }
        
        fn scan_rule_range<'b>(&'b self, min: u64, max: u64, rule: *const Rule<T, S>, ctx: ScanCtx<'b, T>, mut state: &mut S) -> Progress<T>
        {
            let mut new_ctx = self.branch(&ctx, false);
            let mut count = 0u64;

            loop {
                match unsafe { (*rule).run(new_ctx, &mut state) } {
                    Progress::Some(progress, newer_ctx) => {
                        if progress == 0 {
                            return Progress::Some(0, ctx);
                        }

                        new_ctx = newer_ctx;
                        count += 1;

                        if count == max {
                            break;
                        }
                    },
                    Progress::No(initial_ctx) => {
                        new_ctx = initial_ctx;
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

        // TODO Better name
        fn shallow_clone_b(&self) -> Self
        {
            if self.branch_fn.is_some() {
                panic!("Could not `shallow_clone` because a rule part intergral of the root rule you want to `shallow_clone` has a branch_fn we cannot clone.");
            }

            Rule {
                branch_fn: None,
                parts: self.parts.iter().map(|p| p.shallow_clone()).collect(),
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

    struct ParseContext<T> {
        arg1: u64,
        arg2: u64, 
        arg3: Option<String>,
        arg4: Option<(String, String)>,
        range_type: RangeType,
        rule: Option<Rule<T, NoShared>>,
    }

    // TODO Use HashMap. It's faster and maybe we can revert back from `*mut Rule` into `Rule` in the RuleExp struct.
    // https://doc.rust-lang.org/std/collections/
    type RuleExprMap<T> = BTreeMap<&'static str, RuleExpr<T>>;

    struct GrammerShared<T> {
        rule_exps: *const RuleExprMap<T>,
        keep_ws: *const Rule<T, NoShared>,
    }

    // TODO TMeta class R<TB, TM> extends Rule<IParseContext<TB, TM>, IEmpty> {}
    type R<T> = Rule<ParseContext<T>, GrammerShared<T>>;
    
    // TODO Note: This was IRule, remove this comment later after porting.
    struct RuleExpr<T> {
        //TODO Maybe remove? id: &'static str,
        is_defined: bool,
        rule: *mut Rule<T, NoShared>,   
    }

    impl<T> Drop for RuleExpr<T> 
    {
        fn drop(&mut self) 
        {
            unsafe {
                let rule = Box::from_raw(self.rule);
            } 
        }
    }
    
    pub struct Grammer<T> /* <TBranch, TMeta> */
    {
        grammer: R<T>,
        rule_exps: RuleExprMap<T>,
        
        keep_alter_tuple: Box<R<T>>,        // We need to keep rules defined in the `new` function alive.
        keep_integer: Box<R<T>>,            // ..
        keep_ranges: Box<R<T>>,             // ..
        keep_statement: Box<R<T>>,          // ..
        keep_ws: Rule<T, NoShared>,         // ..
    }

    impl<T> Grammer<T>
    {
        pub fn new() -> Self
        {
            let rule_exps = RuleExprMap::new();

            let mut space = Rule::new(None); space.literal(" ");
            let mut tab = Rule::new(None); tab.literal("\t");
            let mut new_line = Rule::new(None); new_line.literal("\n");
            let mut carriage_return = Rule::new(None); carriage_return.literal("\r");
            let mut ws = Rule::new(None);
            ws.any_of(vec![space, tab, new_line, carriage_return]);

            let statement_fn = |mut b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
            {
                match b[0].range_type {
                    RangeType::Not => {
                        let mut r: Rule<T, NoShared> = Rule::new(None);
                        r.not(b.pop().unwrap().rule.unwrap());    // TODO Test this, originally it was b[1]
                                                                        // Does this goes well together with ranges? 

                        vec![ParseContext{ 
                            arg1: 0,
                            arg2: 0,
                            arg3: None,
                            arg4: None,
                            range_type: RangeType::NoRangeType,
                            rule: Some(r), 
                        }]
                    },
                    _ => {
                        b
                    }
                }
            };

            let boxed_ranges = Box::new(R::new(None));  // Allocate memory for the "ranges" rule.
            let ranges = Box::into_raw(boxed_ranges);   // Transform it into a raw pointer to be used by other rules.

            let boxed_statement = Box::new(R::new(Some(Box::new(statement_fn))));
            let statement = Box::into_raw(boxed_statement);
            
            let mut escaped_ctrl_chars: R<T> = R::new(None);
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

            // Integer
            let integer_fn = |_: Vec<ParseContext<T>>, l: &str, _: &mut GrammerShared<T>|
            {
                vec![ParseContext { 
                    arg1: l.parse::<u64>().unwrap(),
                    arg2: 0,
                    arg3: None,
                    arg4: None,
                    range_type: RangeType::NoRangeType,
                    rule: None, 
                }]
            };

            let mut digit = R::new(None); digit.char_in('0', '9');
            let boxed_integer = Box::new(R::new(Some(Box::new(integer_fn))));
            let integer = Box::into_raw(boxed_integer); 
            unsafe { (*integer).at_least(1, digit); } 
            
            // Literal
            let literal_text_fn = |_: Vec<ParseContext<T>>, l: &str, _: &mut GrammerShared<T>|
            {
                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: Some(String::from(l)),
                    arg4: None,
                    range_type: RangeType::NoRangeType,
                    rule: None, 
                }]
            };

            let mut literal_all_except: R<T> = R::new(None);
            literal_all_except.all_except(vec!['<', '{', '(', ')', '|', '[', '+', '?', '*', '.', '$', ' ', '_', '!']);

            let mut literal_char: R<T> = R::new(None);
            unsafe { literal_char.any_of(vec![escaped_ctrl_chars.shallow_clone(None), literal_all_except]); }
            
            let mut literal_text: R<T> = R::new(Some(Box::new(literal_text_fn)));
            literal_text.at_least(1, literal_char);

            let literal_fn = |b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
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
                    arg4: None,
                    range_type: RangeType::NoRangeType,
                    rule: Some(rule),
                }]
            };

            let mut literal: R<T> = R::new(Some(Box::new(literal_fn)));
            unsafe { literal.one(literal_text).maybe_raw(ranges); }
            
            // Any char
            let any_char_fn = |b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
            {
                let mut rule = Rule::new(None);
                rule.all();

                if b.len() == 1 {
                    rule = Grammer::add_range(rule, &b[0]);
                }

                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: None,
                    arg4: None,
                    range_type: RangeType::NoRangeType,
                    rule: Some(rule),
                }]
            };

            let mut any_char = R::new(Some(Box::new(any_char_fn)));
            unsafe { any_char.literal(".").maybe_raw(ranges); }

            // Any char except
            let any_char_except_chars_fn = |_: Vec<ParseContext<T>>, l: &str, _: &mut GrammerShared<T>|
            {
                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: Some(String::from(l)),
                    arg4: None,
                    range_type: RangeType::NoRangeType,
                    rule: None,
                }]
            };

            let any_char_except_fn = |mut b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
            {
                let mut rule = Rule::new(None);

                if b.len() == 1 {
                    rule.all_except(b.pop().unwrap().arg3.unwrap().chars().map(|c| c).collect());
                } 
                else {
                    let last = b.pop().unwrap();
                    rule.all_except(b.pop().unwrap().arg3.unwrap().chars().map(|c| c).collect());
                    rule = Grammer::add_range(rule, &last);
                }

                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: None,
                    arg4: None,
                    range_type: RangeType::NoRangeType,
                    rule: Some(rule),
                }]
            };
            
            let mut any_char_except_any_other = R::new(None); 
            any_char_except_any_other.all_except(vec![']']);

            let mut any_char_except_char = R::new(None); 
            unsafe { any_char_except_char.any_of(vec![escaped_ctrl_chars.shallow_clone(None), any_char_except_any_other]); }

            let mut any_char_except_chars = R::new(Some(Box::new(any_char_except_chars_fn))); 
            any_char_except_chars.at_least(1, any_char_except_char);

            let mut any_char_except = R::new(Some(Box::new(any_char_except_fn))); 
            unsafe { any_char_except.literal("[^").one(any_char_except_chars).literal("]").maybe_raw(ranges); }
            
            // Match character range
            let char_range_fn = |_: Vec<ParseContext<T>>, l: &str, _: &mut GrammerShared<T>|
            {
                let lower = l.chars().next().unwrap();
                let upper = l.chars().skip(2).next().unwrap();

                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: Some(char::to_string(&lower)),
                    arg4: None,
                    range_type: RangeType::NoRangeType,
                    rule: None,
                },
                ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: Some(char::to_string(&upper)),
                    arg4: None,
                    range_type: RangeType::NoRangeType,
                    rule: None,
                }]
            };

            let char_ranges_fn = |mut b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
            {
                let mut rule = Rule::new(None);
                
                if b.len() > 2 {
                    let mut ranges = Vec::new();
                    
                    while b.len() > 1 {
                        let rest = b.split_off(2);
                        
                        let upper = b.pop().unwrap().arg3.unwrap().chars().next().unwrap();
                        let lower = b.pop().unwrap().arg3.unwrap().chars().next().unwrap();

                        let mut r = Rule::new(None);
                        r.char_in(lower, upper);

                        ranges.push(r);

                        b = rest; 
                    }

                    rule.any_of(ranges);
                }
                else {
                    let upper = b.pop().unwrap().arg3.unwrap().chars().next().unwrap();
                    let lower = b.pop().unwrap().arg3.unwrap().chars().next().unwrap();
                    rule.char_in(lower, upper);
                }

                if let Some(ctx) = b.pop() {
                    rule = Grammer::add_range(rule, &ctx);
                }

                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: None,
                    arg4: None,
                    range_type: RangeType::NoRangeType,
                    rule: Some(rule),
                }]
            };

            let mut char_range_all_except = R::new(None);
            char_range_all_except.all_except(vec!['-', ']']);

            let mut char_range_char = R::new(None);
            unsafe { char_range_char.any_of(vec![escaped_ctrl_chars.shallow_clone(None), char_range_all_except]); }

            let mut char_range = R::new(Some(Box::new(char_range_fn)));
            unsafe { char_range.one(char_range_char.shallow_clone(None)).literal("-").one(char_range_char); }

            let mut char_ranges = R::new(Some(Box::new(char_ranges_fn)));
            unsafe { char_ranges.literal("[").at_least(1, char_range).literal("]").maybe_raw(ranges); }

            // EOF
            let eof_fn = |_: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
            {
                let mut rule = Rule::new(None);
                rule.eof();

                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: None,
                    arg4: None,
                    range_type: RangeType::NoRangeType,
                    rule: Some(rule),
                }]
            };

            let mut eof = R::new(Some(Box::new(eof_fn)));
            eof.literal("$");
            
            // One rule
            let rule_name_fn = |_: Vec<ParseContext<T>>, l: &str, _: &mut GrammerShared<T>|
            {
                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: Some(String::from(l)),
                    arg4: None,
                    range_type: RangeType::NoRangeType,
                    rule: None,
                }]
            };

            let mut rule_name_all_except = R::new(None);
            rule_name_all_except.all_except(vec!['>']);

            let mut rule_name_char = R::new(None);
            unsafe { rule_name_char.any_of(vec![escaped_ctrl_chars.shallow_clone(None), rule_name_all_except]); }

            let mut rule_name = R::new(Some(Box::new(rule_name_fn)));
            rule_name.at_least(1, rule_name_char);

            let rule_fn = |mut b: Vec<ParseContext<T>>, _: &str, s: &mut GrammerShared<T>|
            {
                if b.len() == 1 {
                    // TODO This note is not only meant for the code below, but could we just use `ref` instead of popping?
                    let id = b.pop().unwrap().arg3.unwrap();

                    match unsafe { (*s.rule_exps).get(id.as_str()) } {
                        None => {
                            panic!("Rule \"{}\" not found", id)
                        },
                        Some(r) => {
                            let mut rule = Rule::new(None);
                            unsafe { rule.one_raw(r.rule) };

                            vec![ParseContext { 
                                arg1: 0,
                                arg2: 0,
                                arg3: None,
                                arg4: None,
                                range_type: RangeType::NoRangeType,
                                rule: Some(rule),
                            }]
                        },
                    }
                }
                else {
                    let range = b.pop().unwrap();
                    let id = b.pop().unwrap().arg3.unwrap();

                    match unsafe { (*s.rule_exps).get(id.as_str()) } {
                        None => {
                            panic!("Rule \"{}\" not found", id)
                        },
                        Some(r) => {
                            vec![ParseContext { 
                                arg1: 0,
                                arg2: 0,
                                arg3: None,
                                arg4: None,
                                range_type: RangeType::NoRangeType,
                                rule: Some(Grammer::add_range_raw(r.rule, &range)),
                            }]
                        },
                    }
                }
            };

            let mut rule = R::new(Some(Box::new(rule_fn)));
            unsafe { rule.literal("<").one(rule_name).literal(">").maybe_raw(ranges) };
            
            // At least
            let at_least_fn = |b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
            {
                vec![ParseContext { 
                    arg1: b[0].arg1,
                    arg2: 0,
                    arg3: None,
                    arg4: None,
                    range_type: RangeType::AtLeast,
                    rule: None,
                }]
            };

            let mut at_least = R::new(Some(Box::new(at_least_fn))); 
            unsafe { at_least.literal("{").one_raw(integer).literal(",}"); }

            // At least one
            let at_least_one_fn = |_: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
            {
                vec![ParseContext { 
                    arg1: 1,
                    arg2: 0,
                    arg3: None,
                    arg4: None,
                    range_type: RangeType::AtLeast,
                    rule: None,
                }]
            };

            let mut at_least_one = R::new(Some(Box::new(at_least_one_fn))); 
            at_least_one.literal("+");

            // At most
            let at_most_fn = |b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
            {
                vec![ParseContext { 
                    arg1: b[0].arg1,
                    arg2: 0,
                    arg3: None,
                    arg4: None,
                    range_type: RangeType::AtMost,
                    rule: None,
                }]
            };

            let mut at_most = R::new(Some(Box::new(at_most_fn))); 
            unsafe { at_most.literal("{,").one_raw(integer).literal("}"); }

            // Between
            let between_fn = |b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
            {
                vec![ParseContext { 
                    arg1: b[0].arg1,
                    arg2: b[1].arg1,
                    arg3: None,
                    arg4: None,
                    range_type: RangeType::Between,
                    rule: None,
                }]
            };

            let mut between = R::new(Some(Box::new(between_fn))); 
            unsafe { between.literal("{").one_raw(integer).literal(",").one_raw(integer).literal("}"); }

            // Exact
            let exact_fn = |b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
            {
                vec![ParseContext { 
                    arg1: b[0].arg1,
                    arg2: 0,
                    arg3: None,
                    arg4: None,
                    range_type: RangeType::Exact,
                    rule: None,
                }]
            };

            let mut exact = R::new(Some(Box::new(exact_fn))); 
            unsafe { exact.literal("{").one_raw(integer).literal("}"); }

            // Maybe
            let maybe_fn = |_: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
            {
                vec![ParseContext { 
                    arg1: 0,
                    arg2: 1,
                    arg3: None,
                    arg4: None,
                    range_type: RangeType::Between,
                    rule: None,
                }]
            };

            let mut maybe = Rule::new(Some(Box::new(maybe_fn)));
            maybe.literal("?");

            // None or many
            let none_or_many_fn = |_: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
            {
                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: None,
                    arg4: None,
                    range_type: RangeType::AtLeast,
                    rule: None,
                }]
            };

            let mut none_or_many = Rule::new(Some(Box::new(none_or_many_fn)));
            none_or_many.literal("*");
            
            // Not
            let not_fn = |_: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
            {
                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: None,
                    arg4: None,
                    range_type: RangeType::Not,
                    rule: None,
                }]
            };

            let mut not = Rule::new(Some(Box::new(not_fn)));
            not.literal("!");

            // Any of
            let any_of_fn = |mut b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
            {
                let last = b.pop().unwrap();
                let mut rule = Rule::new(None);
                let mut rules: Vec<Rule<T, NoShared>> = b.into_iter().map(|c| c.rule.unwrap()).collect();

                match last.range_type {
                    RangeType::NoRangeType => {
                        rules.push(last.rule.unwrap());
                        rule.any_of(rules);
                    },
                    _ => {
                        rule.any_of(rules);
                        rule = Grammer::add_range(rule, &last);
                    },
                }

                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: None,
                    arg4: None,
                    range_type: RangeType::NoRangeType,
                    rule: Some(rule),
                }]
            };

            let statements_fn = |b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
            {
                if b.len() == 1 {
                    b
                }
                else {
                    let mut rule = Rule::new(None);

                    for ctx in b {
                        rule.one(ctx.rule.unwrap());
                    }

                    vec![ParseContext { 
                        arg1: 0,
                        arg2: 0,
                        arg3: None,
                        arg4: None,
                        range_type: RangeType::NoRangeType,
                        rule: Some(rule),
                    }]
                }                
            };

            let mut statements = R::new(Some(Box::new(statements_fn)));
            unsafe { statements.at_least_raw(1, statement); }

            let mut more = R::new(None);
            unsafe { more.literal("|").one(statements.shallow_clone(None)); }

            let mut any_of = R::new(Some(Box::new(any_of_fn)));
            unsafe { any_of.literal("(").one(statements).none_or_many(more).literal(")").maybe_raw(ranges); }

            // Alter
            let alter_fn = |mut b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
            {
                // TODO Can we use `ref` instead of pop and pushing?
                let last = b.pop().unwrap();

                match last.range_type {
                    RangeType::NoRangeType => {
                        b.push(last);

                        let mut rule = Rule::new(None);
                        rule.alter_string(b.into_iter().map(|i| i.arg4.unwrap()).collect());
                        
                        vec![ParseContext { 
                            arg1: 0,
                            arg2: 0,
                            arg3: None,
                            arg4: None,
                            range_type: RangeType::NoRangeType,
                            rule: Some(rule),
                        }]
                    },
                    _ => {
                        let mut rule = Rule::new(None);
                        rule.alter_string(b.into_iter().map(|i| i.arg4.unwrap()).collect());

                        vec![ParseContext { 
                            arg1: 0,
                            arg2: 0,
                            arg3: None,
                            arg4: None,
                            range_type: RangeType::NoRangeType,
                            rule: Some(Grammer::add_range(rule, &last)),
                        }]
                    },
                }        
            };

            let alter_left_text_fn = |_: Vec<ParseContext<T>>, l: &str, _: &mut GrammerShared<T>|
            {
                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: Some(String::from(l)),
                    arg4: None,
                    range_type: RangeType::NoRangeType,
                    rule: None,
                }]
            };

            let alter_right_text_fn = |_: Vec<ParseContext<T>>, l: &str, _: &mut GrammerShared<T>|
            {
                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: Some(String::from(l)),
                    arg4: None,
                    range_type: RangeType::NoRangeType,
                    rule: None,
                }]
            };

            let alter_tuple_fn = |mut b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
            {
                let to = b.pop().unwrap().arg3.unwrap();
                let from = b.pop().unwrap().arg3.unwrap();

                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: None,
                    arg4: Some((from, to)),
                    range_type: RangeType::NoRangeType,
                    rule: None,
                }]
            };

            let mut alter_all_except_left_char = R::new(None);
            alter_all_except_left_char.all_except(vec![',']);
            
            let mut alter_left_char = R::new(None);
            unsafe { alter_left_char.any_of(vec![escaped_ctrl_chars.shallow_clone(None), alter_all_except_left_char]); }

            let mut alter_left_text = R::new(Some(Box::new(alter_left_text_fn)));
            alter_left_text.at_least(1, alter_left_char);

            let mut alter_all_except_right_char = R::new(None);
            alter_all_except_right_char.all_except(vec!['|', ')']);
            
            let mut alter_right_char = R::new(None);
            alter_right_char.any_of(vec![escaped_ctrl_chars, alter_all_except_right_char]);

            let mut alter_right_text = R::new(Some(Box::new(alter_right_text_fn)));
            alter_right_text.at_least(1, alter_right_char);

            let mut alter_tuple = R::new(Some(Box::new(alter_tuple_fn)));
            alter_tuple.one(alter_left_text).literal(",").one(alter_right_text);
            let alter_tuple = Box::into_raw(Box::new(alter_tuple));

            let mut alter_more = R::new(None);
            unsafe { alter_more.literal("|").one_raw(alter_tuple); }

            let mut alter = R::new(Some(Box::new(alter_fn)));
            unsafe { alter.literal("(~").one_raw(alter_tuple).none_or_many(alter_more).literal(")").maybe_raw(ranges); }

            // Whitespace
            let at_least_one_ws_fn = |_: Vec<ParseContext<T>>, _: &str, s: &mut GrammerShared<T>|
            {
                let mut r = Rule::new(None);
                unsafe { r.at_least_raw(1, s.keep_ws); }

                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: None,
                    arg4: None,
                    range_type: RangeType::NoRangeType,
                    rule: Some(r),
                }]
            };

            let none_or_many_ws_fn = |_: Vec<ParseContext<T>>, _: &str, s: &mut GrammerShared<T>|
            {
                let mut r = Rule::new(None);
                unsafe { r.none_or_many_raw(s.keep_ws); }

                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: None,
                    arg4: None,
                    range_type: RangeType::NoRangeType,
                    rule: Some(r),
                }]
            };

            let mut at_least_one_ws = R::new(Some(Box::new(at_least_one_ws_fn)));
            at_least_one_ws.literal("_");

            let mut none_or_many_ws = R::new(Some(Box::new(none_or_many_ws_fn)));
            none_or_many_ws.literal(" ");

            // Ranges and statements definitions
            unsafe {
                (*ranges).any_of(vec![at_least, at_least_one, at_most, between, exact, maybe, none_or_many]);
                (*statement).maybe(not).any_of(vec![any_char, none_or_many_ws, at_least_one_ws, eof, alter, any_char_except, char_ranges, rule, any_of, literal]);
            }
            
            let mut grammer = R::new(None);
            unsafe { grammer.none_or_many_raw(statement); }
            
            Grammer {
                grammer: grammer, 
                rule_exps: rule_exps,

                keep_alter_tuple: unsafe { Box::from_raw(alter_tuple) },    // We need to keep these rules alive because they are used in other rules as raw references.
                keep_integer: unsafe { Box::from_raw(integer) },            // ..
                keep_ranges: unsafe { Box::from_raw(ranges) },              // ..
                keep_statement: unsafe { Box::from_raw(statement) },        // ..
                keep_ws: ws,                                                // ..
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

            let mut shared = GrammerShared {
                rule_exps: &self.rule_exps,
                keep_ws: &self.keep_ws,
            };

            let result = self.grammer.scan(&expr, &mut shared); 

            match result {
                Err(_) => {
                    panic!("Error compiling rule expression.");    // TODO Return nice error
                },
                Ok(mut branches) => {
                    let new_ruleexp = match self.rule_exps.get_mut(id) {
                        None => {
                            let mut compiled = Rule::new(branch_fn);
                            
                            for r in branches {
                                compiled.one(r.rule.unwrap());
                            }

                            Some(RuleExpr {
                                // TODO Maybe remove? id: id,
                                is_defined: true,
                                rule: Box::into_raw(Box::new(compiled)),
                            })
                        },
                        Some(rulexp) => {
                            rulexp.is_defined = true;
                            // TODO rulexp.rule.meta = meta;

                            unsafe {
                                (*rulexp.rule).branch_fn = branch_fn;

                                for r in branches {
                                    (*rulexp.rule).one(r.rule.unwrap());
                                }
                            }

                            None
                        },
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

                let rule = Box::new(Rule::new(None));
                
                self.rule_exps.insert(id, RuleExpr {
                    // TODO Maybe remove? id: id,
                    is_defined: false,
                    rule: Box::into_raw(rule),
                });
            }
        }
        
        // TODO Make a type of this return type.
        pub fn scan(&self, root_id: &str, code: &str) -> Result<Vec<T>, Vec<RuleError>>
        {
            let mut dummy = NoShared {};

            match self.rule_exps.get(root_id) {
                Some(r) => {
                    unsafe {
                        (*r.rule).scan(code, &mut dummy)
                    }
                },
                None => panic!("Rule with id \"{}\" not found.", root_id),
            }
        }
        
        pub fn ws(&mut self, expr: &str) 
        {
            let mut shared = GrammerShared {
                rule_exps: &self.rule_exps,
                keep_ws: &self.keep_ws,
            };

            match self.grammer.scan(expr, &mut shared) {
                Ok(mut b) => {
                    if b.len() != 1 {
                        panic!("Error compiling rule expression.");
                    }
                    
                    let r = b.pop().unwrap().rule.unwrap();
                    self.keep_ws.clear().one(r);
                },
                Err(_) => panic!("Error compiling rule expression."),
            }
        }
        
        // TODO Replace with add_range_raw.
        fn add_range(rule: Rule<T, NoShared>, ctx: &ParseContext<T>) -> Rule<T, NoShared>
        {
            match ctx.range_type {
                RangeType::AtLeast => {
                    let mut r = Rule::new(None);
                    r.at_least(ctx.arg1, rule);
                    r
                },
                RangeType::AtMost => {
                    let mut r = Rule::new(None);
                    r.at_most(ctx.arg1, rule);
                    r
                },
                RangeType::Between => {
                    let mut r = Rule::new(None);
                    r.between(ctx.arg1, ctx.arg2, rule);
                    r
                },
                RangeType::Exact => {
                    let mut r = Rule::new(None);
                    r.exact(ctx.arg1, rule);
                    r
                },
                RangeType::NoRangeType => {
                    rule
                },
                RangeType::Not => {
                    panic!("Application error")     // TODO Fix this, this is `unreachable!()`
                },
            }
        }

        fn add_range_raw(rule: *const Rule<T, NoShared>, ctx: &ParseContext<T>) -> Rule<T, NoShared>
        {
            match ctx.range_type {
                RangeType::AtLeast => {
                    let mut r = Rule::new(None);
                    unsafe { r.at_least_raw(ctx.arg1, rule); }
                    r
                },
                RangeType::AtMost => {
                    let mut r = Rule::new(None);
                    unsafe { r.at_most_raw(ctx.arg1, rule); }
                    r
                },
                RangeType::Between => {
                    let mut r = Rule::new(None);
                    unsafe { r.between_raw(ctx.arg1, ctx.arg2, rule); }
                    r
                },
                RangeType::Exact => {
                    let mut r = Rule::new(None);
                    unsafe { r.exact_raw(ctx.arg1, rule); }
                    r
                },
                RangeType::NoRangeType => {
                    let mut r = Rule::new(None);
                    unsafe { r.one_raw(rule); }
                    r
                },
                RangeType::Not => {
                    panic!("Application error")     // TODO Fix this, this is `unreachable!()`
                },
            }
        }
    }
}

#[cfg(test)]
mod tests 
{
    use abitvin::Grammer;
    use abitvin::NoShared;

    #[test]
    fn grammer_alter()
    {
        let code = "\\<東\\<💝\\>中\\>"; // There are gonna be 7 replacements.
        
        let f = |_: Vec<i32>, l: &str, _: &mut NoShared| {
            assert_eq!(l, "<AAA<BBB>CCC>");
            vec![111, 222]
        };

        let mut grammer: Grammer<i32> = Grammer::new();
        grammer.add("alter", "(~\\\\<,<|\\\\>,>|東,AAA|💝,BBB|中,CCC)", None);
        grammer.add("root", "<alter>{7}", Some(Box::new(f)));
        
        if let Ok(b) = grammer.scan("root", code) {
            assert_eq!(b.len(), 2);
            assert_eq!(b[0], 111);
            assert_eq!(b[1], 222);
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn grammer_any_char()
    {
        let mut grammer: Grammer<i32> = Grammer::new();
        grammer.add("test-a", ".", None);
        grammer.add("test-b", ".?", None);
        grammer.add("test-c", ".+", None);
        grammer.add("test-d", "\\.", None);
        
        assert!(grammer.scan("test-a", "").is_err());
        assert!(grammer.scan("test-a", "A").is_ok());
        assert!(grammer.scan("test-a", "💝").is_ok());
        assert!(grammer.scan("test-a", "💝💝").is_err());
        assert!(grammer.scan("test-b", "").is_ok());
        assert!(grammer.scan("test-b", "💝").is_ok());
        assert!(grammer.scan("test-b", "💝💝").is_err());
        assert!(grammer.scan("test-c", "").is_err());
        assert!(grammer.scan("test-c", "💝").is_ok());
        assert!(grammer.scan("test-c", "💝💝").is_ok());
        assert!(grammer.scan("test-d", "A").is_err());
        assert!(grammer.scan("test-d", ".").is_ok());
    }

    #[test]
    fn grammer_any_char_except()
    {
        let mut grammer: Grammer<i32> = Grammer::new();
        grammer.add("test-a", "[^ABC💝]", None);
        grammer.add("test-b", "[^ABC💝]*", None);
        
        assert!(grammer.scan("test-a", "").is_err());
        assert!(grammer.scan("test-a", "a").is_ok());
        assert!(grammer.scan("test-a", "A").is_err());
        assert!(grammer.scan("test-a", "💝").is_err());
        assert!(grammer.scan("test-b", "").is_ok());
        assert!(grammer.scan("test-b", "banana is love!").is_ok());
        assert!(grammer.scan("test-b", "BANANA IS LOVE!").is_err());
        assert!(grammer.scan("test-b", "banana is 💝!").is_err());
    }

    #[test]
    fn grammer_any_of()
    {
        let mut grammer: Grammer<i32> = Grammer::new();
        grammer.add("a", "a", None);
        grammer.add("abc", "(<a>|b|c+)", None);
        grammer.add("test-a", "<abc>", None);
        grammer.add("test-b", "XXX<abc>", None);
        grammer.add("test-c", "<abc>YYY", None);
        grammer.add("test-d", "XXX<abc>YYY", None);

        assert!(grammer.scan("test-a", "a").is_ok());
        assert!(grammer.scan("test-a", "aa").is_err());
        assert!(grammer.scan("test-a", "b").is_ok());
        assert!(grammer.scan("test-a", "bb").is_err());
        assert!(grammer.scan("test-a", "x").is_err());
        assert!(grammer.scan("test-a", "c").is_ok());
        assert!(grammer.scan("test-a", "cc").is_ok());

        assert!(grammer.scan("test-b", "XXXa").is_ok());
        assert!(grammer.scan("test-b", "XXXaa").is_err());
        assert!(grammer.scan("test-b", "XXXb").is_ok());
        assert!(grammer.scan("test-b", "XXXbb").is_err());
        assert!(grammer.scan("test-b", "XXXx").is_err());
        assert!(grammer.scan("test-b", "XXXc").is_ok());
        assert!(grammer.scan("test-b", "XXXcc").is_ok());

        assert!(grammer.scan("test-c", "aYYY").is_ok());
        assert!(grammer.scan("test-c", "aaYYY").is_err());
        assert!(grammer.scan("test-c", "bYYY").is_ok());
        assert!(grammer.scan("test-c", "bbYYY").is_err());
        assert!(grammer.scan("test-c", "xYYY").is_err());
        assert!(grammer.scan("test-c", "cYYY").is_ok());
        assert!(grammer.scan("test-c", "ccYYY").is_ok());
        
        assert!(grammer.scan("test-d", "XXXaYYY").is_ok());
        assert!(grammer.scan("test-d", "XXXaaYYY").is_err());
        assert!(grammer.scan("test-d", "XXXbYYY").is_ok());
        assert!(grammer.scan("test-d", "XXXbbYYY").is_err());
        assert!(grammer.scan("test-d", "XXXxYYY").is_err());
        assert!(grammer.scan("test-d", "XXXcYYY").is_ok());
        assert!(grammer.scan("test-d", "XXXccYYY").is_ok());
    }

    #[test]
    fn grammer_at_least()
    {
        let f = |_: Vec<i32>, _: &str, _: &mut NoShared| {
            vec![1234]
        };

        let mut grammer: Grammer<i32> = Grammer::new();
        grammer.add("root", "monkey{2,}", Some(Box::new(f)));

        if let Ok(_) = grammer.scan("root", "") {
            assert!(false);
        }
        else {
            assert!(true);
        }

        if let Ok(_) = grammer.scan("root", "monkey") {
            assert!(false);
        }
        else {
            assert!(true);
        }

        if let Ok(branches) = grammer.scan("root", "monkeymonkey") {
            assert_eq!(branches[0], 1234);
        }
        else {
            assert!(false);
        }

        if let Ok(branches) = grammer.scan("root", "monkeymonkeymonkeymonkeymonkeymonkey") {
            assert_eq!(branches[0], 1234);
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn grammer_at_least_one()
    {
        let f = |_: Vec<i32>, _: &str, _: &mut NoShared| {
            vec![5678]
        };

        let mut grammer: Grammer<i32> = Grammer::new();
        grammer.add("root", "monkey+", Some(Box::new(f)));

        if let Ok(_) = grammer.scan("root", "") {
            assert!(false);
        }
        else {
            assert!(true);
        }

        if let Ok(branches) = grammer.scan("root", "monkey") {
            assert_eq!(branches[0], 5678);
        }
        else {
            assert!(false);
        }

        if let Ok(branches) = grammer.scan("root", "monkeymonkeymonkeymonkeymonkeymonkey") {
            assert_eq!(branches[0], 5678);
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn grammer_at_most()
    {
        let f = |_: Vec<i32>, _: &str, _: &mut NoShared| {
            vec![1234]
        };

        let mut grammer: Grammer<i32> = Grammer::new();
        grammer.add("root", "monkey{,2}", Some(Box::new(f)));

        if let Ok(branches) = grammer.scan("root", "") {
            assert_eq!(branches[0], 1234);
        }
        else {
            assert!(false);
        }

        if let Ok(branches) = grammer.scan("root", "monkey") {
            assert_eq!(branches.len(), 1);
            assert_eq!(branches[0], 1234);
        }
        else {
            assert!(false);
        }

        if let Ok(branches) = grammer.scan("root", "monkeymonkey") {
            assert_eq!(branches.len(), 1);
            assert_eq!(branches[0], 1234);
        }
        else {
            assert!(false);
        }

        if let Ok(_) = grammer.scan("root", "monkeymonkeymonkeymonkeymonkeymonkey") {
            assert!(false);
        }
        else {
            assert!(true);
        }
    }

    #[test]
    fn grammer_between()
    {
        let f = |_: Vec<i32>, _: &str, _: &mut NoShared| {
            vec![1234]
        };

        let mut grammer: Grammer<i32> = Grammer::new();
        grammer.add("root", "monkey{2,4}", Some(Box::new(f)));

        if let Ok(_) = grammer.scan("root", "") {
            assert!(false);
        }
        else {
            assert!(true);
        }

        if let Ok(_) = grammer.scan("root", "monkey") {
            assert!(false);
        }
        else {
            assert!(true);
        }

        if let Ok(branches) = grammer.scan("root", "monkeymonkey") {
            assert_eq!(branches.len(), 1);
            assert_eq!(branches[0], 1234);
        }
        else {
            assert!(false);
        }

        if let Ok(branches) = grammer.scan("root", "monkeymonkeymonkeymonkey") {
            assert_eq!(branches.len(), 1);
            assert_eq!(branches[0], 1234);
        }
        else {
            assert!(false);
        }

        if let Ok(_) = grammer.scan("root", "monkeymonkeymonkeymonkeymonkey") {
            assert!(false);
        }
        else {
            assert!(true);
        }
    }

    #[test]
    fn grammer_char_in()
    {
        let mut grammer: Grammer<i32> = Grammer::new();
        grammer.add("test-a", "[a-z]", None);
        grammer.add("test-b", "[😀-🙏]", None);         // Emoticons (Emoji) U+1F600 — U+1F64F
        grammer.add("test-c", "[a-zA-Z0-9]+", None);
        
        assert!(grammer.scan("test-a", "").is_err());
        assert!(grammer.scan("test-a", "x").is_ok());
        assert!(grammer.scan("test-a", "A").is_err());
        assert!(grammer.scan("test-b", "☺").is_err());  // Alhough a smiley (emoji), this char (U+263A) is not in the range we given. 
        assert!(grammer.scan("test-b", "😍").is_ok());
        assert!(grammer.scan("test-b", "😷").is_ok());
        assert!(grammer.scan("test-c", "Banana304").is_ok());
        assert!(grammer.scan("test-c", "Monkey80085").is_ok());
    }

    #[test]
    fn grammer_custom_spaces()
    {
        let mut grammer: Grammer<i32> = Grammer::new();
        grammer.add("test-a", "_", None);
        grammer.add("test-b", " ", None);
        grammer.add("test-c", "monkey monkey_monkey", None);
        
        // It's better practice to add the whitespace declaration at the beginning.
        grammer.ws("\\*");    // TODO Make a more advanced whitespace rule.

        assert!(grammer.scan("test-a", "").is_err());
        assert!(grammer.scan("test-a", "***").is_ok());
        assert!(grammer.scan("test-b", "").is_ok());
        assert!(grammer.scan("test-b", "***").is_ok());
        assert!(grammer.scan("test-c", "monkey*****monkey*************monkey").is_ok());
        assert!(grammer.scan("test-c", "monkeymonkey*monkey").is_ok());
        assert!(grammer.scan("test-c", "monkey*monkeymonkey").is_err());
    }

    #[test]
    fn grammer_eof()
    {
        let mut grammer: Grammer<i32> = Grammer::new();
        grammer.add("new-line", "\r?\n", None);      // TODO Fix id naming bug, rename this id to something like "aaaaaaa" will make the test successfull.
        grammer.add("line", "line(<new-line>|$)", None);
        grammer.add("root", "<line>*", None);

        assert!(grammer.scan("root", "").is_ok());
        assert!(grammer.scan("root", "line").is_ok());
        assert!(grammer.scan("root", "line\n").is_ok());
        assert!(grammer.scan("root", "line\nline").is_ok());
        assert!(grammer.scan("root", "line\r\nline\n").is_ok());
    }
    
    #[test]
    fn grammer_exact()
    {
        let f = |_: Vec<i32>, _: &str, _: &mut NoShared| {
            vec![1234]
        };

        let mut grammer: Grammer<i32> = Grammer::new();
        grammer.add("root", "monkey{2}", Some(Box::new(f)));

        if let Ok(_) = grammer.scan("root", "") {
            assert!(false);
        }
        else {
            assert!(true);
        }

        if let Ok(_) = grammer.scan("root", "monkey") {
            assert!(false);
        }
        else {
            assert!(true);
        }

        if let Ok(branches) = grammer.scan("root", "monkeymonkey") {
            assert_eq!(branches.len(), 1);
            assert_eq!(branches[0], 1234);
        }
        else {
            assert!(false);
        }

        if let Ok(_) = grammer.scan("root", "monkeymonkeymonkeymonkeymonkeymonkey") {
            assert!(false);
        }
        else {
            assert!(true);
        }
    }

    #[test]
    fn grammer_literal()
    {
        let f = |_: Vec<i32>, _: &str, _: &mut NoShared| {
            vec![123, 456, 789]
        };

        let mut grammer: Grammer<i32> = Grammer::new();
        grammer.add("root", "monkey", Some(Box::new(f)));

        if let Ok(branches) = grammer.scan("root", "monkey") {
            assert_eq!(branches[0], 123);
            assert_eq!(branches[1], 456);
            assert_eq!(branches[2], 789);
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn grammer_maybe()
    {
        let f = |_: Vec<i32>, _: &str, _: &mut NoShared| {
            vec![1940, 3, 10]
        };

        let mut grammer: Grammer<i32> = Grammer::new();
        //TODO grammer.add("root", "Chuck\\ Norris\\ counted\\ to\\ infinity\\ -\\ twice?", Some(Box::new(f)));
        grammer.add("root", "twice?", Some(Box::new(f)));

        if let Ok(branches) = grammer.scan("root", "") {
            assert_eq!(branches[0], 1940);
            assert_eq!(branches[1], 3);
            assert_eq!(branches[2], 10);
        }
        else {
            assert!(false);
        }

        //TODO if let Ok(branches) = grammer.scan("root", "Chuck Norris counted to infinity - twice") {
        if let Ok(branches) = grammer.scan("root", "twice") {
            assert_eq!(branches[0], 1940);
            assert_eq!(branches[1], 3);
            assert_eq!(branches[2], 10);
        }
        else {
            assert!(false);
        }

        if let Ok(_) = grammer.scan("root", "twicetwice") {
            assert!(false);
        }
        else {
            assert!(true);
        }
    }

    #[test]
    fn grammer_none_or_many()
    {
        let f = |_: Vec<i32>, _: &str, _: &mut NoShared| {
            vec![1983, 2, 7]
        };

        let mut grammer: Grammer<i32> = Grammer::new();
        grammer.add("root", "monkey*", Some(Box::new(f)));

        if let Ok(branches) = grammer.scan("root", "") {
            assert_eq!(branches[0], 1983);
            assert_eq!(branches[1], 2);
            assert_eq!(branches[2], 7);
        }
        else {
            assert!(false);
        }

        if let Ok(branches) = grammer.scan("root", "monkey") {
            assert_eq!(branches[0], 1983);
            assert_eq!(branches[1], 2);
            assert_eq!(branches[2], 7);
        }
        else {
            assert!(false);
        }

        if let Ok(branches) = grammer.scan("root", "monkeymonkeymonkey") {
            assert_eq!(branches[0], 1983);
            assert_eq!(branches[1], 2);
            assert_eq!(branches[2], 7);
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn grammer_rule()
    {
        let f = |_: Vec<i32>, _: &str, _: &mut NoShared| {
            vec![7777]
        };

        let mut grammer: Grammer<i32> = Grammer::new();
        grammer.declare(vec!["monkey"]);    // Also testing `declare` here.
        grammer.add("test-a", "<monkey>", None);
        grammer.add("test-b", "<monkey><monkey><monkey>", None);
        grammer.add("test-c", "<monkey>+", None);
        grammer.add("test-d", "<monkey>*", None);
        grammer.add("monkey", "monkey", Some(Box::new(f)));

        if let Ok(_) = grammer.scan("test-a", "ape") {
            assert!(false);
        }
        else {
            assert!(true);
        }

        if let Ok(branches) = grammer.scan("test-a", "monkey") {
            assert_eq!(branches[0], 7777);
        }
        else {
            assert!(false);
        }

        if let Ok(branches) = grammer.scan("test-b", "monkeymonkeymonkey") {
            assert_eq!(branches.len(), 3);
            assert_eq!(branches[0], 7777);
            assert_eq!(branches[1], 7777);
            assert_eq!(branches[2], 7777);
        }
        else {
            assert!(false);
        }

        if let Ok(_) = grammer.scan("test-c", "") {
            assert!(false);
        }
        else {
            assert!(true);
        }

        if let Ok(branches) = grammer.scan("test-c", "monkeymonkeymonkeymonkey") {
            assert_eq!(branches.len(), 4);
            assert_eq!(branches[0], 7777);
            assert_eq!(branches[1], 7777);
            assert_eq!(branches[2], 7777);
            assert_eq!(branches[3], 7777);
        }
        else {
            assert!(false);
        }

        if let Ok(branches) = grammer.scan("test-d", "") {
            assert_eq!(branches.len(), 0);
        }
        else {
            assert!(false);
        }

        if let Ok(branches) = grammer.scan("test-d", "monkeymonkeymonkeymonkey") {
            assert_eq!(branches.len(), 4);
            assert_eq!(branches[0], 7777);
            assert_eq!(branches[1], 7777);
            assert_eq!(branches[2], 7777);
            assert_eq!(branches[3], 7777);
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn grammer_spaces()
    {
        let mut grammer: Grammer<i32> = Grammer::new();
        grammer.add("test-a", "_", None);
        grammer.add("test-b", " ", None);
        grammer.add("test-c", "monkey monkey_monkey", None);

        assert!(grammer.scan("test-a", "").is_err());
        assert!(grammer.scan("test-a", "   ").is_ok());
        assert!(grammer.scan("test-b", "").is_ok());
        assert!(grammer.scan("test-b", "   ").is_ok());
        assert!(grammer.scan("test-c", "monkey     monkey                      monkey").is_ok());
        assert!(grammer.scan("test-c", "monkeymonkey monkey").is_ok());
        assert!(grammer.scan("test-c", "monkey monkeymonkey").is_err());
    }

    #[test]
    fn grammer_misc_calc()
    {
        let mut grammer: Grammer<f64> = Grammer::new();
        grammer.declare(vec!["add", "expr", "mul"]);
        grammer.add("num", "[0-9]+", Some(Box::new(|_, l, _| vec![l.parse().unwrap()])));
        grammer.add("brackets", "\\(<expr>\\)", None);
        grammer.add("mul", "(<num>|<brackets>)(\\*<mul>)?", Some(Box::new(|b, _, _| if b.len() == 1 { vec![b[0]] } else { vec![b[0] * b[1]] } )));
        grammer.add("add", "<mul>(\\+<add>)?", Some(Box::new(|b, _, _| if b.len() == 1 { vec![b[0]] } else { vec![b[0] + b[1]] } )));
        grammer.add("expr", "(<add>|<brackets>)", None);

        if let Ok(branches) = grammer.scan("expr", "2*(3*4*5)") {
            assert_eq!(branches[0], 120f64);
        }
        else {
            assert!(false);
        }

        if let Ok(branches) = grammer.scan("expr", "2*(3+4)*5") {
            assert_eq!(branches[0], 70f64);
        }
        else {
            assert!(false);
        }

        if let Ok(branches) = grammer.scan("expr", "((2+3*4+5))") {
            assert_eq!(branches[0], 19f64);
        }
        else {
            assert!(false);
        }
    }
    


    use abitvin::Rule;

    // TODO Remove me later.
    //struct NoShared {}
    
    #[test]
    fn all()
    {
        let mut dummy = 80085; 
        let code = "abcdefg";
        
        let f = |_: Vec<bool>, l: &str, _: &mut i32| {
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
        
        let f = |_: Vec<u32>, l: &str, _: &mut bool| {
            assert_eq!(l, "abc");
            vec![0u32, 1u32, 2u32, 3u32]
        };
        
        let mut c = Rule::new(None);
        c.all_except(vec!['A', 'B', 'C', 'D']);
        
        let mut r: Rule<u32, bool> = Rule::new(Some(Box::new(f)));
        r.exact(3, c);
        
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
        let code = "\\<東\\<💝\\>中\\>"; // There are gonna be 7 replacements.
        
        let alterations = vec![
            ("\\<", "<"),
            ("\\>", ">"),
            ("東", "AAA"),
            ("💝", "BBB"),
            ("中", "CCC"),
        ];
        
        let mut a = Rule::new(None);
        a.alter(alterations);

        let f = |_: Vec<i32>, l: &str, _: &mut f64| {
            assert_eq!(l, "<AAA<BBB>CCC>");
            vec![111, 222]
        }; 
        
        let mut r: Rule<i32, f64> = Rule::new(Some(Box::new(f)));
        r.exact(7, a);
        
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
        
        let aaa_fn = |_: Vec<i32>, l: &str, _: &mut bool| {
            assert_eq!(l, "aaa");
            vec![111]
        }; 
        
        let bbb_fn = |_: Vec<i32>, l: &str, _: &mut bool| {
            assert_eq!(l, "bbb");
            vec![222]
        };
        
        let ccc_fn = |_: Vec<i32>, l: &str, _: &mut bool| {
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
        any_of_these.any_of(vec![aaa, bbb, ccc]);
        
        let mut root: Rule<i32, bool> = Rule::new(None);
        root.exact(3, any_of_these);
        
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
        
        unsafe {
            if let Ok(branches) = root.at_least_raw(3, &x).scan(&code, &mut dummy) {
                assert_eq!(branches[0], 10);
                assert_eq!(branches[1], 10);
                assert_eq!(branches[2], 10);
                assert_eq!(branches[3], 10);
            }
            else {
                assert!(false);
            }
            
            if let Ok(branches) = root.clear().at_least_raw(4, &x).scan(&code, &mut dummy) {
                assert_eq!(branches[0], 10);
                assert_eq!(branches[1], 10);
                assert_eq!(branches[2], 10);
                assert_eq!(branches[3], 10);
            }
            else {
                assert!(false);
            }
            
            if let Ok(_) = root.clear().at_least_raw(5, &x).scan(&code, &mut dummy) {
                assert!(false);
            }
            else {
                assert!(true);
            }
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
        
        unsafe {
            if let Ok(_) = root.at_most_raw(2, &y).scan(&code, &mut dummy) {
                assert!(false);
            }
            else {
                assert!(true);
            }
            
            if let Ok(branches) = root.clear().at_most_raw(3, &y).scan(&code, &mut dummy) {
                assert_eq!(branches[0], 14);
                assert_eq!(branches[1], 14);
                assert_eq!(branches[2], 14);
            }
            else {
                assert!(false);
            }
            
            if let Ok(branches) = root.clear().at_most_raw(4, &y).scan(&code, &mut dummy) {
                assert_eq!(branches[0], 14);
                assert_eq!(branches[1], 14);
                assert_eq!(branches[2], 14);
            }
            else {
                assert!(false);
            }
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
        
        unsafe {
            if let Ok(branches) = root.between_raw(1, 3, &z).scan(&code, &mut dummy) {
                assert_eq!(branches[0], 34);
                assert_eq!(branches[1], 34);
                assert_eq!(branches[2], 34);
            }
            else {
                assert!(true);
            }
            
            if let Ok(branches) = root.clear().between_raw(0, 10, &z).scan(&code, &mut dummy) {
                assert_eq!(branches[0], 34);
                assert_eq!(branches[1], 34);
                assert_eq!(branches[2], 34);
            }
            else {
                assert!(false);
            }
            
            if let Ok(_) = root.clear().between_raw(4, 5, &z).scan(&code, &mut dummy) {
                assert!(false);
            }
            else {
                assert!(true);
            }
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
        hex.any_of(vec![digit, af]);

        let mut parser: Rule<u32, i32> = Rule::new(Some(Box::new(|b, _, _| 
        {
            let mut m = 1u32;
            let mut n = 0u32;
            
            for i in b.iter().rev() {
                n += i * m;
                m <<= 4;
            }
            
            vec![n]
        })));
        parser.between(1, 8, hex);
        
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
        
        unsafe {
            if let Ok(branches) = root.exact_raw(10, &dot).scan(&code, &mut dummy) {
                assert!(branches.len() == 10 && branches.into_iter().any(|c| c == '.'));
            }
            else {
                assert!(false);
            }
            
            if let Ok(_) = root.clear().exact_raw(9, &dot).scan(&code, &mut dummy) {
                assert!(false);
            }
            else {
                assert!(true);
            }
            
            if let Ok(_) = root.clear().exact_raw(11, &dot).scan(&code, &mut dummy) {
                assert!(false);
            }
            else {
                assert!(true);
            }
            
            if let Ok(branches) = root.clear().exact_raw(0, &nope).exact_raw(10, &dot).exact_raw(0, &nope).scan(&code, &mut dummy) {
                assert!(branches.len() == 10 && branches.into_iter().any(|c| c == '.'));
            }
            else {
                assert!(false);
            }
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
        unsafe { root.maybe_raw(&dots).one(xxx).maybe_raw(&dots); }
        
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
        
        unsafe {
            if let Err(_) = code1.none_or_many_raw(&dot).none_or_many_raw(&x).none_or_many_raw(&dot).scan("", &mut dummy) {
                assert!(false);
            }
            
            if let Err(_) = code2.none_or_many_raw(&dot).none_or_many_raw(&x).none_or_many_raw(&dot).scan("x", &mut dummy) {
                assert!(false);
            }
            
            if let Err(_) = code3.none_or_many_raw(&dot).none_or_many_raw(&x).none_or_many_raw(&dot).scan("..", &mut dummy) {
                assert!(false);
            }
            
            if let Err(_) = code4.none_or_many_raw(&dot).none_or_many_raw(&x).none_or_many_raw(&dot).scan("xx.", &mut dummy) {
                assert!(false);
            }
            
            if let Err(_) = code5.none_or_many_raw(&dot).none_or_many_raw(&x).none_or_many_raw(&dot).scan("..xx", &mut dummy) {
                assert!(false);
            }
        }
    }
    
    #[test]
    fn not()
    {
        let mut dummy = 0;
        
        let mut not_this = Rule::new(None);
        not_this.literal("not this");
        
        let mut r: Rule<i32, i32> = Rule::new(None);
        r.literal("aaa").not(not_this).literal("bbb").literal("ccc");
        
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
        root.one(one).one(two).one(three);
        
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

        let mut a: Rule<i32, Shared> = Rule::new(Some(Box::new(|_: Vec<i32>, _: &str, s: &mut Shared| {
            s.number = 123; 
            Vec::new()
        })));
        a.literal("a");

        let mut b: Rule<i32, Shared> = Rule::new(Some(Box::new(|_: Vec<i32>, _: &str, s: &mut Shared| {
            s.number = 456777; 
            Vec::new() 
        })));
        b.literal("b");
        
        let mut c: Rule<i32, Shared> = Rule::new(Some(Box::new(|_: Vec<i32>, _: &str, s: &mut Shared| { 
            s.number = -999;
            Vec::new()
        })));
        c.literal("c");

        let mut failed: Rule<i32, Shared> = Rule::new(Some(Box::new(|_: Vec<i32>, _: &str, s: &mut Shared| { 
            s.number = 123456;
            Vec::new()
        })));
        failed.literal("---");

        if let Ok(_) = a.scan("a", &mut shared) {
            assert_eq!(shared.number, 123);
        }
        else {
            assert!(false);
        }

        if let Ok(_) = b.scan("b", &mut shared) {
            assert_eq!(shared.number, 456777);
        }
        else {
            assert!(false);
        }

        if let Ok(_) = c.scan("c", &mut shared) {
            assert_eq!(shared.number, -999);
        }
        else {
            assert!(false);
        }

        if let Ok(_) = failed.scan("xxx", &mut shared) {
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
            num.at_least(1, digit);

            let mut brackets = Rule::new(None);
            brackets.literal("(").one_raw(&expr).literal(")"); 

            let mut mul_right = Rule::new(None);
            mul_right.literal("*").one_raw(&mul);
            mul.any_of_raw(vec![&num, &brackets]).maybe(mul_right);

            let mut add_right = Rule::new(None);
            add_right.literal("+").one_raw(&add);
            add.one_raw(&mul).maybe(add_right);

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