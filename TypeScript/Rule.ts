namespace Abitvin
{
	interface IScanContext<TBranch>
	{
        branches: TBranch[];
        code: string;
        errorIndex: number;
        errorMessage: string;
        hasEof: boolean,
        index: number;
        lexeme: string;
	}
    
    export type BranchFn<TBranch> = (branches: TBranch[], lexeme: string) => TBranch[];

    export class Rule<TBranch>
	{
        protected _branchFn: BranchFn<TBranch>;
		protected _parts: {(ctx): boolean}[] = [];

		constructor(branchFn: BranchFn<TBranch> = null)
        {
			this.setBranchFn(branchFn);
		}
        
        public static get version(): string  { return "0.1"; }

        public allExcept(list: string[]): Rule<TBranch>
		{
            list.forEach(item => {
                if (item.length !== 1)
                    throw new Error("An 'all except' list item can only be a single character.")
            });
            
            this._parts.push(this.scanAllExcept.bind(this, list));
			return this;
		}

		public alter(list: string[]): Rule<TBranch>
		{
            if (list.length % 2 === 1)
                throw new Error("Alter list must be a factor of 2.");

			this._parts.push(this.scanAlter.bind(this, list));
			return this;
		}

		public atLeastOne(rule: Rule<TBranch>): Rule<TBranch>
		public atLeastOne(text: string): Rule<TBranch>
		public atLeastOne(arg: any): Rule<TBranch>
		{
            if (typeof arg === 'string')
                this._parts.push(this.scanAtLeastOne.bind(this, new Rule<TBranch>().literal(arg)));
			else
				this._parts.push(this.scanAtLeastOne.bind(this, arg));

			return this;
		}

		public anyOf(rules: Rule<TBranch>[]): Rule<TBranch>
		public anyOf(literals: string[]): Rule<TBranch>
		public anyOf(items: any[]): Rule<TBranch>
		{
			if (typeof items[0] === 'string')
                this._parts.push(this.scanAnyOf.bind(this, items.map(l => new Rule<TBranch>().literal(l))));
			else
				this._parts.push(this.scanAnyOf.bind(this, items));

			return this;
		}

		public between(charA: string, charB: string): Rule<TBranch>
		{
			this._parts.push(this.scanBetween.bind(this, charA.charCodeAt(0), charB.charCodeAt(0)));
			return this;
		}
        
        public eof(): Rule<TBranch>
        {
            this._parts.push(this.scanEof.bind(this));
            return this;
        }

		public maybe(rule: Rule<TBranch>): Rule<TBranch>
		public maybe(text: string): Rule<TBranch>
		public maybe(item: any): Rule<TBranch>
		{
			if (typeof item === 'string')
                this._parts.push(this.scanMaybe.bind(this, new Rule<TBranch>().literal(item)));
			else
				this._parts.push(this.scanMaybe.bind(this, item));

			return this;
		}

		public literal(text: string): Rule<TBranch>
		{
			this._parts.push(this.scanLiteral.bind(this, text));
			return this;
		}
        
        public noneOrMany(rule: Rule<TBranch>): Rule<TBranch>
		public noneOrMany(text: string): Rule<TBranch>
		public noneOrMany(item: any): Rule<TBranch>
		{
            if (typeof item === 'string')
			    this._parts.push(this.scanNoneOrMany.bind(this, new Rule<TBranch>().literal(item)));
            else
			    this._parts.push(this.scanNoneOrMany.bind(this, item));

			return this;
		}	

		public one(rule: Rule<TBranch>): Rule<TBranch>
		{
			this._parts.push(this.scanOne.bind(this, rule));
			return this;
		}

		public scan(code: string): TBranch[]
		{
            const ctx: IScanContext<TBranch> = {
                branches: [],
                code: code,
                hasEof: false,
				errorMessage: "",
                errorIndex: -1,
                index: 0, 
                lexeme: ""
            };

			if (!this.scanRule(ctx))
            {
				this.showCode(ctx.code, ctx.errorIndex);
				throw new Error(`Error on position ${ctx.errorIndex}: ${ctx.errorMessage}`);
            }
            
            if (ctx.hasEof)
                ctx.index--;
            
            if (ctx.index !== ctx.code.length)
            {
				this.showCode(ctx.code, ctx.index);
                throw new Error(`Error: Root rule scan stopped on position ${ctx.index}. No rules matching after this position.`);
            }
			
			return ctx.branches;
		}

        public setBranchFn(fn: (branches: TBranch[], lexeme: string) => TBranch[])
        {
            this._branchFn = fn;
        }
        
        private branch(ctx: IScanContext<TBranch>): IScanContext<TBranch>
		{
			return {
				branches: [],
                code: ctx.code,
                hasEof: ctx.hasEof,
				errorIndex: ctx.errorIndex,
				errorMessage: ctx.errorMessage,
				index: ctx.index,
				lexeme: ""
			};
		}

		private merge(target: IScanContext<TBranch>, source: IScanContext<TBranch>, isRule: boolean = false): boolean
		{
			target.hasEof = source.hasEof;
            target.index = source.index;
			target.lexeme += source.lexeme;

            if (!isRule || this._branchFn === null)
                this.pushList(target.branches, source.branches);
            else
                this.pushList(target.branches, this._branchFn(source.branches, source.lexeme));

			// TODO Is the following true?
            // Always true so we can create a tail call by invocation. 
            return true;
		}

		private pushList<T>( a: T[], b: T[] ): void
		{
			b.forEach( (i: T) => a.push( i ) );
		}

        private scanAllExcept(list: string[], ctx: IScanContext<TBranch>): boolean
		{
            const char: string = ctx.code[ctx.index] || null;

            if (char === null)
            {
                ctx.errorMessage = "End of code.";
                ctx.errorIndex = ctx.index;
                return false;
            }

            if (list.indexOf(char) !== -1)
            {
                ctx.errorMessage = `Character '${char}' is not allowed in 'all except' rule.`;
                ctx.errorIndex = ctx.index;
                return false;
            }

            ctx.lexeme += char;
            ctx.index++;
            return true;
		}

		private scanAlter(list: string[], ctx: IScanContext<TBranch>): boolean
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
            
            ctx.errorMessage = "Alter characters not found on this position.";
            ctx.errorIndex = ctx.index;
            return false;
		}

		private scanAtLeastOne(rule: Rule<TBranch>, ctx: IScanContext<TBranch>): boolean
		{
            let ok: boolean = false;
            const newCtx: IScanContext<TBranch> = this.branch(ctx);

            while (newCtx.index !== newCtx.code.length && rule.scanRule(newCtx))
                ok = true;

            if (ok)
                return this.merge(ctx, newCtx);
            
            this.updateError(ctx, newCtx);
            return false;
		}

		private scanAnyOf(rules: Rule<TBranch>[], ctx: IScanContext<TBranch>): boolean
		{
            const c: number = rules.length;

            for(let i: number = 0; i < c; i++)
            {
                const newCtx: IScanContext<TBranch> = this.branch(ctx);
                const rule: Rule<TBranch> = rules[i];

                if (rule.scanRule(newCtx))
                    return this.merge(ctx, newCtx);
                else
                    this.updateError(ctx, newCtx);
            }

            return false;
		}

		private scanBetween(codeA: number, codeB: number, ctx: IScanContext<TBranch>): boolean
		{
            const char: string = ctx.code[ctx.index] || null;
            
            if (char === null)
            {
                ctx.errorMessage = "End of code.";
                ctx.errorIndex = ctx.index;
                return false;
            }

            const code: number = char.charCodeAt(0);
            
            if (code < codeA || code > codeB)
            {
                ctx.errorMessage = `Expected a character between '${String.fromCharCode( codeA )}' and '${String.fromCharCode( codeB)}'; got a '${char}'`;
                ctx.errorIndex = ctx.index;
                return false;
            }

            ctx.lexeme += char;
            ctx.index++;
            return true;
		}
        
        private scanEof(ctx: IScanContext<TBranch>): boolean
        {
            if (ctx.index === ctx.code.length)
            {
                ctx.hasEof = true;
                ctx.index++;
                return true;
            }
               
            ctx.errorMessage = `Expected an EOF at index ${ctx.index} but the EOF is at ${ctx.code.length}`;
            ctx.errorIndex = ctx.index;    
            return false;
        }

		private scanLiteral(find: string, ctx: IScanContext<TBranch>): boolean
		{
            const len: number = find.length;
            const text: string = ctx.code.substr(ctx.index, len);

            if (find === text)
            {
                ctx.lexeme += text;
                ctx.index += len;
                return true;
            }

            ctx.errorMessage = `Expected '${find}'; got '${text}'.`;
            ctx.errorIndex = ctx.index;
            return false;
		}

		private scanMaybe(rule: Rule<TBranch>, ctx: IScanContext<TBranch>): boolean
		{
            const newCtx: IScanContext<TBranch> = this.branch(ctx);

            if (rule.scanRule(newCtx))
                this.merge(ctx, newCtx);

            return true;
		}

		private scanNoneOrMany(rule: Rule<TBranch>, ctx: IScanContext<TBranch>): boolean
		{
            const newCtx: IScanContext<TBranch> = this.branch(ctx);
            while (rule.scanRule(newCtx)) {}
            return this.merge(ctx, newCtx);
		}

		private scanOne(rule: Rule<TBranch>, ctx: IScanContext<TBranch>): boolean
		{
            const newCtx: IScanContext<TBranch> = this.branch(ctx);

            if (rule.scanRule(newCtx))
                return this.merge(ctx, newCtx);

            this.updateError(ctx, newCtx);
            return false;
		}

		private scanRule(ctx: IScanContext<TBranch>): boolean
		{
			if (this._parts.length === 0)
				throw new Error("Empty rule.");

			const l: number = this._parts.length;
            const newCtx: IScanContext<TBranch> = this.branch(ctx);

            for (let i: number = 0 ; i < l; i++)
            {
                if (this._parts[i](newCtx))
                    continue;

                this.updateError(ctx, newCtx);
				return false;
            }
			
            return this.merge(ctx, newCtx, true);
		}

		private showCode(text: string, position: number): void
        {
            console.error(text.substr(position, 40));
        }

		private updateError(oldCtx: IScanContext<TBranch>, newCtx: IScanContext<TBranch>): void
		{
			if (newCtx.errorIndex < oldCtx.errorIndex)
				return;

			oldCtx.errorIndex = newCtx.errorIndex;
			oldCtx.errorMessage = newCtx.errorMessage;
		}
	}
}