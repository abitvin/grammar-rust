// TODO Remove me.

namespace Abitvin
{
	interface IScanContext<TBranch>
	{
        branches: TBranch[];
        code: string;
		errorIndex: number;
        errorMessage: string;
        index: number;
        lexeme: string;
	}

    export class Rule2<TBranch>
	{
		protected _branchFn: ( branches: TBranch[], lexeme: string ) => TBranch[];
		protected _parts: {(ctx): boolean}[] = [];

		constructor( branchFn: ( branches: TBranch[], lexeme: string ) => TBranch[] = null )
		{
			this.setBranchFn( branchFn );
		}

        public allExcept( list: string[] ): Rule2<TBranch>
		{
            list.forEach( item => {
                if( item.length !== 1 )
                    throw new Error( "An 'all except' list item can only be a single character." )
            });
            
            this._parts.push( this.scanAllExcept.bind( this, list ) );
			return this;
		}

		public alter( list: string[] ): Rule2<TBranch>
		{
            if( list.length % 2 === 1 )
                throw new Error( "Alter list must be a factor of 2." );

			this._parts.push( this.scanAlter.bind( this, list ) );
			return this;
		}

		public atLeastOne( rule: Rule2<TBranch> ): Rule2<TBranch>
		public atLeastOne( text: string ): Rule2<TBranch>
		public atLeastOne( item: any ): Rule2<TBranch>
		{
            if( typeof item === 'string' )
                this._parts.push( this.scanAtLeastOne.bind( this, new Rule2<TBranch>().literal( item ) ) );
			else
				this._parts.push( this.scanAtLeastOne.bind( this, item ) );

			return this;
		}

		public anyOf( rules: Rule2<TBranch>[] ): Rule2<TBranch>
		public anyOf( literals: string[] ): Rule2<TBranch>
		public anyOf( items: any[] ): Rule2<TBranch>
		{
			if( typeof items[0] === 'string' )
                this._parts.push( this.scanAnyOf.bind( this, items.map( l => new Rule2<TBranch>().literal( l ) ) ) );
			else
				this._parts.push( this.scanAnyOf.bind( this, items ) );

			return this;
		}

		public between( charA: string, charB: string ): Rule2<TBranch>
		{
			this._parts.push( this.scanBetween.bind( this, charA.charCodeAt(0), charB.charCodeAt(0) ) );
			return this;
		}

		public maybe( rule: Rule2<TBranch> ): Rule2<TBranch>
		public maybe( text: string ): Rule2<TBranch>
		public maybe( item: any ): Rule2<TBranch>
		{
			if( typeof item === 'string' )
                this._parts.push( this.scanMaybe.bind( this, new Rule2<TBranch>().literal( item ) ) );
			else
				this._parts.push( this.scanMaybe.bind( this, item ) );

			return this;
		}

		public literal( text: string ): Rule2<TBranch>
		{
			this._parts.push( this.scanLiteral.bind( this, text ) );
			return this;
		}

		public noneOrMany( rule: Rule2<TBranch> ): Rule2<TBranch>
		public noneOrMany( text: string ): Rule2<TBranch>
		public noneOrMany( item: any ): Rule2<TBranch>
		{
            if( typeof item === 'string' )
			    this._parts.push( this.scanNoneOrMany.bind( this, new Rule2<TBranch>().literal( item ) ) );
            else
			    this._parts.push( this.scanNoneOrMany.bind( this, item ) );

			return this;
		}	

		public one( rule: Rule2<TBranch> ): Rule2<TBranch>
		{
			this._parts.push( this.scanOne.bind( this, rule ) );
			return this;
		}

		public scan( code: string ): TBranch[]
		{
            var ctx: IScanContext<TBranch> = {
                branches: [],
                code: code,
				errorMessage: "",
                errorIndex: -1,
                index: 0, 
                lexeme: ""
            };

			if( !this.scanRule( ctx ) )
            {
				this.showCode( ctx.code, ctx.errorIndex );
				throw new Error( "Error on position " + ctx.errorIndex + ": " + ctx.errorMessage );
            }
            else if( ctx.index !== ctx.code.length )
            {
				this.showCode( ctx.code, ctx.index );
                throw new Error( "Error: Root rule scan stopped on position " + ctx.index + ". No rules matching after this position." );
            }
			
			return ctx.branches;
		}

        public setBranchFn( fn: ( branches: TBranch[], lexeme: string ) => TBranch[] )
        {
            this._branchFn = fn;
        }

		private branch( ctx: IScanContext<TBranch> ): IScanContext<TBranch>
		{
			return {
				branches: [],
                code: ctx.code,
				errorIndex: ctx.errorIndex,
				errorMessage: ctx.errorMessage,
				index: ctx.index,
				lexeme: ""
			};
		}

		private merge( target: IScanContext<TBranch>, source: IScanContext<TBranch>, isRule: boolean = false ): boolean
		{
			target.index = source.index;
			target.lexeme += source.lexeme;

            if( !isRule || this._branchFn === null )
                this.pushList( target.branches, source.branches );
            else
                this.pushList( target.branches, this._branchFn( source.branches, source.lexeme ) );

			// Always true so we can create a tail call by invocation.
			return true;
		}

		private pushList<T>( a: T[], b: T[] ): void
		{
			b.forEach( (i: T) => a.push( i ) );
		}

        private scanAllExcept( list: string[], ctx: IScanContext<TBranch> ): boolean
		{
            var char: string = ctx.code[ctx.index] || null;

            if( char === null )
            {
                ctx.errorMessage = "End of code.";
                ctx.errorIndex = ctx.index;
                return false;
            }

            if( list.indexOf( char ) !== -1 )
            {
                ctx.errorMessage = "Character '" + char + "' is not allowed in 'all except' rule.";
                ctx.errorIndex = ctx.index;
                return false;
            }

            ctx.lexeme += char;
            ctx.index++;
            return true;
		}

		private scanAlter( list: string[], ctx: IScanContext<TBranch> ): boolean
		{
            for( var i = 0; i < list.length; i += 2 )
            {
                var find: string = list[i];
                var len: number = find.length;

                if( find === ctx.code.substr( ctx.index, len ) )
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

		private scanAtLeastOne( rule: Rule2<TBranch>, ctx: IScanContext<TBranch> ): boolean
		{
            var ok: boolean = false;
            var newCtx: IScanContext<TBranch> = this.branch( ctx );

            while( newCtx.index !== newCtx.code.length && rule.scanRule( newCtx ) )
                ok = true;

            if( ok )
                return this.merge( ctx, newCtx );
            
            this.updateError( ctx, newCtx );
            return false;
		}

		private scanAnyOf( rules: Rule2<TBranch>[], ctx: IScanContext<TBranch> ): boolean
		{
            var c: number = rules.length;

            for( var i: number = 0; i < c; i++ )
            {
                var newCtx: IScanContext<TBranch> = this.branch( ctx );
                var rule: Rule2<TBranch> = rules[i];

                if( rule.scanRule( newCtx ) )
                    return this.merge( ctx, newCtx );
                else
                    this.updateError( ctx, newCtx );
            }

            return false;
		}

		private scanBetween( codeA: number, codeB: number, ctx: IScanContext<TBranch> ): boolean
		{
            var char: string = ctx.code[ctx.index] || null;
            
            if( char === null )
            {
                ctx.errorMessage = "End of code.";
                ctx.errorIndex = ctx.index;
                return false;
            }

            var code: number = char.charCodeAt( 0 );
            
            if( code < codeA || code > codeB )
            {
                ctx.errorMessage = "Expected a character between '" + String.fromCharCode( codeA ) + "' and '" + String.fromCharCode( codeB) + "'; got a '" + char + "'";
                ctx.errorIndex = ctx.index;
                return false;
            }

            ctx.lexeme += char;
            ctx.index++;
            return true;
		}

		private scanLiteral( find: string, ctx: IScanContext<TBranch> ): boolean
		{
            var len: number = find.length;
            var text: string = ctx.code.substr( ctx.index, len );

            if( find === text )
            {
                ctx.lexeme += text;
                ctx.index += len;
                return true;
            }

            ctx.errorMessage = "Expected '" + find + "'; got '" + text + "'.";
            ctx.errorIndex = ctx.index;
            return false;
		}

		private scanMaybe( rule: Rule2<TBranch>, ctx: IScanContext<TBranch> ): boolean
		{
            var newCtx: IScanContext<TBranch> = this.branch( ctx );

            if( rule.scanRule( newCtx ) )
                this.merge( ctx, newCtx );

            return true;
		}

		private scanNoneOrMany( rule: Rule2<TBranch>, ctx: IScanContext<TBranch> ): boolean
		{
            var newCtx: IScanContext<TBranch> = this.branch( ctx );
            while( rule.scanRule( newCtx ) ) {}
            return this.merge( ctx, newCtx );
		}

		private scanOne( rule: Rule2<TBranch>, ctx: IScanContext<TBranch> ): boolean
		{
            var newCtx: IScanContext<TBranch> = this.branch( ctx );

            if( rule.scanRule( newCtx ) )
                return this.merge( ctx, newCtx );

            this.updateError( ctx, newCtx );
            return false;
		}

		private scanRule( ctx: IScanContext<TBranch> ): boolean
		{
			if( this._parts.length === 0 )
				throw new Error( "This rule has no parts." );

			var l: number = this._parts.length;
            var newCtx: IScanContext<TBranch> = this.branch( ctx );

            for( var i: number = 0 ; i < l; i++ )
            {
                if( this._parts[i]( newCtx ) )
                    continue;

                this.updateError( ctx, newCtx );
				return false;
            }
			
            return this.merge( ctx, newCtx, true );
		}

		private showCode( text: string, position: number ): void
        {
            console.error( text.substr( position, 40 ) );
        }

		private updateError( oldCtx: IScanContext<TBranch>, newCtx: IScanContext<TBranch> ): void
		{
			if( newCtx.errorIndex < oldCtx.errorIndex )
				return;

			oldCtx.errorIndex = newCtx.errorIndex;
			oldCtx.errorMessage = newCtx.errorMessage;
		}
	}
}