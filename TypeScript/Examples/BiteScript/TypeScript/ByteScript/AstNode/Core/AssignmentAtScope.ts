namespace Abitvin.ByteScript.AstNode
{
    export class AssignmentAtScope implements IAstNode
    {
        private _expr: IAstNode;
        private _id: string;

        constructor( id: string, expr: IAstNode )
        {
            this._expr = expr;
            this._id = id;
        }

        public exit( interperter: Interpreter ): IVariable
        {
            interperter.setVariable( this._id, interperter.popVariable() );
            return null;
        }

        public getChild( index: number ): IAstNode
        {
            return index === 0 ? this._expr : null;
        }

        public isDefinitionScope(): boolean
        {
            return false;
        }
    }
} 