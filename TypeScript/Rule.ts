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
    
    export type BranchFn<TBranch> = (branches: TBranch[], lexeme: string) => TBranch[];

    export class Rule<TBranch, TMeta>
	{
        private _branchFn: BranchFn<TBranch>;
		private _meta: TMeta;
        private _parts: {(ctx): boolean}[] = [];
        
		constructor(branchFn: BranchFn<TBranch> = null, meta: TMeta = null)
        {
			this._branchFn = branchFn;
            this._meta = meta;
		}
        
        public static get version(): string { return "0.4.0"; }
        public get meta(): TMeta { return this._meta; }
        
        public allExcept(...list: string[]): this
        public allExcept(list: string[]): this
        public allExcept(arg1: any): this
		{
            const list: string[] = this.getVariadicArray<string>(arguments);
            
            list.forEach(item => {
                if (item.length !== 1)
                    throw new Error("An 'all except' list item can only be a single character.")
            });
            
            this._parts.push(this.scanAllExceptLeaf.bind(this, list));
			return this;
		}

		public alter(...list: string[]): this
        public alter(list: string[]): this
        public alter(arg1: any): this
		{
            const list: string[] = this.getVariadicArray<string>(arguments); 
            
            if (list.length % 2 === 1)
                throw new Error("Alter list must be a factor of 2.");

			this._parts.push(this.scanAlterLeaf.bind(this, list));
			return this;
		}

		public atLeast(num: number, rule: Rule<TBranch, TMeta>): this
		public atLeast(num: number, text: string): this
		public atLeast(num: number, arg2: any): this
		{
            if (this.isString(arg2))
                this._parts.push(this.scanRuleRange.bind(this, num, Number.POSITIVE_INFINITY, new Rule<TBranch, TMeta>().literal(arg2)));
			else
				this._parts.push(this.scanRuleRange.bind(this, num, Number.POSITIVE_INFINITY, arg2));

			return this;
		}
        
        public atMost(num: number, rule: Rule<TBranch, TMeta>): this
		public atMost(num: number, text: string): this
		public atMost(num: number, arg2: any): this
		{
            if (this.isString(arg2))
                this._parts.push(this.scanRuleRange.bind(this, 0, num, new Rule<TBranch, TMeta>().literal(arg2)));
			else
				this._parts.push(this.scanRuleRange.bind(this, 0, num, arg2));

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
                this._parts.push(this.scanAnyOf.bind(this, (<string[]>items).map(l => new Rule<TBranch, TMeta>().literal(l))));
			else
				this._parts.push(this.scanAnyOf.bind(this, items));

			return this;
		}

		public between(min: number, max: number, rule: Rule<TBranch, TMeta>): this
        public between(charA: string, charB: string, notUsed?: any): this
        public between(arg1: any, arg2: any, arg3: any): this
		{
            if (this.isString(arg1))
                this._parts.push(this.scanCharRangeLeaf.bind(this, arg1.charCodeAt(0), arg2.charCodeAt(0)));
            else
                this._parts.push(this.scanRuleRange.bind(this, arg1, arg2, arg3));
                
			return this;
		}
        
        public eof(): this
        {
            this._parts.push(this.scanEofLeaf.bind(this));
            return this;
        }
        
        public exact(num: number, rule: Rule<TBranch, TMeta>): this
		public exact(num: number, text: string): this
		public exact(num: number, arg2: any): this
		{
            if (this.isString(arg2))
                this._parts.push(this.scanRuleRange.bind(this, num, num, new Rule<TBranch, TMeta>().literal(arg2)));
			else
				this._parts.push(this.scanRuleRange.bind(this, num, num, arg2));

			return this;
		}

		public maybe(rule: Rule<TBranch, TMeta>): this
		public maybe(text: string): this
		public maybe(item: any): this
		{
			if (this.isString(item))
                this._parts.push(this.scanRuleRange.bind(this, 0, 1, new Rule<TBranch, TMeta>().literal(item)));
			else
				this._parts.push(this.scanRuleRange.bind(this, 0, 1, item));

			return this;
		}

		public literal(text: string): this
		{
			this._parts.push(this.scanLiteralLeaf.bind(this, text));
			return this;
		}
        
        public noneOrMany(rule: Rule<TBranch, TMeta>): this
		public noneOrMany(text: string): this
		public noneOrMany(item: any): this
		{
            if (this.isString(item))
			    this._parts.push(this.scanRuleRange.bind(this, 0, Number.POSITIVE_INFINITY, new Rule<TBranch, TMeta>().literal(item)));
            else
			    this._parts.push(this.scanRuleRange.bind(this, 0, Number.POSITIVE_INFINITY, item));

			return this;
		}	

		public one(rule: Rule<TBranch, TMeta>): this
		{
			this._parts.push(this.scanRuleRange.bind(this, 1, 1, rule));
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

			if (!this.run(ctx))
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
        
        private isString(v: any): v is string
        {
            return v == null ? false : v.constructor === String;
        }
        
        private merge(target: IScanContext<TBranch, TMeta>, source: IScanContext<TBranch, TMeta>, isRootOfRule: boolean = false): boolean
		{
            if (isRootOfRule)
                while (source.metaPushed-- > 0)
                    source.trail.pop();
            
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
                
			// Always true so we can create a tail call by invocation. 
            return true;
		}

		private pushList<T>(a: T[], b: T[]): void
		{
			b.forEach((i: T) => a.push(i));
		}
        
        private run(ctx: IScanContext<TBranch, TMeta>): boolean
		{
			const l: number = this._parts.length;
            const newCtx: IScanContext<TBranch, TMeta> = this.branch(ctx, true);

            for (let i: number = 0 ; i < l; i++)
            {
                if (this._parts[i](newCtx))
                    continue;
                
                return false;
            }
			
            return this.merge(ctx, newCtx, true);
		}
        
        private scanAllExceptLeaf(list: string[], ctx: IScanContext<TBranch, TMeta>): boolean
		{
            const char: string = ctx.code[ctx.index];

            if (char == null)
                return this.updateError(ctx, "End of code while checking for not allowed character.");

            if (list.indexOf(char) !== -1)
                return this.updateError( ctx, `Character '${char}' is not allowed here.`);

            ctx.lexeme += char;
            ctx.index++;
            return true;
		}

		private scanAlterLeaf(list: string[], ctx: IScanContext<TBranch, TMeta>): boolean
		{
            for (let i = 0; i < list.length; i += 2)
            {
                const find: string = list[i];
                const len: number = find.length;

                if (find === ctx.code.substr(ctx.index, len))
                {
                    ctx.lexeme += list[i+1];
                    ctx.index += len;
                    return true;
                }
            }
            
            return this.updateError(ctx, "Alter characters not found on this position.");
		}

		private scanAnyOf(rules: Rule<TBranch, TMeta>[], ctx: IScanContext<TBranch, TMeta>): boolean
		{
            const c: number = rules.length;

            for(let i: number = 0; i < c; i++)
            {
                const rule: Rule<TBranch, TMeta> = rules[i];
                const newCtx: IScanContext<TBranch, TMeta> = this.branch(ctx, false);
                
                if (rule.run(newCtx))
                    return this.merge(ctx, newCtx);
            }

            return false;
		}

		private scanCharRangeLeaf(codeA: number, codeB: number, ctx: IScanContext<TBranch, TMeta>): boolean
		{
            const char: string = ctx.code[ctx.index];
            
            if (char == null)
                return this.updateError(ctx, `End of code. Expected a character between '${String.fromCharCode(codeA)}' and '${String.fromCharCode(codeB)}'.`);
                
            const code: number = char.charCodeAt(0);
            
            if (code < codeA || code > codeB)
                return this.updateError(ctx, `Expected a character between '${String.fromCharCode(codeA)}' and '${String.fromCharCode(codeB)}'; got a '${char}'.`);
                
            ctx.lexeme += char;
            ctx.index++;
            return true;
		}
        
        private scanEofLeaf(ctx: IScanContext<TBranch, TMeta>): boolean
        {
            if (ctx.index === ctx.code.length)
            {
                ctx.hasEof = true;
                ctx.index++;
                return true;
            }
            return this.updateError(ctx, "No EOF on this position.");
        }
        
        private scanLiteralLeaf(find: string, ctx: IScanContext<TBranch, TMeta>): boolean
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
            return true;
		}

		private scanRuleRange(min: number, max: number, rule: Rule<TBranch, TMeta>, ctx: IScanContext<TBranch, TMeta>): boolean
		{
            let count: number = 0;
            const newCtx: IScanContext<TBranch, TMeta> = this.branch(ctx, false);
            
            while (rule.run(newCtx))
                if (++count === max)
                    break;

            if (count >= min && count <= max)
                return this.merge(ctx, newCtx);
            
            return false;
        }
        
		private showCode(text: string, position: number): void
        {
            console.error(text.substr(position, 40));
        }
        
        private updateError(newCtx: IScanContext<TBranch, TMeta>, errorMsg: string): boolean
		{
            const errors = newCtx.errors;
            
            if (errors.length !== 0)
            {
                if (newCtx.index < errors[0].index)
                    return;
                    
                // Clear the errors array without destroying reference.
                if (newCtx.index > errors[0].index)
                    while (errors.pop()) {};
            }
                
            errors.push({
                msg: errorMsg,
                index: newCtx.index,
                trail: newCtx.trail.slice(0)
            });
            
            // Always false so we can create a tail call by invocation. 
            return false;
		}
	}
}