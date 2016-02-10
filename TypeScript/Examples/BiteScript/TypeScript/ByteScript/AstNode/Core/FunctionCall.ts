namespace Abitvin.ByteScript.AstNode
{
    export class FunctionCall implements IAstNode
    {
		private _args: IAstNode[];
        private _id: string;
		private _var: IAstNode;

        constructor( id: string, v: IAstNode, args: IAstNode[] )
        {
            this._args = args;
            this._id = id;
            this._var = v;
        }

        public exit( interperter: Interpreter ): IVariable
        {
            return interperter.getReturn();
        }

        public getChild( index: number, interperter: Interpreter ): IAstNode
        {
            if( index < this._args.length )
                return this._args[index];

            if( index === this._args.length )
                return this._var;
			
            if( index === this._args.length + 1 )
			{
                var fn: IFunction = interperter.popVariable().toFunction();
                
                if( this._args.length !== fn.parameters.length )
                    throw new Error( "Runtime error. There are " + this._args.length + " arguments giving for function `" + this._id + "` but expected " + fn.parameters.length + "." );

                var args: IVariable[] = [];
                var i: number;

				for( i = 0; i < fn.parameters.length; i++ )
                    args.push( interperter.popVariable() );

                for( i = 0; i < fn.parameters.length; i++ )
					interperter.defineVariable( fn.parameters[i], args.pop() );

                return fn.branch;
			}

            return null;
        }

        public isDefinitionScope(): boolean
        {
            return true;
        }
    }
} 