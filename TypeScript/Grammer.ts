///<reference path="Rule.ts"/>

namespace Abitvin
{
    const enum RangeType
    {
        NoRangeType = 0,
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
        private _ws: Rule<TBranch, TMeta>;
        
        constructor()
        {
            this._ws = new Rule<TBranch, TMeta>().anyOf(" ", "\t", "\n", "\r");
            
            const ranges = new R<TBranch, TMeta>();
            const statement = new R<TBranch, TMeta>();
            
            const char = new R<TBranch, TMeta>().between("a", "z");
            const CHAR = new R<TBranch, TMeta>().between("A", "Z");
            const Char = new R<TBranch, TMeta>().anyOf(char, CHAR);
            const digit = new R<TBranch, TMeta>().between("0", "9");
            
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
            
            const literalControlChars = new R<TBranch, TMeta>().alter("\\<", "<", "\\>", ">", "\\{", "{", "\\}", "}", "\\(", "(", "\\)", ")", "\\[", "[", "\\]", "]", "\\+", "+", "\\?", "?", "\\*", "*", "\\|", "|", "\\.", ".", "\\$", "$", "\\^", "^", "\\,", ",", "\\ ", " ", "\\_", "_");
            const literalAllExcept = new R<TBranch, TMeta>().allExcept("<", ">", "{", "}", "(", ")", "[", "]", "+", "?", "*", "|", ".", "$", ",", " ", "_");
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
                    rule = this.addRange(rule, b[1]);
                   
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
                
                if (b.length === 1)
                    rule = this.addRange(rule, b[0]);
                
                return [{ 
                    arg1: null,
                    arg2: null,
                    arg3: null,
                    rangeType: RangeType.NoRangeType,
                    rule: rule 
                }];
            };
             
            const anyChar = new R<TBranch, TMeta>(allFn).literal(".").maybe(ranges);
            
            // All except
            const allExceptCharsFn = (b, l) => [{
                arg1: null,
                arg2: null,
                arg3: l,
                rangeType: RangeType.NoRangeType,
                rule: null
            }];
            
            const allExceptFn = (b, l) =>
            {
                let rule = new Rule<TBranch, TMeta>().allExcept(b[0].arg3.split(""));
                const last = b[b.length - 1];
                
                if (last.rangeType !== RangeType.NoRangeType)
                    rule = this.addRange(rule, last);
                
                return [{ 
                    arg1: null,
                    arg2: null,
                    arg3: null,
                    rangeType: RangeType.NoRangeType,
                    rule: rule 
                }];
            };
            
            const allExceptEscaped = new R<TBranch, TMeta>().alter("\\]", "]");
            const allExceptAnyOther = new R<TBranch, TMeta>().allExcept("]");
            const allExceptChar = new R<TBranch, TMeta>().anyOf(allExceptEscaped, allExceptAnyOther);
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
                
                return [{ 
                    arg1: null,
                    arg2: null,
                    arg3: null,
                    rangeType: RangeType.NoRangeType,
                    rule: rule 
                }];
            };
            
            const charRange = new R<TBranch, TMeta>(charRangeFn).one(literalChar).literal("-").one(literalChar);
            const charRanges = new R<TBranch, TMeta>(charRangesFn).literal("[").atLeast(1, charRange).literal("]").maybe(ranges);
            
            // EOF
            const eofFn = (b, l) => [{ 
                arg1: null,
                arg2: null,
                arg3: null,
                rangeType: RangeType.NoRangeType,
                rule: new Rule<TBranch, TMeta>().eof()
            }];
            
            const eof = new R<TBranch, TMeta>(eofFn).literal("$");
            
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
                    rule = new Rule<TBranch, TMeta>().one(r.rule);
                else
                    rule = this.addRange(r.rule, b[1]);
                
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
                const last = b[b.length - 1];
                let rules = b.map(r => r.rule);
                
                if (last.rangeType !== RangeType.NoRangeType)
                    rules = rules.slice(0, -1);
                    
                const rule = this.addRange(new Rule<TBranch, TMeta>().anyOf(rules), last);
                
                return [{
                    arg1: null,
                    arg2: null,
                    arg3: null,
                    rangeType: RangeType.NoRangeType,
                    rule: rule
                }];
            }
            
            const statementsFn = (b, l) =>
            {
                if (b.length === 1)
                   return b; 
                
                const rule = new Rule<TBranch, TMeta>();
                
                for (const pc of b)
                    rule.one(pc.rule);
                 
                return [{
                    arg1: null,
                    arg2: null,
                    arg3: null,
                    rangeType: RangeType.NoRangeType,
                    rule: rule
                }];
            };
            
            const statements = new R<TBranch, TMeta>(statementsFn).atLeast(1, statement); 
            const more = new R<TBranch, TMeta>().literal("|").one(statements);
            const anyOf = new R<TBranch, TMeta>(anyOfFn).literal("(").one(statements).noneOrMany(more).literal(")").maybe(ranges);
            
            // Alter
            const alterCharFn = (b, l) => [{
                arg1: null,
                arg2: null,
                arg3: l,
                rangeType: RangeType.NoRangeType,
                rule: null
            }];
            
            const alterFn = (b, l) => 
            {
                const last = b[b.length - 1];
                
                if (last.rangeType !== RangeType.NoRangeType)
                    b = b.slice(0, -1);
                
                return [{
                    arg1: null,
                    arg2: null,
                    arg3: null,
                    rangeType: RangeType.NoRangeType,
                    rule: this.addRange(new Rule<TBranch, TMeta>().alter(b.map(i => i.arg3)), last)
                }];
            }
            
            const alterChar = new R<TBranch, TMeta>(alterCharFn).atLeast(1, literalChar);
            const alterMore = new R<TBranch, TMeta>().literal(",").one(alterChar);
            const alter = new R<TBranch, TMeta>(alterFn).literal("(").one(alterChar).noneOrMany(alterMore).literal(")").maybe(ranges);
            
            // Whitespace
            const atLeastOneWsFn = () => [{
                arg1: null,
                arg2: null,
                arg3: null,
                rangeType: RangeType.NoRangeType,
                rule: new Rule<TBranch, TMeta>().atLeast(1, this._ws)
            }];
            
            const noneOrManyWsFs = () => [{
                arg1: null,
                arg2: null,
                arg3: null,
                rangeType: RangeType.NoRangeType,
                rule: new Rule<TBranch, TMeta>().noneOrMany(this._ws)
            }];
            
            const atLeastOneWs = new R<TBranch, TMeta>(atLeastOneWsFn).literal("_");
            const noneOrManyWs = new R<TBranch, TMeta>(noneOrManyWsFs).literal(" ");
            
            // Ranges and statements definitions
            ranges.anyOf(atLeast, atLeastOne, atMost, between, exact, maybe, noneOrMany);
            statement.anyOf(noneOrManyWs, atLeastOneWs, literal, eof, alter, allExcept, charRanges, anyChar, rule, anyOf);
            
            this._grammer = new R<TBranch, TMeta>().noneOrMany(statement);
            this._rulexps = {};
        }
        
        public static get version(): string { return "0.1.9"; }
        
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
        
        public ws(expr: string): void
        {
            const result = this._grammer.scan(expr);
            
            // TODO Show nice errors.
            if (!result.isSuccess)
                throw new Error("Error compiling rule expression.");
                
            this._ws.clear();
            this._ws.one(result.branches[0].rule);
        }
        
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
    }
    
    /*
    
    // TODO Implement this in QBaksteen
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
    
    const alter = new Grammer<string, IEmpty>();
    alter.add("root", "(\\n,\n,asdasd,asdasxd,aa,bb,yyy,xxx)+", (b, l) => [l]);
    //console.log(alter.scan("root", "yyy\\n\\nasdasdaaasdasd\\naaasdasdyyy"));
    
    const calc = new Grammer<number, IEmpty>();
    calc.declare("add", "expr", "mul");
    calc.add("num", "[0-9]+", (b, l) => [parseInt(l)]);
    calc.add("brackets", "\\(<expr>\\)");   // Identity function
    calc.add("mul", "(<num>|<brackets>)(\\*<mul>)?", b => b.length === 1 ? b : [b[0] * b[1]]);
    calc.add("add", "<mul>(\\+<add>)?", b => b.length === 1 ? b : [b[0] + b[1]]);
    calc.add("expr", "(<add>|<brackets>)");
    
    //console.log(calc.scan("expr", "2*(3*4*5)")); // 120
    //console.log(calc.scan("expr", "2*(3+4)*5")); // 70
    //console.log(calc.scan("expr", "((2+3*4+5))")); // 19
    
    interface IBinaryOperator
    {
        operator: Operator;
        left: AstNode;
        right: AstNode;
    }  
    
    interface IAssignemnt
    {
        variable: string;
        expr: AstNode;
    }

    interface IFunction
    {
        arguments: AstNode[],
        name: string
    }
    
    enum Operator { multiply, add }
    
    type AstNode = IBinaryOperator | number | IFunction | string | IAssignemnt;

    const calcAst = new Grammer<AstNode, IEmpty>();
    calcAst.declare("add", "expr", "function", "mul", "base");
    
    //calcAst.ws("(\\ |\t)");
    
    calcAst.add("num", "[0-9]+", (b, l) => [parseInt(l)]);
    calcAst.add("symbol", "[a-z]+", (b, l) => [l]);
    
    calcAst.add("brackets", "\\( <expr> \\)");   // Identity function
    calcAst.add("mul", "<base>( \\* <mul>)?", b => b.length === 1 ? b : [{ operator: Operator.multiply, left: b[0], right: b[1] }]);
    calcAst.add("add", "<mul>( \\+ <add>)?", b => b.length === 1 ? b : [{ operator: Operator.add, left: b[0], right: b[1] }]);
    calcAst.add("expr", "<add>");
    calcAst.add("function", "<symbol>\\( (<expr>( \\, <expr>)*)? \\)", b => [{ name: <string>b[0], arguments: b.slice(1)}])
    calcAst.add("variable", "<symbol>")
    calcAst.add("base", "(<function>|<num>|<brackets>|<variable>)");

    calcAst.add("assignment", "<symbol> := <expr>", b => [{ variable: <string>b[0], expr: b[1]}]);
    calcAst.add("statement", " (<assignment>|<expr>) (;|$) ")
    calcAst.add("codeblock", "<statement>*");
    
    calcAst.add("code", "<codeblock>");
    
    const showAst = (code: string) => {
        const result = calcAst.scan("code", code);
        console.log(JSON.stringify(result.branches));
    }

    showAst("14;");
    showAst("(14);");
    showAst("2* ( 3*4*5 );");
    showAst("2*(\t3+4)*5;");
    showAst("foo();");
    showAst("2*foo();");
    showAst("foo(7);");
    showAst("2*foo(7 +   bar( 4*5  ));");
    showAst("foo(  5,10   )");
    showAst("2*foo(5  ,  10  ,  34,345, 45)");
    showAst("x    :=  \t\t\t  2*y+1  ;  y := 12   ");
    
    
    // console.log(JSON.stringify(calcAst.scan("expr", "2*(3*4*5)").branches[0])); // 120
    //console.log(calcAst.scan("expr", "2*(3+4)*5")); // 70
    //console.log(calcAst.scan("expr", "((2+3*4+5))")); // 19
    // console.log(JSON.stringify(calcAst.scan("expr", "2*foo()").branches[0])); // 120
    //console.log(calcAst.scan("expr", "2*foo()")); // 120
    
    /*
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