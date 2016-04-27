// Copyright (c) 2015-2016 Abitvin <foss@abitvin.net>
// Licensed under the MIT license <LICENSE.md or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

namespace Abitvin
{
    export interface IRuleError<TMeta>
    {
        index: number;
        msg: string;
        trail: TMeta[]
    }
    
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
    
	interface IScanContext<TBranch, TMeta>
	{
        branches: TBranch[];
        code: string;
        errors: IRuleError<TMeta>[];
        hasEof: boolean,
        index: number;
        lexeme: string;
        metaPushed: number;
        trail: TMeta[];
	}
    
    export type BranchFn<TBranch> = (branches: TBranch[], lexeme: string) => TBranch|TBranch[];

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
        
        public static get version(): string { return "0.5.1"; }
        public set branchFn(value: BranchFn<TBranch>) { this._branchFn = value; }
        public get meta(): TMeta { return this._meta; }
        public set meta(value: TMeta) { this._meta = value; }
        
        public all(): this
        {
            this._parts.push(this.scanAllLeaf.bind(this, []));
			return this;
        }
        
        public allExcept(...list: string[]): this
        public allExcept(list: string[]): this
        public allExcept(arg1: any): this
		{
            const list: string[] = this.getVariadicArray<string>(arguments);
            
            if (list.length === 0)
                throw new Error("No arguments given for `allExcept`.");
            
            if (list.some(i => i == null))
                throw new Error("An argument in `allExcept` cannot be null or undefined.");
            
            if (list.some(i => i.length !== 1))
                throw new Error("An 'allExcept' item can only be a single character.")
            
            this._parts.push(this.scanAllLeaf.bind(this, list));
			return this;
		}

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

		public literal(text: string): this
		{
            if (!this.isString(text) || text.length < 1)
                throw new Error("Literal text must be a string of at least 1 character.");
            
			this._parts.push(this.scanLiteralLeaf.bind(this, text));
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

        private branch(ctx: IScanContext<TBranch, TMeta>, isRootOfRule: boolean): IScanContext<TBranch, TMeta>
		{
            const newCtx: IScanContext<TBranch, TMeta> = {
				branches: [],
                code: ctx.code,
                hasEof: ctx.hasEof,
				errors: ctx.errors,
                index: ctx.index,
				lexeme: "",
                metaPushed: isRootOfRule ? 0 : ctx.metaPushed,
                trail: ctx.trail.slice(0)
			};
            
            if (isRootOfRule && this._meta)
            {
                newCtx.metaPushed++;
                newCtx.trail.push(this._meta);
            }
            
            return newCtx;
		}
        
        private getVariadicArray<T>(args: IArguments): T[]
        {
            if (Array.isArray(args[0]))
                return args[0];
            
            const arr: T[] = [];
                
            for (let i: number = 0; i < args.length; i++)
                arr.push(args[i]);
                
            return arr;
        }
        
        private isArray<T>(v: any): v is T[]
        {
            return v == null ? false : v.constructor === Array;
        }
        
        private isInteger(v: any): v is Number
        {
            return this.isNumber(v) ? v % 1 === 0 : false;
        }
        
        private isNumber(v: any): v is Number
        {
            return v == null ? false : v.constructor === Number;
        }
        
        private isRule(v: any): v is Rule<TBranch, TMeta>
        {
            return v instanceof Rule;
        }
        
        private isString(v: any): v is string
        {
            return v == null ? false : v.constructor === String;
        }
        
        private merge(target: IScanContext<TBranch, TMeta>, source: IScanContext<TBranch, TMeta>, isRootOfRule: boolean = false): number
		{
            if (isRootOfRule)
                while (source.metaPushed-- > 0)
                    source.trail.pop();
            
            const step: number = source.index - target.index;
            
			target.errors = source.errors;
            target.hasEof = source.hasEof;
            target.index = source.index;
			target.lexeme += source.lexeme;
            target.metaPushed = 0;
            target.trail = source.trail;
            
            if (isRootOfRule && this._branchFn !== null)
                this.pushList(target.branches, this._branchFn(source.branches, source.lexeme));
            else
                this.pushList(target.branches, source.branches);
                
			return step;
		}

		private pushList<T>(dest: T[], src: T|T[]): void
		{
            if (this.isArray(src))
            	src.forEach((i: T) => dest.push(i));
            else
                dest.push(src);
		}
        
        private run(ctx: IScanContext<TBranch, TMeta>): number
		{
			const l: number = this._parts.length;
            
            if (l === 0)
                throw new Error("Rule is not defined.");
            
            const newCtx: IScanContext<TBranch, TMeta> = this.branch(ctx, true);
            
            for (let i: number = 0 ; i < l; i++)
                if (this._parts[i](newCtx) === -1)
                    return -1;
            
            return this.merge(ctx, newCtx, true);
		}
        
        private scanAllLeaf(exclude: string[], ctx: IScanContext<TBranch, TMeta>): number
		{
            const char: string = ctx.code[ctx.index];

            if (char == null)
                return this.updateError(ctx, "End of code while checking for not allowed character.");

            if (exclude.indexOf(char) !== -1)
                return this.updateError( ctx, `Character '${char}' is not allowed here.`);

            ctx.lexeme += char;
            ctx.index++;
            return 1;
		}

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
        
        private scanLiteralLeaf(find: string, ctx: IScanContext<TBranch, TMeta>): number
		{
            let i: number = 0;
            const len: number = find.length;
            const code: string = ctx.code;
            
            while (i < len)
            {
                const c: string = code[ctx.index];

                if (c == null)
                    return this.updateError(ctx, `End of code. The literal '${find}' not found.`);

                if (c !== find[i])
                    return this.updateError(ctx, `The literal '${find}' not found.`);

                ctx.index++;
                i++;
            }

            ctx.lexeme += find;
            return len;
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
        
        private updateError(newCtx: IScanContext<TBranch, TMeta>, errorMsg: string): number
		{
            const errors = newCtx.errors;
            
            if (errors.length !== 0)
            {
                if (newCtx.index < errors[0].index)
                    return -1;
                    
                // Clear the errors array without destroying reference.
                if (newCtx.index > errors[0].index)
                    while (errors.pop()) {};
            }
                
            errors.push({
                msg: errorMsg,
                index: newCtx.index,
                trail: newCtx.trail.slice(0)
            });
            
            return -1;
		}
	}
}