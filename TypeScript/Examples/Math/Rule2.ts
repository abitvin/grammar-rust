module Abitvin
{
    export interface IFailed<TBranch, TMetaData>
    {
        errorMsg: string[];
		index: number;
        metaData: TMetaData[][];
    }
    
    export interface IScanContext<TBranch, TMetaData>
	{
        branches: TBranch[];
        code: string;
		failed: IFailed<TBranch, TMetaData>;
        index: number;
        lexeme: string;
        metaData: TMetaData[];
	}

    export interface IScanner<TBranch, TMetaData>
    {
        scan( ctx: IScanContext<TBranch, TMetaData> ): boolean;
    }

    export interface MergeFn<TBranch, TMetaData>
    {
        ( branches: TBranch[], lexeme: string ): TBranch[];
    }

    function branch<TBranch, TMetaData>( ctx: IScanContext<TBranch, TMetaData> ): IScanContext<TBranch, TMetaData>
	{
        return {
			branches: [],
            code: ctx.code,
            failed: ctx.failed,
			index: ctx.index,
			lexeme: "",
            metaData: ctx.metaData
		};
	}

    function merge<TBranch, TMetaData>( target: IScanContext<TBranch, TMetaData>, source: IScanContext<TBranch, TMetaData>, mergeFn: MergeFn<TBranch, TMetaData> = null ): boolean
	{
		target.index = source.index;
		target.lexeme += source.lexeme;

        if( mergeFn === null )
            pushList<TBranch>( target.branches, source.branches );
        else
            pushList<TBranch>( target.branches, mergeFn( source.branches, source.lexeme ) );

		return true;
	}

    function pushList<T>( a: T[], b: T[] ): void
	{
		b.forEach( (i: T) => a.push( i ) );
	}

    function updateError<TBranch, TMetaData>( ctx: IScanContext<TBranch, TMetaData>, errorMsg: string ): boolean
	{
        var failed: IFailed<TBranch, TMetaData> = ctx.failed;
        
        if( ctx.index === failed.index )
        {
			failed.errorMsg.push( errorMsg );
            failed.metaData.push( ctx.metaData.slice(0) );
        }
        else if( ctx.index > failed.index )
        {
			failed.errorMsg = [errorMsg];
			failed.index = ctx.index;
            failed.metaData = [ctx.metaData.slice(0)];
        }

        return false;
	}
    
    // TODO Remove me
    export class Rule2<TBranch, TMetaData> implements IScanner<TBranch, TMetaData>
	{
		private _mergeFn: MergeFn<TBranch, TMetaData>;
        private _metaData: TMetaData;
		private _parts: IScanner<TBranch, TMetaData>[] = [];

		constructor( mergeFn: MergeFn<TBranch, TMetaData> = null, metaData: TMetaData = null )
		{
			this._mergeFn = mergeFn;
            this._metaData = metaData;
		}

        public allExcept( list: string[] ): Rule2<TBranch, TMetaData>
		{
            list.forEach( item => {
                if( item.length !== 1 )
                    throw new Error( "An 'all except' list item can only be a single character." );
            });
            
            this._parts.push( new AllExcept<TBranch, TMetaData>( list ) );
			return this;
		}

		public alter( list: string[] ): Rule2<TBranch, TMetaData>
		{
            if( list.length % 2 === 1 )
                throw new Error( "Alter list must be a factor of 2." );

			this._parts.push( new Alter<TBranch, TMetaData>( list ) );
			return this;
		}

		public atLeastOne( rule: Rule2<TBranch, TMetaData> ): Rule2<TBranch, TMetaData>;
		public atLeastOne( text: string ): Rule2<TBranch, TMetaData>;
		public atLeastOne( item: any ): Rule2<TBranch, TMetaData>
		{
            if( typeof item === "string" )
                this._parts.push( new AtLeastOne<TBranch, TMetaData>( new Rule2<TBranch, TMetaData>().literal( item ) ) );
			else
				this._parts.push( new AtLeastOne<TBranch, TMetaData>( item ) );

			return this;
		}

		public anyOf( rules: Rule2<TBranch, TMetaData>[] ): Rule2<TBranch, TMetaData>;
		public anyOf( literals: string[] ): Rule2<TBranch, TMetaData>;
		public anyOf( items: any[] ): Rule2<TBranch, TMetaData>
		{
			if( typeof items[0] === "string" )
                this._parts.push( new AnyOf<TBranch, TMetaData>( items.map( l => new Rule2<TBranch, TMetaData>().literal( l ) ) ) );
			else
				this._parts.push( new AnyOf<TBranch, TMetaData>( items ) );

			return this;
		}

		public between( charA: string, charB: string ): Rule2<TBranch, TMetaData>
		{
			this._parts.push( new Between<TBranch, TMetaData>( charA.charCodeAt(0), charB.charCodeAt(0) ) );
			return this;
		}

		public evaluate( code: string ): TBranch[]
		{
            var ctx: IScanContext<TBranch, TMetaData> = {
                branches: [],
                code: code,
				failed: {
                    errorMsg: [],
					index: 0,
                    metaData: []
                },
                index: 0, 
                lexeme: "",
                metaData: []
            };

			if( !this.scan( ctx ) )
            {
                throw ctx.failed;
            }
            else if( ctx.index !== ctx.code.length )
            {
                // TODO: Make this work on the new implementation.
				this.showCode( ctx.code, ctx.index );

                // TODO: Can we be more specific? Like showing the code where it stopped en how many characters are left to scan?
                throw new Error( "Error: Root rule scan stopped on position " + ctx.index + ". No rules matching after this position." );
            }
			
			return ctx.branches;
		}
        
        public literal( text: string ): Rule2<TBranch, TMetaData>
		{
			this._parts.push( new Literal<TBranch, TMetaData>( text ) );
			return this;
		}

		public maybe( rule: Rule2<TBranch, TMetaData> ): Rule2<TBranch, TMetaData>;
		public maybe( text: string ): Rule2<TBranch, TMetaData>;
		public maybe( item: any ): Rule2<TBranch, TMetaData>
		{
			if( typeof item === "string" )
                this._parts.push( new Maybe<TBranch, TMetaData>( new Rule2<TBranch, TMetaData>().literal( item ) ) );
			else
				this._parts.push( new Maybe<TBranch, TMetaData>( item ) );

			return this;
		}

		public noneOrMany( rule: Rule2<TBranch, TMetaData> ): Rule2<TBranch, TMetaData>;
		public noneOrMany( text: string ): Rule2<TBranch, TMetaData>;
		public noneOrMany( item: any ): Rule2<TBranch, TMetaData>
		{
            if( typeof item === "string" )
			    this._parts.push( new NoneOrMany<TBranch, TMetaData>( new Rule2<TBranch, TMetaData>().literal( item ) ) );
            else
			    this._parts.push( new NoneOrMany<TBranch, TMetaData>( item ) );

			return this;
		}	

		public one( rule: Rule2<TBranch, TMetaData> ): Rule2<TBranch, TMetaData>
		{
			this._parts.push( new One<TBranch, TMetaData>( rule ) );
			return this;
		}

		public scan( ctx: IScanContext<TBranch, TMetaData> ): boolean
		{
			var l: number = this._parts.length;

			if( l === 0 )
				throw new Error( "This rule has no parts." );
			
            var newCtx: IScanContext<TBranch, TMetaData> = branch<TBranch, TMetaData>( ctx );

            if( this._metaData !== null )
                newCtx.metaData.push( this._metaData );

            for( var i: number = 0; i < l; i++ )
            {
                if( this._parts[i].scan( newCtx ) )
                    continue;
				
                if( this._metaData !== null )
                    newCtx.metaData.pop();

                return false;
            }
			
            return merge<TBranch, TMetaData>( ctx, newCtx, this._mergeFn );
		}

		private showCode( text: string, position: number ): void
        {
            console.error( text.substr( position, 40 ) );
        }
	}

    class AllExcept<TBranch, TMetaData> implements IScanner<TBranch, TMetaData>
    {
        private _list: string[];

        constructor( list: string[] )
        {
            this._list = list;
        }
        
        public scan( ctx: IScanContext<TBranch, TMetaData> ): boolean
		{
            var char: string = ctx.code[ctx.index] || null;

            if( char === null )
                return updateError<TBranch, TMetaData>( ctx, "End of code." );;

            if( this._list.indexOf( char ) !== -1 )
                return updateError<TBranch, TMetaData>( ctx, "Character '" + char + "' is not allowed in 'all except' rule." );

            ctx.lexeme += char;
            ctx.index++;
            return true;
		}
    }

    class Alter<TBranch, TMetaData> implements IScanner<TBranch, TMetaData>
    {
        private _list: string[];

        constructor( list: string[] )
        {
            this._list = list;
        }
        
        public scan( ctx: IScanContext<TBranch, TMetaData> ): boolean
		{
            for( var i = 0; i < this._list.length; i += 2 )
            {
                var find: string = this._list[i];
                var len: number = find.length;

                if( find === ctx.code.substr( ctx.index, len ) )
                {
                    ctx.lexeme += this._list[i+1];
                    ctx.index += len;
                    return true;
                }
            }
            
            return updateError<TBranch, TMetaData>( ctx, "Alter characters not found on this position." );
		}
	}

    class AtLeastOne<TBranch, TMetaData> implements IScanner<TBranch, TMetaData>
    {
        private _rule: Rule2<TBranch, TMetaData>;
        
        constructor( rule: Rule2<TBranch, TMetaData> )
        {
            this._rule = rule;
        }
        
        public scan( ctx: IScanContext<TBranch, TMetaData> ): boolean
		{
            var ok: boolean = false;
            var newCtx: IScanContext<TBranch, TMetaData> = branch<TBranch, TMetaData>( ctx );

			while( this._rule.scan( newCtx ) )
                ok = true;

            if( ok )
                return merge<TBranch, TMetaData>( ctx, newCtx );
            
            return false;
		}
    }

    class AnyOf<TBranch, TMetaData> implements IScanner<TBranch, TMetaData>
    {
        private _rules: Rule2<TBranch, TMetaData>[];

        constructor( rules: Rule2<TBranch, TMetaData>[] )
        {
            this._rules = rules;
        }
        
        public scan( ctx: IScanContext<TBranch, TMetaData> ): boolean
		{
            var c: number = this._rules.length;

            for( var i: number = 0; i < c; i++ )
            {
                var rule: Rule2<TBranch, TMetaData> = this._rules[i];
                var newCtx: IScanContext<TBranch, TMetaData> = branch<TBranch, TMetaData>( ctx );

                if( rule.scan( newCtx ) )
                    return merge<TBranch, TMetaData>( ctx, newCtx );
            }

            return false;
		}
    }

    class Between<TBranch, TMetaData> implements IScanner<TBranch, TMetaData>
    {
        private _codeA: number;
        private _codeB: number;

        constructor( codeA: number, codeB: number )
        {
            this._codeA = codeA;
            this._codeB = codeB;
        }
        
        public scan( ctx: IScanContext<TBranch, TMetaData> ): boolean
		{
            var char: string = ctx.code[ctx.index] || null;
            
            if( char === null )
                return updateError<TBranch, TMetaData>( ctx, "End of code." );

            var code: number = char.charCodeAt( 0 );
            
            if( code < this._codeA || code > this._codeB )
                return updateError<TBranch, TMetaData>( ctx, "Expected a character between '" + String.fromCharCode( this._codeA ) + "' and '" + String.fromCharCode( this._codeB ) + "'; got a '" + char + "'" );

            ctx.lexeme += char;
            ctx.index++;
            return true;
		}
    }

    class Literal<TBranch, TMetaData> implements IScanner<TBranch, TMetaData>
    {
        private _find: string;

        constructor( find: string )
        {
            this._find = find;
        }
        
        public scan( ctx: IScanContext<TBranch, TMetaData> ): boolean
		{
            var i: number = 0;

            while( i < this._find.length )
            {
                var c: string = ctx.code.charAt( ctx.index );

                if( c === "" )
                    return updateError<TBranch, TMetaData>( ctx, "End of code." );

                if( c !== this._find[i] )
                    return updateError<TBranch, TMetaData>( ctx, "The literal '" + this._find + "' not found on the current location." );

                ctx.index++;
                i++;
            }

            ctx.lexeme += this._find;
            return true;
		}
    }

    class Maybe<TBranch, TMetaData> implements IScanner<TBranch, TMetaData>
    {
        private _rule: Rule2<TBranch, TMetaData>;
        
        constructor( rule: Rule2<TBranch, TMetaData> )
        {
            this._rule = rule;
        }
        
        public scan( ctx: IScanContext<TBranch, TMetaData> ): boolean
		{
            var newCtx: IScanContext<TBranch, TMetaData> = branch<TBranch, TMetaData>( ctx );

            if( this._rule.scan( newCtx ) )
                merge<TBranch, TMetaData>( ctx, newCtx );

            return true;
		}
    }

    class NoneOrMany<TBranch, TMetaData> implements IScanner<TBranch, TMetaData>
    {
        private _rule: Rule2<TBranch, TMetaData>;
        
        constructor( rule: Rule2<TBranch, TMetaData> )
        {
            this._rule = rule;
        }
        
        public scan( ctx: IScanContext<TBranch, TMetaData> ): boolean
		{
            var newCtx: IScanContext<TBranch, TMetaData> = branch<TBranch, TMetaData>( ctx );
            while( this._rule.scan( newCtx ) ) {}
            return merge<TBranch, TMetaData>( ctx, newCtx );
		}
    }

    class One<TBranch, TMetaData> implements IScanner<TBranch, TMetaData>
    {
        private _rule: Rule2<TBranch, TMetaData>;
        
        constructor( rule: Rule2<TBranch, TMetaData> )
        {
            this._rule = rule;
        }
        
        public scan( ctx: IScanContext<TBranch, TMetaData> ): boolean
		{
            var newCtx: IScanContext<TBranch, TMetaData> = branch<TBranch, TMetaData>( ctx );

            if( this._rule.scan( newCtx ) )
                return merge<TBranch, TMetaData>( ctx, newCtx );

            return false;
		}
    }
}