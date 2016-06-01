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
        AllExcept(Vec<char>),   // TODO Shouldn't we be doing &Vec<char>?
        Alter(&'a Vec<(&'static str, &'static str)>),
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
        
        pub fn all_except(&mut self, exclude: Vec<char>) -> &mut Self
        {
            if exclude.len() == 0 {
                panic!("List of excluded characters is empty.");
            }
            
            self.parts.push(ScanFn::AllExcept(exclude));
            self
        }
        
        /*
        public alter(...list: string[]): this
        public alter(list: string[]): this
        public alter(arg1: any): this
		{
            const list: string[] = this.getVariadicArray<string>(arguments); 
            
            if (list.length === 0)
                throw new Error("No arguments given for `allExcept`.");
            
            if (list.some(i => !this.isString(i) || i.length === 0))
                throw new Error("An argument in `allExcept` can only be a string with minimal one character.");
            
            if (list.length % 2 === 1)
                throw new Error("Alter list must be a factor of 2.");

			this._parts.push(this.scanAlterLeaf.bind(this, list));
			return this;
		}
        */
        
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
        
        let f = |b: &Vec<bool>, l: &str|
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
        /* TODO
        let code = "abcdefg";
        
        let f = |b: &Vec<bool>, l: &str|
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
        */
    }
    
    #[test]
    fn test_alter()
    {
        let code = "aaabbbccc";
        
        let alterThese = vec![
            ("aaa", "AAA"),
            ("bbb", "BBB"),
            ("ccc", "CCC"),
        ];
        
        let f = |b: &Vec<i32>, l: &str|
        {
            assert_eq!(l, "AAABBBCCC");
            vec![111, 222]
        }; 
        
        let mut r: Rule<i32> = Rule::new(Some(Box::new(f)));
        r.alter(&alterThese).alter(&alterThese).alter(&alterThese);
        
        if let Ok(branches) = r.scan(&code) {
            assert_eq!(branches[0], 111);
            assert_eq!(branches[1], 222);
        }
        else {
            assert!(false);
        }
    }
    
    #[test]
    fn test_literal()
    {
        //let code = String::from("sevensevensevenseven");
        //let code = "sevensevensevenseven";
        let code = "y̆y̆y̆x̆";
        
        let mut r: Rule<u64> = Rule::new(Some(Box::new(|b, l| vec![7777u64, 8888u64, 9999u64] )));
        r.literal("y̆").literal("y̆").literal("y̆").literal("x̆");
        
        let res = r.scan(&code);
        
        match res {
            Ok(branches) => {
                println!("Ok!");
                
                for i in branches {
                    println!("{}", i);
                }
                
                assert!(true);
            }
            Err(errors) => {
                for e in errors {
                    println!("Error: {}", e.msg);
                }
                
                assert!(false);
            }
        }
    }
    
    
    
    
    
    
    
    struct Point {
        x: f64,
        y: f64
    }
    
    fn do_bla6(branches: &Vec<Point>, lexeme: &str) -> Vec<Point>
    {
        println!("{}", lexeme);
        
        let mut v = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 1.1, y: 3.3 },
            Point { x: 2.2, y: 6.6 }
        ];
        
        for i in branches {
            v.push(Point { x: i.x, y: i.y });
        }
        
        v
    }
    
    fn compare_str() -> bool
    {
        //let hello3 = "Hello";
        //let iter3 = hello3.chars();
        
        let hello = String::from("Hello, 日本人 y̆!");
        /*//let hello2 = String::from("Hello, 日");
        
        let mut slice = hello.chars().as_str();
        //let mut slice2 = hello2.chars();
        
        // is_char_boundary
        // to_uppercase
        // nth
        
        */
        
        let bytes = hello.into_bytes();
        
        
        
        
        true
    }
    
    
    /*
        private scanAlterLeaf(list: string[], ctx: IScanContext<TBranch, TMeta>): number
		{
            for (let i = 0; i < list.length; i += 2)
            {
                const find: string = list[i];
                const len: number = find.length;

                if (find === ctx.code.substr(ctx.index, len))
                {
                    ctx.lexeme += list[i+1];
                    ctx.index += len;
                    return len;
                }
            }
            
            return this.updateError(ctx, "Alter characters not found on this position.");
		}
        */
        
    
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
        
        
        
        
        
        /*let hello = String::from("Hello, 日本人 y̆!");
        let hello2 = String::from("Hello, 日");
        
        let mut iter = hello.chars();
        let mut iter2 = hello2.chars();
        
        let l = hello.len();
        let mut i = 0;
        
        while i < l 
        {
            let a = iter.next();
            let b = iter2.next();
            let c = a == b;
            
            println!("--------");
            
            if let Some(aa) = a 
            {
                println!("{}: {}", aa, aa.len_utf8());
                i += aa.len_utf8();
            }
            else 
            {
                break;
            }
            
            if let Some(bb) = b {
                println!("{}", bb);
            }
            
            println!("{}", c);
        }*/
        
        /*
        //let hello = String::from("Hello, 日本人 y̆!");
        //let hello2 = "Hello, 日本人!";
        let hello = String::from("abcdefghijklmnop");
        
        let mut iter = hello.chars();
        iter.next();
        iter.next();
        
        if let Some(c) = iter.next() {
            println!("{}", c);
        }
        
        let mut iter2 = iter.clone();
        
        iter.next();
        
        iter2.next();
        iter2.next();
        iter2.next();
        
        if let Some(c) = iter.next() {
            println!("{}", c);
        }
        
        if let Some(c) = iter2.next() {
            println!("{}", c);
        }
        */
        
        /*let mut i = 0;
        
        for (j, c) in hello.chars().enumerate()
        {
            i += 1;
            println!("{}: {}: {}", i, j, c);
        }
        
        println!("---");
        i = 0;
        
        for (j, u) in hello.encode_utf16().enumerate()
        {
            i += 1;
            println!("{}: {}: {}", i, j, u);
        }*/
    }
    
    #[test]
    fn test()
    {
        let mut r: Rule<u64> = Rule::new(Some(Box::new(|branches, lexeme|
        {
            println!("{}", lexeme);
            
            let mut v: Vec<u64> = Vec::new();
            
            v.push(0u64);
            v.push(1u64);
            v.push(2u64);
            
            for i in branches {
                v.push(i.clone());
            }
            
            v
        })));
        
        // TODO Calling `.all()` after `new()` gives compiler errors.
        r.all().literal("bla").all().literal("buz");
        
        /*let x = r.scan(vec![8888u64]);
        
        for i in x {
            println!("{}", i);
        }*/
        
        let r: Rule<Point> = Rule::new(Some(Box::new(do_bla6)));
        /*let x = r.scan(vec![Point { x: 0.777, y: 0.9999 }]);
        
        for i in x {
            println!("Point {}, {}", i.x, i.y);
        }*/
        
        
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
        
        public clear(): this
        {
            this._parts = [];
            return this;
        }
        
        public eof(): this
        {
            this._parts.push(this.scanEofLeaf.bind(this));
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
        
        private scanEofLeaf(ctx: IScanContext<TBranch, TMeta>): number
        {
            if (ctx.index === ctx.code.length)
            {
                ctx.hasEof = true;
                ctx.index++;
                return 1;
            }
            
            return this.updateError(ctx, "No EOF on this position.");
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