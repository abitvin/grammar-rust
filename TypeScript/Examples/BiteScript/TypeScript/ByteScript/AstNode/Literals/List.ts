namespace Abitvin.ByteScript.AstNode
{
    export class List implements IAstNode
    {
        protected _items: IAstNode[];

        constructor( items: IAstNode[] )
        {
            this._items = items;
        }

        public exit( interperter: Interpreter ): IVariable
        {
            var items: IVariable[] = [];
            this._items.forEach( () => items.unshift( interperter.popVariable() ) );
            return new Type.List( items );
        }

        public getChild( index: number ): IAstNode
        {
            return this._items[index] ||  null;
        }

        public isDefinitionScope(): boolean
        {
            return false;
        }
    }
} 