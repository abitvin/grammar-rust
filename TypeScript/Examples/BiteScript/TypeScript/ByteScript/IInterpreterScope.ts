namespace Abitvin.ByteScript
{
    export interface IInterpreterScope
    {
		index: number;
        node: IAstNode;
        parent: IInterpreterScope;
        return: { value: IVariable };
        stackLength: number;
        variables: { [id: string]: IVariable };
    }
} 