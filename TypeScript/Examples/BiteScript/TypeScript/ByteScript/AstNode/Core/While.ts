namespace Abitvin.ByteScript.AstNode
{
    export class While implements IAstNode
    {
        private _branch: IAstNode;
        private _compare: IAstNode;
        private _index: number;
        
        constructor(compare: IAstNode, branch: IAstNode)
        {
            this._branch = branch;
            this._compare = compare;
        }

        public exit(interperter: Interpreter): IVariable
        {
            return null;
        }

        public getChild(index: number, interperter: Interpreter): IAstNode
        {
			if (index % 2 === 0)
                return this._compare;

            return interperter.popVariable().toBoolean() ? this._branch : null;
        }

        public isDefinitionScope(): boolean
        {
            return false;
        }
    }
} 