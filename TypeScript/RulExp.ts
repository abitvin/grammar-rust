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
    
    
    const enum RangeType
    {
        NoRangeType,
        AtLeast,
        //AtMost,
        //Between,
        //Exact,
        //NoneOrMany
    }
    
    interface IParseContext<TB, TM>
    {
        arg1: number;   // Range argument 1
        arg2: number;   // Range argument 2
        arg3: string;   // Text or id
        rangeType: RangeType;
        rule: Rule<TB, TM>;
    }
    
    interface IEmpty {}
    
    class R<TB, TM> extends Rule<IParseContext<TB, TM>, IEmpty> {}
    
    interface IRule<TBranch, TMeta>
    {
        id: string;
        isDefined: boolean;
        rule: Rule<TBranch, TMeta>;
    }
    
    export class RulExp<TBranch, TMeta> 
    {
        private _grammer: R<TBranch, TMeta>;
        private _rulexps: {[name: string]: IRule<TBranch, TMeta>};
        
        constructor()
        {
            const ranges = new R<TBranch, TMeta>();
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
            const literalTextFn = (b, l) => [{
                arg1: null, 
                arg2: null,
                arg3: l, 
                rangeType: RangeType.NoRangeType, 
                rule: null 
            }];
            
            const literalText = new R<TBranch, TMeta>(literalTextFn).atLeast(1, Char);
            
            const literalFn = (b, l) =>
            {
                const text = b[0].arg3;
                let rule = new Rule<TBranch, TMeta>().literal(text);
                
                if (b.length === 2) 
                    switch(b[1].rangeType)
                    {
                        case RangeType.AtLeast:
                        {
                            rule = new Rule<TBranch, TMeta>().atLeast(b[1].arg1, rule);
                            break;
                        }
                        
                        default:
                            throw new Error("Not implemented.");
                    }
                    
                return [{ 
                    arg1: null,
                    arg2: null,
                    arg3: null,
                    rangeType: RangeType.NoRangeType,
                    rule: rule 
                }];
            };
            
            const literal = new R<TBranch, TMeta>(literalFn).one(literalText).maybe(ranges);
            
            // One rule
            const ruleNameFn = (b, l) => [{
                arg1: null, 
                arg2: null,
                arg3: l, 
                rangeType: RangeType.NoRangeType, 
                rule: null 
            }];
            
            const ruleName = new R<TBranch, TMeta>(ruleNameFn).atLeast(1, Char);
            
            const ruleFn = (b, l) =>
            {
                const id = b[0].arg3;
                const r = this._rulexps[id];
                
                if (r == null)
                    throw new Error(`Rule "${id}" not found.`);
                
                let rule = null;
                
                if (b.length === 1)
                {
                    rule = new Rule<TBranch, TMeta>().one(r.rule); 
                }
                else switch(b[1].rangeType)
                {
                    case RangeType.AtLeast:
                    {
                        rule = new Rule<TBranch, TMeta>().atLeast(b[1].arg1, r.rule);
                        break;
                    }
                    
                    default:
                        throw new Error("Not implemented.");
                }
                    
                return [{ 
                    arg1: null,
                    arg2: null,
                    arg3: null,
                    rangeType: RangeType.NoRangeType,
                    rule: rule 
                }];
            };
             
            const rule = new R<TBranch, TMeta>(ruleFn).literal("<").one(ruleName).literal(">").maybe(ranges);
            
            // At least one
            const atLeastOneFn = (b, l) => [{
                arg1: 1,
                arg2: null,
                arg3: null,
                rangeType: RangeType.AtLeast,
                rule: null
            }];
            
            const atLeastOne = new R<TBranch, TMeta>(atLeastOneFn).literal("+");
            
            //const atMost = new R().one(rule).literal("{,").atLeast(1, digit).literal("}");
            //const atLeast = new R().one(rule).literal("{").atLeast(1, digit).literal(",}");
            //const between = new R().one(rule).literal("{").atLeast(1, digit).literal(",").atLeast(1, digit).literal("}");
            //const maybe = new R().one(rule).literal("?");
            //const noneOrMany = new R().one(rule).literal("*");
            
            // Any of
            //const anyOfFn = (b, l) => [new Rule<TBranch, TMeta>().anyOf(b)];
            //const more = new R<TBranch, TMeta>().literal("|").anyOf(statement);
            //const anyOf = new R<TBranch, TMeta>(anyOfFn).literal("(").anyOf(statement).noneOrMany(more).literal(")");
            
            ranges.anyOf(atLeastOne);
            
            //const statement = new R().anyOf(all, allExcept, eof, atMost, atLeast, between, maybe, noneOrMany);
            //statement.anyOf(anyOf, atLeastOne, literal, rule);
            statement.anyOf(literal, rule);
            
            this._grammer = new R<TBranch, TMeta>().noneOrMany(statement);
            this._rulexps = {};
        }
        
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
        
        public declare(...ids: string[]): void
        {
            for (const id of ids)
            {
                if (this._rulexps[id] != null)
                    throw new Error(`The rule "${id}" already used.`);
                
                this._rulexps[id] = {
                    id: id,
                    isDefined: false,
                    rule: new Rule<TBranch, TMeta>()
                };
            }
        }
        
        public scan(rootId: string, code: string): RuleResult<TBranch, TMeta>
        {
            const root: IRule<TBranch, TMeta> = this._rulexps[rootId];
            
            if (root == null)
                throw new Error(`Rule with id "${rootId}" not found.`);
             
            return root.rule.scan(code);
        }
    }
    
    
    const grammer = new RulExp<number, IEmpty>();
    //grammer.declare("one", "two", "three");
    //grammer.add("bla", "<one>two<two>three<three>three");
    //grammer.add("tosti", "<bla><bla>");
    //grammer.add("anynumber", "(<one>+|<two>+|<three>+)");
    //grammer.add("one", "one", () => [1]);
    //grammer.add("two", "two", () => [2]);
    //grammer.add("three", "three", () => [3]);
    //grammer.add("bla", "");
    grammer.add("foo", "foo", () => [777]);
    grammer.add("bar", "bar", () => [888]);
    //grammer.add("bla", "<foo>+");
    grammer.add("bla", "<foo>+<bar>+");
    
    //console.log(grammer.scan("one", "one"));
    //console.log(grammer.scan("two", "two"));
    //console.log(grammer.scan("three", "three"));
    //console.log(grammer.scan("anynumber", "one"));
    //console.log(grammer.scan("anynumber", "twotwo"));
    console.log(grammer.scan("bla", "foobarbarbar"));
    console.log(grammer.scan("bla", "foofoobarbar"));
    console.log(grammer.scan("bla", "foofoofoobar"));
    //console.log(grammer.scan("tosti", "onetwotwothreethreethreeonetwotwothreethreethree"));
    
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