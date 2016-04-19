///<reference path="Rule.ts"/>

namespace Abitvin
{
    // all:         .
    // allExcept:   ^char
    // alter:       [x,y]
    // between:     [a-z]
    // eof:         EOF
        
    
    const enum RangeType
    {
        NoRangeType,
        AtLeast,
        AtMost,
        Between,
        Exact
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
    
    export class Grammer<TBranch, TMeta> 
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
            const digit = new R<TBranch, TMeta>().between("0", "9");
            //const anyChar = new R().all();
            
            //const all = new R(() => [new Rule().all()]).literal(".");
            //const allExcept = new R().literal("^").one(anyChar);
            //const eof = new R().literal("EOF");
            
            // Integer
            const integerFn = (b, l) => [{
                arg1: parseInt(l), 
                arg2: null,
                arg3: null, 
                rangeType: RangeType.NoRangeType, 
                rule: null 
            }];
            
            const integer = new R<TBranch, TMeta>(integerFn).atLeast(1, digit);
            
            // Literal
            const literalTextFn = (b, l) => [{
                arg1: null, 
                arg2: null,
                arg3: l, 
                rangeType: RangeType.NoRangeType, 
                rule: null 
            }];
            
            const literalControlChars = new R<TBranch, TMeta>().alter("\\<", "<", "\\>", ">", "\\{", "{", "\\}", "}", "\\(", "(", "\\)", ")", "\\+", "+", "\\?", "?", "\\*", "*", "\\|", "|", "\\.", ".");
            const literalAllExcept = new R<TBranch, TMeta>().allExcept("<", ">", "{", "}", "(", ")", "+", "?", "*", "|", ".");
            const literalChar = new R<TBranch, TMeta>().anyOf(literalControlChars, literalAllExcept);
            const literalText = new R<TBranch, TMeta>(literalTextFn).atLeast(1, literalChar);
            
            /* TODO Maybe implement stuff below...
            const af = new R<TBranch, TMeta>().between("a", "f");
            const AF = new R<TBranch, TMeta>().between("A", "F");
            const hex = new R<TBranch, TMeta>().anyOf(digit, af, AF);
            
            const combineCharsFn = (b, l) => [{
                arg1: null, 
                arg2: null,
                arg3: b.map(i => i.arg3).join(""), 
                rangeType: RangeType.NoRangeType, 
                rule: null 
            }];
            
            const parseCharCodeFn = (b, l) => [{
                arg1: null, 
                arg2: null,
                arg3: String.fromCharCode(parseInt(l.substr(2), 16)), 
                rangeType: RangeType.NoRangeType, 
                rule: null 
            }];
            
            const passLexemeFn = (b, l) => [{
                arg1: null, 
                arg2: null,
                arg3: l, 
                rangeType: RangeType.NoRangeType, 
                rule: null 
            }];
            
            const strEscapeControl = new R<TBranch, TMeta>(passLexemeFn).alter("\\0", "\0", "\\b", "\b", "\\f", "\f", "\\n", "\n", "\\r", "\r", "\\t", "\t", "\\v", "\v", "\\\"", "\"");
            const strEscapeLatin1 = new R<TBranch, TMeta>(parseCharCodeFn).literal("\\x").exact(2, hex);
            const strEscapeUTF16 = new R<TBranch, TMeta>(parseCharCodeFn).literal("\\u").exact(4, hex);
            const strEscapeUnknown = new R<TBranch, TMeta>(passLexemeFn).literal("\\");
            //const strAllExceptBs = new R<TBranch, TMeta>(passLexemeFn).allExcept(["\""]);
            const strAllExceptBs = new R<TBranch, TMeta>(passLexemeFn).allExcept("<", "{", "(", "+", "?", "*");
            const strChar = new R<TBranch, TMeta>().anyOf(strEscapeControl, strEscapeLatin1, strEscapeUTF16, strEscapeUnknown, strAllExceptBs);
            //const strValue = new R<TBranch, TMeta>(combineCharsFn).noneOrMany(strChar);
            const literalText = new R<TBranch, TMeta>(combineCharsFn).atLeast(1, strChar);
            //const str = new R<TBranch, TMeta>().literal("\"").one(strValue).literal("\"");
            //const literalText = new R<TBranch, TMeta>(literalTextFn).atLeast(1, literalChar);
            */
            
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
                        
                        case RangeType.AtMost:
                        {
                            rule = new Rule<TBranch, TMeta>().atMost(b[1].arg1, rule);
                            break;
                        }
                        
                        case RangeType.Between:
                        {
                            rule = new Rule<TBranch, TMeta>().between(b[1].arg1, b[1].arg2, rule);
                            break;
                        }
                        
                        case RangeType.Exact:
                        {
                            rule = new Rule<TBranch, TMeta>().exact(b[1].arg1, rule);
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
            
            // All
            const allFn = (b, l) =>
            {
                let rule = new Rule<TBranch, TMeta>().all();
                
                if (b.length > 0) 
                switch(b[0].rangeType)
                {
                    case RangeType.AtLeast:
                    {
                        rule = new Rule<TBranch, TMeta>().atLeast(b[0].arg1, rule);
                        break;
                    }
                    
                    case RangeType.AtMost:
                    {
                        rule = new Rule<TBranch, TMeta>().atMost(b[0].arg1, rule);
                        break;
                    }
                    
                    case RangeType.Between:
                    {
                        rule = new Rule<TBranch, TMeta>().between(b[0].arg1, b[0].arg2, rule);
                        break;
                    }
                    
                    case RangeType.Exact:
                    {
                        rule = new Rule<TBranch, TMeta>().exact(b[0].arg1, rule);
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
             
            const anyChar = new R<TBranch, TMeta>(allFn).literal(".").maybe(ranges);
            
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
                    
                    case RangeType.AtMost:
                    {
                        rule = new Rule<TBranch, TMeta>().atMost(b[1].arg1, r.rule);
                        break;
                    }
                    
                    case RangeType.Between:
                    {
                        rule = new Rule<TBranch, TMeta>().between(b[1].arg1, b[1].arg2, r.rule);
                        break;
                    }
                    
                    case RangeType.Exact:
                    {
                        rule = new Rule<TBranch, TMeta>().exact(b[1].arg1, r.rule);
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
            
            // At least
            const atLeastFn = (b, l) => [{
                arg1: b[0].arg1,
                arg2: null,
                arg3: null,
                rangeType: RangeType.AtLeast,
                rule: null
            }];
            
            const atLeast = new R<TBranch, TMeta>(atLeastFn).literal("{").one(integer).literal(",}");
            
            // At least one
            const atLeastOneFn = (b, l) => [{
                arg1: 1,
                arg2: null,
                arg3: null,
                rangeType: RangeType.AtLeast,
                rule: null
            }];
            
            const atLeastOne = new R<TBranch, TMeta>(atLeastOneFn).literal("+");
            
            // At most
            const atMostFn = (b, l) => [{
                arg1: b[0].arg1,
                arg2: null,
                arg3: null,
                rangeType: RangeType.AtMost,
                rule: null
            }];
            
            const atMost = new R<TBranch, TMeta>(atMostFn).literal("{,").one(integer).literal("}");
            
            // Between
            const betweenFn = (b, l) => [{
                arg1: b[0].arg1,
                arg2: b[1].arg1,
                arg3: null,
                rangeType: RangeType.Between,
                rule: null
            }];
            
            const between = new R<TBranch, TMeta>(betweenFn).literal("{").one(integer).literal(",").one(integer).literal("}");
            
            // Exact
            const exactFn = (b, l) => [{
                arg1: b[0].arg1,
                arg2: null,
                arg3: null,
                rangeType: RangeType.Exact,
                rule: null
            }];
            
            const exact = new R<TBranch, TMeta>(exactFn).literal("{").one(integer).literal("}");
            
            // Maybe
            const maybeFn = (b, l) => [{
                arg1: 0,
                arg2: 1,
                arg3: null,
                rangeType: RangeType.Between,
                rule: null
            }];
            
            const maybe = new R<TBranch, TMeta>(maybeFn).literal("?");
            
            // None or many
            const noneOrManyFn = (b, l) => [{
                arg1: 0,
                arg2: null,
                arg3: null,
                rangeType: RangeType.AtLeast,
                rule: null
            }];
            
            const noneOrMany = new R<TBranch, TMeta>(noneOrManyFn).literal("*");
            
            // Any of
            const anyOfFn = (b, l) =>
            {
                const rules = b.map(r => r.rule);
                const last = b[b.length - 1];
                let rule = null;
                
                switch(last.rangeType)
                {
                    case RangeType.AtLeast:
                    {
                        rule = new Rule<TBranch, TMeta>().anyOf(rules.slice(0, -1));
                        rule = new Rule<TBranch, TMeta>().atLeast(last.arg1, rule);
                        break;
                    }
                    
                    case RangeType.AtMost:
                    {
                        rule = new Rule<TBranch, TMeta>().anyOf(rules.slice(0, -1));
                        rule = new Rule<TBranch, TMeta>().atMost(last.arg1, rule);
                        break;
                    }
                    
                    case RangeType.Between:
                    {
                        rule = new Rule<TBranch, TMeta>().anyOf(rules.slice(0, -1));
                        rule = new Rule<TBranch, TMeta>().between(last.arg1, last.arg2, rule);
                        break;
                    }
                    
                    case RangeType.Exact:
                    {
                        rule = new Rule<TBranch, TMeta>().anyOf(rules.slice(0, -1));
                        rule = new Rule<TBranch, TMeta>().exact(last.arg1, rule);
                        break;
                    }
                    
                    case RangeType.NoRangeType:
                    {
                        rule = new Rule<TBranch, TMeta>().anyOf(rules);
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
            }
            
            const more = new R<TBranch, TMeta>().literal("|").one(statement);
            const anyOf = new R<TBranch, TMeta>(anyOfFn).literal("(").one(statement).noneOrMany(more).literal(")").maybe(ranges);
            
            // Ranges and statements definitions
            ranges.anyOf(atLeast, atLeastOne, atMost, between, exact, maybe, noneOrMany);
            //const statement = new R().anyOf(all, allExcept, eof, atMost, atLeast, between, maybe, noneOrMany);
            statement.anyOf(literal, anyChar, rule, anyOf);
            
            this._grammer = new R<TBranch, TMeta>().noneOrMany(statement);
            this._rulexps = {};
        }
        
        public static get version(): string { return "0.1.1"; }
        
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
    
    
    const grammer = new Grammer<number, IEmpty>();
    
    //grammer.declare("digit", "two", "three");
    //grammer.add("bla", "<one>two<two>three<three>three");
    //grammer.add("tosti", "<bla><bla>");
    
    //grammer.add("digit", "x+");
    //grammer.add("multiplication", "<digit> (* <multiplication>)?");
    //grammer.add("addition", "<multiplication> (+ <addition)?")
    
    //grammer.scan("addition", "3 + 3 * 5");
    
    //grammer.add("anynumber", "one{0,}<two>{2,10}<three>*");
    //grammer.add("one", "one", () => [1]);
    //grammer.add("two", "two", () => [2]);
    //grammer.add("three", "three", () => [3]);
    //grammer.add("foo", "foo", () => [999]);
    //grammer.add("bar", "bar", () => [888]);
    //grammer.add("bla", "<foo>+");
    //grammer.add("bla", "<foo>?<foo>?<foo>?<bar>*");
    //grammer.add("bla", "foo?foo?foo?bar*", () => [9999]);
    //grammer.add("bla", "\\*\\<test>+", () => [7777]);
    //grammer.add("bla", "(<one>|<two>|<three>){,4}");
    //grammer.add("bla", "(one|two|three)+", () => [8888]);
    //grammer.add("bla", "(<one>|two|three){,4}");
    //grammer.add("bla", "(\\n,\n){,4}");
    //grammer.add("bla", "(|<one>,two,three){,4}");
    //grammer.add("bla", "(anyOf <one>,two,three)");
    //grammer.add("bla", "(alter \\n,\n)");
    //grammer.add("bla", "(all except \\n,\n)");
    //console.log(grammer.scan("bla", "oneone"));
    //console.log(grammer.scan("bla", "onetwothree"));
    //console.log(grammer.scan("bla", "threetwoonetwothree"));
    //console.log(grammer.scan("bla", "four"));
    grammer.add("bla", ".*", () => [1111]);
    console.log(grammer.scan("bla", "A"));
    console.log(grammer.scan("bla", "B"));
    console.log(grammer.scan("bla", "CC"));
    
    grammer.add("foo", "..*", () => [2222]);
    console.log(grammer.scan("foo", "A"));
    console.log(grammer.scan("foo", "B"));
    console.log(grammer.scan("foo", "CC"));
    
    grammer.add("faa", ".{0,10}", () => [3333]);
    console.log(grammer.scan("faa", ""));
    console.log(grammer.scan("faa", "B"));
    console.log(grammer.scan("faa", "BB"));
    
    grammer.add("blo", ".?.{0,}", () => [4444]);
    console.log(grammer.scan("blo", ""));
    console.log(grammer.scan("blo", "B"));
    console.log(grammer.scan("blo", "CC"));
    
    
    //const literalControlChars = new R<TBranch, TMeta>().alter("\\<", "<", "\\{", "{", "\\(", "(", "\\+", "+", "\\?", "?", "\\*", "*");
    //const literalAllExcept = new R<TBranch, TMeta>().allExcept("<", "{", "(", "+", "?", "*");
    
    //grammer.add("q", "\\+\\*", () => [8888]);
    //grammer.add("banana", "<q>+");
    //console.log(grammer.scan("banana", ""));
    //console.log(grammer.scan("banana", "+*+*"));
    //console.log(grammer.scan("banana", "+*+*+*+*+*+*+*"));
            
    
        
    //console.log(grammer.scan("one", "one"));
    //console.log(grammer.scan("two", "two"));
    //console.log(grammer.scan("three", "three"));
    //console.log(grammer.scan("anynumber", "onetwothreethreethree"));
    //console.log(grammer.scan("anynumber", "twotwo"));
    //console.log(grammer.scan("bla", "foobarbarbar"));
    //console.log(grammer.scan("bla", "foofoobarbar"));
    //console.log(grammer.scan("bla", "foofoofoobar"));
    //console.log(grammer.scan("tosti", "onetwotwothreethreethreeonetwotwothreethreethree"));
    //console.log(grammer.scan("bla", "*<test>*<test>*<test>"));
    
    //const one  = new Rule<number, IEmpty>(() => [1]).literal("one");
    //const two  = new Rule<number, IEmpty>(() => [2]).literal("two");
    //const three  = new Rule<number, IEmpty>(() => [3]).literal("three");
    //const anyNumber = new Rule<number, IEmpty>().anyOf(one, two, three);
    //const root = new Rule<number, IEmpty>().one(anyNumber);
    
    
    
    
    
    /* TODO Implement this in QBaksteen
    
    interface IParseContextB
    {      
        num: number;
    }
    
    const mul = new Rule<IParseContextB, IEmpty>((b, l) =>
    {
        if (b.length === 1)
            return b;
        else
            return [{ num: b[0].num * b[1].num }];
    });
    
    const add = new Rule<IParseContextB, IEmpty>((b, l) =>
    {
        if (b.length === 1)
            return b;
        else
            return [{ num: b[0].num + b[1].num }];
    });
    
    const expr = new Rule<IParseContextB, IEmpty>((b, l) =>
    {
        return b;
    });
    
    const digit = new Rule<IParseContextB, IEmpty>().between("0", "9");
    const num = new Rule<IParseContextB, IEmpty>((b, l) => [{ num: parseInt(l) }]).atLeast(1, digit);
    const brackets = new Rule<IParseContextB, IEmpty>().literal("(").one(expr).literal(")");
    
    const mulRight = new Rule<IParseContextB, IEmpty>().literal("*").one(mul);
    mul.anyOf(num, brackets).maybe(mulRight);
    
    const addRight = new Rule<IParseContextB, IEmpty>().literal("+").one(add);
    add.one(mul).maybe(addRight);
    
    expr.anyOf(add, brackets);
    
    
    
    
    console.log(expr.scan("2*(3*4*5)")); // 120
    console.log(expr.scan("2*(3+4)*5")); // 70
    console.log(expr.scan("((2+3*4+5))")); // 19
    */
    
    //grammer.add("digit", "x+");
    //grammer.add("multiplication", "<digit> (* <multiplication>)?");
    //grammer.add("addition", "<multiplication> (+ <addition)?")
    
    
    
    
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