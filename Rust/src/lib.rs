// Copyright (c) 2015-2016 Abitvin <foss@abitvin.net>
// Licensed under the MIT license <LICENSE.md or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

mod abitvin
{
    // TODO What is the best way to store the branch closure?
    // http://stackoverflow.com/questions/27831944/how-do-i-store-a-closure-in-rust
    // TODO Simplify `Option<Box<Fn<...>>>` if we can.
    pub type BranchFn<T> = Option<Box<Fn(&Vec<T>, &str) -> Vec<T>>>;
    
    pub struct Rule<'a, T> {
        branch_fn: BranchFn<T>,
        parts: Vec<ScanFn<'a>>,
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
    
    enum ScanFn<'a> {
        All,
        AllExcept(&'a Vec<char>),
        Alter(&'a Vec<(&'static str, &'static str)>),
        Eof,
        Literal(&'static str),
    }
    
    use std::str::Chars;
    
    struct ScanCtx<'a, T> {
        branches: Vec<T>,
        code_iter: Chars<'a>,
        errors: Vec<RuleError>,
        has_eof: bool,
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
        
        pub fn literal(&mut self, text: &'static str) -> &mut Self
        {
            if text.len() < 1 {
                panic!("Literal text must at least 1 character long.");
            }
                
            self.parts.push(ScanFn::Literal(&text));
            self
        }
        
        // TODO I think `code` needs to be a String 
        pub fn scan(&self, code: &'static str) -> Result<Vec<T>, Vec<RuleError>>
        {
            let mut ctx: ScanCtx<T> = ScanCtx {
                branches: Vec::new(),
                code_iter: code.chars(),
                errors: Vec::new(),
                has_eof: false,
                index: 0,
                lexeme: String::new(),
            };
            
            if self.run(&mut ctx) == -1 {
                // TODO //return RuleResult.failed<TBranch, TMeta>(ctx.errors);
                
                /*return Err(RuleError {
                    index: 0i64,
                    msg: String::from("Not implemented yet."),
                });*/
                
                return Err(ctx.errors);
            }
			
            /*if (ctx.hasEof)
                ctx.index--;
            
            if (ctx.index !== ctx.code.length)
                return RuleResult.failed<TBranch, TMeta>(ctx.errors);
			
			return RuleResult.success<TBranch, TMeta>(ctx.branches);
            */
            
            /*Err(RuleError {
                index: 0u64,
                msg: "Not implemented yet.",
            })*/
            
            Ok(ctx.branches)
        }
        
        // Private functions
        
        fn branch(&'a self, ctx: &ScanCtx<'a, T> /*, isRootOfRule: bool*/) -> ScanCtx<T>
        {
            let new_ctx: ScanCtx<T> = ScanCtx {
                branches: Vec::new(),
                //code: ctx.code,
                code_iter: ctx.code_iter.clone(),
                has_eof: ctx.has_eof,
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
        
        fn merge(&'a self, target: &mut ScanCtx<'a, T>, source: &mut ScanCtx<'a, T> /*, isRootOfRule: bool = false*/) -> i64
        {
            /* TODO
            if (isRootOfRule)
                while (source.metaPushed-- > 0)
                    source.trail.pop();
            */
            
            let step = source.index - target.index;
            
			target.code_iter = source.code_iter.clone();
            target.errors = source.errors.clone();
            target.has_eof = source.has_eof;
            target.index = source.index;
            target.lexeme.push_str(&source.lexeme.to_string());
            // TODO target.metaPushed = 0;
            // TODO target.trail = source.trail;
           
            match self.branch_fn {
                /* TODO isRootOfRule &&*/ Some(ref f) => {
                    target.branches.append(&mut f(&source.branches, &source.lexeme));
                },
                None => {
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
            
            let mut new_ctx = self.branch(&ctx /* TODO, true*/);
            
            for p in &self.parts {
                let r = match *p {
                    ScanFn::All => self.scan_all_leaf(&mut new_ctx),
                    ScanFn::AllExcept(ref exclude) => self.scan_all_except_leaf(&exclude, &mut new_ctx),
                    ScanFn::Alter(ref alter) => self.scan_alter_leaf(&alter, &mut new_ctx),
                    ScanFn::Eof => self.scan_eof(&mut new_ctx),
                    ScanFn::Literal(find) => self.scan_literal_leaf(&find, &mut new_ctx),
                };
                
                if r == -1 {
                    return -1;
                }
            }
            
            self.merge(&mut ctx, &mut new_ctx/* TODO ,true */)
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
        
        fn scan_eof(&self, mut ctx: &mut ScanCtx<T>) -> i64
        {
            if let None = ctx.code_iter.next() {
                ctx.has_eof = true;
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
        
        let f = |_: &Vec<bool>, l: &str|
        {
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
        
        let f = |_: &Vec<u32>, l: &str|
        {
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
        
        let f = |_: &Vec<i32>, l: &str|
        {
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
    fn test_literal()
    {
        let code = "y̆y̆y̆x̆";
        
        let mut r: Rule<u64> = Rule::new(Some(Box::new(|_, _| vec![7777u64, 8888u64, 9999u64] )));
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
    fn test2()
    {
        let v = vec![
            ("aaa", "AAA"),
            ("bbb", "BBB"),
            ("ccc", "CCC"),
        ];
        
        let a = &"aaabbb"[0..3];
        let b = &"bbbaaa"[3..6];
        assert_eq!(a, b);
        
        unsafe {
            let a = String::from("aaabbb");
            let c = a.slice_unchecked(0, 3);
            
            let b = String::from("bbbaaa");
            let d = b.slice_unchecked(3, 6);
            
            assert_eq!(c, d);
            assert_eq!(v[0].0, c);
            assert_eq!(v[0].0, d);
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
        
        public static get version(): string { return "0.5.2"; }
        public set branchFn(value: BranchFn<TBranch>) { this._branchFn = value; }
        public get meta(): TMeta { return this._meta; }
        public set meta(value: TMeta) { this._meta = value; }
       


		public atLeast(count: number, rule: Rule<TBranch, TMeta>): this
		public atLeast(count: number, text: string): this
		public atLeast(count: number, arg2: any): this
		{
            if (!this.isInteger(count))
                throw new Error("First argument is not an integer.");
                
            if (count < 0)
                throw new Error("Count cannot be negative.");
                
            if (this.isString(arg2))
                this._parts.push(this.scanRuleRange.bind(this, count, Number.POSITIVE_INFINITY, new Rule<TBranch, TMeta>().literal(arg2)));
			else if (this.isRule(arg2))
				this._parts.push(this.scanRuleRange.bind(this, count, Number.POSITIVE_INFINITY, arg2));
            else
                throw new Error("Second argument is not a rule or a string.");

			return this;
		}
        
        public atMost(count: number, rule: Rule<TBranch, TMeta>): this
		public atMost(count: number, text: string): this
		public atMost(count: number, arg2: any): this
		{
            if (!this.isInteger(count))
                throw new Error("First argument is not an integer.");
            
            if (count < 0)
                throw new Error("Count cannot be negative.");
                
            if (this.isString(arg2))
                this._parts.push(this.scanRuleRange.bind(this, 0, count, new Rule<TBranch, TMeta>().literal(arg2)));
			else if (this.isRule(arg2))
				this._parts.push(this.scanRuleRange.bind(this, 0, count, arg2));
            else
                throw new Error("Second argument is not a rule or a string.");

			return this;
		}

        public anyOf(...rules: Rule<TBranch, TMeta>[]): this
        public anyOf(rules: Rule<TBranch, TMeta>[]): this
		public anyOf(...literals: string[]): this
        public anyOf(literals: string[]): this
		public anyOf(arg1: any): this
		{
            const items: (Rule<TBranch, TMeta>|string)[] = this.getVariadicArray<Rule<TBranch, TMeta>|string>(arguments);
            
            if (this.isString(items[0]))
            {
                if (items.some(i => !this.isString(i)))
                    throw new Error("Not all the items in `anyOf` are a string.");
                
                this._parts.push(this.scanAnyOf.bind(this, (<string[]>items).map(l => new Rule<TBranch, TMeta>().literal(l))));
            }
			else if (this.isRule(items[0]))
            {
                if (items.some(i => !this.isRule(i)))
                    throw new Error("Not all the items in `anyOf` are a rule.");
                    
                this._parts.push(this.scanAnyOf.bind(this, items));
            }
			else
            {
                throw new Error("The items in `anyOf` can only be a string or a rule.");
            }	

			return this;
		}

		public between(min: number, max: number, rule: Rule<TBranch, TMeta>): this
        public between(charA: string, charB: string, notUsed?: any): this
        public between(arg1: any, arg2: any, arg3: any): this
		{
            if (this.isString(arg1))
            {
                if (arg1.length !== 1)
                    throw new Error("First argument can only be a one character string.");
                
                if (!this.isString(arg2) || arg2.length !== 1)
                    throw new Error("Second argument can only be a one character string.");
                
                this._parts.push(this.scanCharRangeLeaf.bind(this, arg1.charCodeAt(0), arg2.charCodeAt(0)));
            }
            else if(this.isInteger(arg1))
            {
                if (!this.isInteger(arg2))
                    throw new Error("Second argument is not an integer.");
                    
                if (!this.isRule(arg3))
                    throw new Error("Third argument is not a rule.");
                    
                this._parts.push(this.scanRuleRange.bind(this, arg1, arg2, arg3));
            }
            else
            {
                throw new Error("First argument is not an integer or a one character string.");
            }
                
			return this;
		}
        
        public exact(count: number, rule: Rule<TBranch, TMeta>): this
		public exact(count: number, text: string): this
		public exact(count: number, arg2: any): this
		{
            if (!this.isInteger(count))
                throw new Error("First argument is not an integer.");
                
            if (count < 0)
                throw new Error("Count cannot be negative.");
            
            if (this.isString(arg2))
                this._parts.push(this.scanRuleRange.bind(this, count, count, new Rule<TBranch, TMeta>().literal(arg2)));
			else if(this.isRule(arg2))
				this._parts.push(this.scanRuleRange.bind(this, count, count, arg2));
            else
                throw new Error("Second argument is not a string or a rule.");
            
			return this;
		}

        public literal(text: string): this
		{
            if (!this.isString(text) || text.length < 1)
                throw new Error("Literal text must be a string of at least 1 character.");
            
			this._parts.push(this.scanLiteralLeaf.bind(this, text));
			return this;
		}
        
		public maybe(rule: Rule<TBranch, TMeta>): this
		public maybe(text: string): this
		public maybe(arg1: any): this
		{
			if (this.isString(arg1))
                this._parts.push(this.scanRuleRange.bind(this, 0, 1, new Rule<TBranch, TMeta>().literal(arg1)));
			else if(this.isRule(arg1))
				this._parts.push(this.scanRuleRange.bind(this, 0, 1, arg1));
            else
                throw new Error("Argument is not a string or a rule.");

			return this;
		}

		public noneOrMany(rule: Rule<TBranch, TMeta>): this
		public noneOrMany(text: string): this
		public noneOrMany(arg1: any): this
		{
            if (this.isString(arg1))
			    this._parts.push(this.scanRuleRange.bind(this, 0, Number.POSITIVE_INFINITY, new Rule<TBranch, TMeta>().literal(arg1)));
            else if(this.isRule(arg1))
			    this._parts.push(this.scanRuleRange.bind(this, 0, Number.POSITIVE_INFINITY, arg1));
            else
                throw new Error("Argument is not a string or a rule.");
                
			return this;
		}	
        
        public not(rule: Rule<TBranch, TMeta>): this
        public not(text: string): this
        public not(arg1: any): this
        {
            if (this.isString(arg1))
                this._parts.push(this.scanNot.bind(this, new Rule<TBranch, TMeta>().literal(arg1)));
            else if (this.isRule(arg1))
                this._parts.push(this.scanNot.bind(this, arg1));
            else
                throw new Error("Argument is not a string or a rule.");
            
            return this;
        }

		public one(...rules: Rule<TBranch, TMeta>[]): this
		{
            for (const r of rules)
            {
                if(!this.isRule(r))
                    throw new Error("Argument is not a rule.");
                
                this._parts.push(this.scanRuleRange.bind(this, 1, 1, r));
            }
            
			return this;
		}

		public scan(code: string): RuleResult<TBranch, TMeta>
		{
            const ctx: IScanContext<TBranch, TMeta> = {
                branches: [],
                code: code,
                hasEof: false,
				errors: [],
                index: 0, 
                lexeme: "",
                metaPushed: 0,
                trail: []
            };

			if (this.run(ctx) === -1)
				return RuleResult.failed<TBranch, TMeta>(ctx.errors);
            
            if (ctx.hasEof)
                ctx.index--;
            
            if (ctx.index !== ctx.code.length)
                return RuleResult.failed<TBranch, TMeta>(ctx.errors);
			
			return RuleResult.success<TBranch, TMeta>(ctx.branches);
		}

		private scanAnyOf(rules: Rule<TBranch, TMeta>[], ctx: IScanContext<TBranch, TMeta>): number
		{
            const c: number = rules.length;

            for (let i: number = 0; i < c; i++)
            {
                const rule: Rule<TBranch, TMeta> = rules[i];
                const newCtx: IScanContext<TBranch, TMeta> = this.branch(ctx, false);
                
                if (rule.run(newCtx) !== -1)
                    return this.merge(ctx, newCtx);
            }

            return -1;
		}

		private scanCharRangeLeaf(codeA: number, codeB: number, ctx: IScanContext<TBranch, TMeta>): number
		{
            const char: string = ctx.code[ctx.index];
            
            if (char == null)
                return this.updateError(ctx, `End of code. Expected a character between '${String.fromCharCode(codeA)}' and '${String.fromCharCode(codeB)}'.`);
                
            const code: number = char.charCodeAt(0);
            
            if (code < codeA || code > codeB)
                return this.updateError(ctx, `Expected a character between '${String.fromCharCode(codeA)}' and '${String.fromCharCode(codeB)}'; got a '${char}'.`);
                
            ctx.lexeme += char;
            ctx.index++;
            return 1;
		}
        
        private scanNot(rule: Rule<TBranch, TMeta>, ctx: IScanContext<TBranch, TMeta>): number
        {
            if (rule.run(this.branch(ctx, false)) === -1)
                return 0;
            else
                return -1;
        }

		private scanRuleRange(min: number, max: number, rule: Rule<TBranch, TMeta>, ctx: IScanContext<TBranch, TMeta>): number
		{
            const newCtx: IScanContext<TBranch, TMeta> = this.branch(ctx, false);
            let count: number = 0;
            let progress: number;
            
            while ((progress = rule.run(newCtx)) !== -1)
            {
                if (progress === 0)
                    return 0;
                
                if (++count === max)
                    break;
            }
            
            if (count >= min && count <= max)
                return this.merge(ctx, newCtx);
            
            return -1;
        }
        
		private showCode(text: string, position: number): void
        {
            console.error(text.substr(position, 40));
        }
	}
}
*/