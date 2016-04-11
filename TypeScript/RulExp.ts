///<reference path="Rule.ts"/>

namespace Abitvin
{
    // all:         .
    // allExcept:   ^char
    // alter:       [x,y]
    // atLeast:     <rule>{n,}
    // atLeastOne:  <rule>+
    // atMost:      <rule>{,m}
    // anyOf:       (literal|<rule>|<rule>)
    // between:     <rule>{n,m}
    // between:     [a-z]
    // eof:         EOF
    // exact:       <rule>{n}
    // maybe:       <rule>?
    // literal:     text
    // noneOrMany:  <rule>*
    // one:         <rule>
    
    
    
    
    interface IEmpty {}
    
    class R<TB, TM> extends Rule<Rule<TB, TM>, IEmpty> {}
    
    interface IRule<TBranch, TMeta>
    {
        id: string;
        isDefined: boolean;
        rule: Rule<TBranch, TMeta>;
    }
    
    export class RulExp<TBranch, TMeta> 
    {
        private _grammer: R<TBranch, TMeta>;
        private _rules: {[name: string]: IRule<TBranch, TMeta>};
        
        constructor()
        {
            const statement = new R<TBranch, TMeta>();
            
            const char = new R<TBranch, TMeta>().between("a", "z");
            const CHAR = new R<TBranch, TMeta>().between("A", "Z");
            const Char = new R<TBranch, TMeta>().anyOf(char, CHAR);
            //const digit = new R().between("0", "9");
            //const anyChar = new R().all();
            
            //const all = new R(() => [new Rule().all()]).literal(".");
            //const allExcept = new R().literal("^").one(anyChar);
            //const eof = new R().literal("EOF");
            
            // Literal
            const literal = new R<TBranch, TMeta>((b, l) => [new Rule<TBranch, TMeta>().literal(l)]).atLeast(1, Char);
            
            // One rule
            const ruleFn = (b, l) =>
            {
                const id = l.slice(1,-1);
                const r = this._rules[id];
                
                if (r == null)
                    throw new Error(`Rule "${id}" not found.`);
                    
                return [new Rule<TBranch, TMeta>().one(r.rule)];
            };
            
            //const ranges = new R<TBranch, TMeta>(); 
            
            // TODO add ranges (*, +, [n,m]) after this rule to fix the atLeastOne stack overflow.
            const rule = new R<TBranch, TMeta>(ruleFn).literal("<").atLeast(1, Char).literal(">"); //.maybe(ranges);
            
            // At least one
            //const atLeastOneFn = (b, l) => [new Rule<TBranch, TMeta>().atLeast(1, b[0])];
            //const atLeastOne = new R<TBranch, TMeta>(atLeastOneFn).one(rule /* TODO if we use `statement` here we get a stack overflow (which is logical). */).literal("+");
            //const atMost = new R().one(rule).literal("{,").atLeast(1, digit).literal("}");
            //const atLeast = new R().one(rule).literal("{").atLeast(1, digit).literal(",}");
            //const between = new R().one(rule).literal("{").atLeast(1, digit).literal(",").atLeast(1, digit).literal("}");
            //const maybe = new R().one(rule).literal("?");
            //const noneOrMany = new R().one(rule).literal("*");
            
            // Any of
            //const anyOfFn = (b, l) => [new Rule<TBranch, TMeta>().anyOf(b)];
            //const more = new R<TBranch, TMeta>().literal("|").anyOf(statement);
            //const anyOf = new R<TBranch, TMeta>(anyOfFn).literal("(").anyOf(statement).noneOrMany(more).literal(")");
            
            //const statement = new R().anyOf(all, allExcept, eof, atMost, atLeast, between, maybe, noneOrMany);
            //statement.anyOf(anyOf, atLeastOne, literal, rule);
            statement.anyOf(literal, rule);
            
            this._grammer = new R<TBranch, TMeta>().noneOrMany(statement);
            this._rules = {};
        }
        
        public add(id: string, expr: string, branchFn: BranchFn<TBranch> = null, meta: TMeta = null): void
        {
            const r: IRule<TBranch, TMeta> = this._rules[id];
            
            if (r != null && r.isDefined)
                throw new Error(`The rule "${id}" already used.`);
            
            const result = this._grammer.scan(expr); 
                    
            // TODO Show nice errors.
            if (!result.isSuccess)
                throw new Error("Error compiling rule expression.");
            
            if (r == null)
            {
                let rule = null;
                
                if (result.branches.length === 1)
                {
                    rule = result.branches[0];
                }
                else if (result.branches.length > 1)
                {
                    rule = new Rule<TBranch, TMeta>();
                    
                    for (const rr of result.branches)
                        rule.one(rr);
                }
                else
                {
                    throw new Error("Application error.");
                }
            
                rule.branchFn = branchFn;
                rule.meta = meta;
                
                this._rules[id] = {
                    id: id,
                    isDefined: true,
                    rule: rule
                };
            }
            else
            {
                r.isDefined = true;
                
                for (const rr of result.branches)
                    r.rule.one(rr);
                    
                r.rule.branchFn = branchFn;
                r.rule.meta = meta;
            }
        }
        
        public declare(...ids: string[]): void
        {
            for (const id of ids)
            {
                if (this._rules[id] != null)
                    throw new Error(`The rule "${id}" already used.`);
                
                this._rules[id] = {
                    id: id,
                    isDefined: false,
                    rule: new Rule<TBranch, TMeta>()
                };
            }
        }
        
        public scan(rootId: string, code: string): RuleResult<TBranch, TMeta>
        {
            const root: IRule<TBranch, TMeta> = this._rules[rootId];
            
            if (root == null)
                throw new Error(`Rule with id "${rootId}" not found.`);
             
            return root.rule.scan(code);
        }
    }
    
    
    const grammer = new RulExp<number, IEmpty>();
    
    grammer.declare("one", "two", "three");
    grammer.add("bla", "<one>two<two>three<three>three");
    grammer.add("tosti", "<bla><bla>");
    //grammer.add("anynumber", "(<one>+|<two>+|<three>+)");
    grammer.add("one", "one", () => [1]);
    grammer.add("two", "two", () => [2]);
    grammer.add("three", "three", () => [3]);
    //grammer.add("bla", "");
    
    
    //console.log(grammer.scan("one", "one"));
    //console.log(grammer.scan("two", "two"));
    //console.log(grammer.scan("three", "three"));
    //console.log(grammer.scan("anynumber", "one"));
    //console.log(grammer.scan("anynumber", "twotwo"));
    //console.log(grammer.scan("anynumber", "threethreethree"));
    console.log(grammer.scan("tosti", "onetwotwothreethreethreeonetwotwothreethreethree"));
    
    const one  = new Rule<number, IEmpty>(() => [1]).literal("one");
    const two  = new Rule<number, IEmpty>(() => [2]).literal("two");
    const three  = new Rule<number, IEmpty>(() => [3]).literal("three");
    const anyNumber = new Rule<number, IEmpty>().anyOf(one, two, three);
    const root = new Rule<number, IEmpty>().one(anyNumber);
    
    
    //console.log(root.scan("three"));
    
    
    /*
    
    const grammer = new RulExp<number, number>();
    grammer.add("digit", "[0-9]");
    grammer.add("integer", "<digit>+");
    grammer.add("char", "[a-z]");
    grammer.add("CHAR", "[A-Z]");
    grammer.add("Char", "(<char>|<CHAR>)");
    
    return grammer.scan(code);
    
    
    */
    
    
    /*
    const digit = new RulExp("[0-9]");
    const integerA = new RulExp("[0-9]+");
    const integerB = new RulExp("{digit}+");
    const char = new RulExp("[a-z]");
    const CHAR = new RulExp("[A-Z]");
    const Char = new RulExp("[a-zA-Z]");
    */
    
        
    
}


/*
        public all(): this
        
        public allExcept(...list: string[]): this
        public allExcept(list: string[]): this
        public allExcept(arg1: any): this
		
		public alter(...list: string[]): this
        public alter(list: string[]): this
        public alter(arg1: any): this
		
		public atLeast(count: number, rule: Rule<TBranch, TMeta>): this
		public atLeast(count: number, text: string): this
		public atLeast(count: number, arg2: any): this
		
        public atMost(count: number, rule: Rule<TBranch, TMeta>): this
		public atMost(count: number, text: string): this
		public atMost(count: number, arg2: any): this
		
        public anyOf(...rules: Rule<TBranch, TMeta>[]): this
        public anyOf(rules: Rule<TBranch, TMeta>[]): this
		public anyOf(...literals: string[]): this
        public anyOf(literals: string[]): this
		public anyOf(arg1: any): this
		
		public between(min: number, max: number, rule: Rule<TBranch, TMeta>): this
        public between(charA: string, charB: string, notUsed?: any): this
        public between(arg1: any, arg2: any, arg3: any): this
		
        public eof(): this
        
        public exact(count: number, rule: Rule<TBranch, TMeta>): this
		public exact(count: number, text: string): this
		public exact(count: number, arg2: any): this
		
		public maybe(rule: Rule<TBranch, TMeta>): this
		public maybe(text: string): this
		public maybe(arg1: any): this
		
		public literal(text: string): this
		
        public noneOrMany(rule: Rule<TBranch, TMeta>): this
		public noneOrMany(text: string): this
		public noneOrMany(arg1: any): this
		
		public one(rule: Rule<TBranch, TMeta>): this
		
*/