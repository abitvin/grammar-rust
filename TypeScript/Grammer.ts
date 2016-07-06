// Copyright (c) 2015-2016 Abitvin <foss@abitvin.net>
// Licensed under the MIT license <LICENSE.md or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

///<reference path="Rule.ts"/>

namespace Abitvin
{
    const enum RangeType
    {
        NoRangeType = 0,
        AtLeast,
        AtMost,
        Between,
        Exact,
        Not
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
                
                // TODO Isn't this is a bug? Shoudn't we be checking if `last == null`?
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
        }
        
        public static get version(): string { return "0.3.0"; }
        
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
}