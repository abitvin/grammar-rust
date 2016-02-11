namespace Abitvin.ByteScript
{
    export interface IInterperterScope
    {
		index: number;
        node: IAstNode;
        parent: IInterperterScope;
        return: { value: IVariable };
        stackLength: number;
        variables: { [id: string]: IVariable };
    }
} 