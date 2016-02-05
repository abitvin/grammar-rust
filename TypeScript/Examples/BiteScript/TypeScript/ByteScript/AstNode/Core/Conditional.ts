module Abitvin.ByteScript.AstNode
{
    export class Conditional implements IAstNode
    {
        private _branch: IAstNode;
        private _compare: IAstNode;
		private _return: IVariable;

        constructor( compare: IAstNode, branch: IAstNode )
        {
            this._branch = branch;
            this._compare = compare;
        }

        public exit( interperter: Interpreter ): IVariable
        {
            return this._return;
        }

        public getChild( index: number, interperter: Interpreter ): IAstNode
        {
            if( index === 0 )
                return this._compare;
            
            if( index === 1 )
			{
				this._return = interperter.popVariable();
                return this._return.toBoolean() ? this._branch : null;
			}

            return null;
        }

        public isDefinitionScope(): boolean
        {
            return false;
        }
    }
} 