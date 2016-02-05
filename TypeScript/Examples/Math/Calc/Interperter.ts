/// <reference path="../References.ts"/>

module Abitvin.Calc
{
    export class Interpreter
    {
        private _current: IInterpreterScope;
        private _varStack: IVariable[] = [];

        constructor(program: IAstNode)
        {
            this._current = {
                index: 0,
                node: program,
                parent: null,
                return: { value: new Type.Null() },
                stackLength: 0
            };
        }

        public getReturn(): IVariable
        {
            return this._current.return.value;
        }

        public popVariable(): IVariable
        {
            var popped: IVariable = this._varStack.pop();

			if (!popped)
				throw new Error("Runtime error. Popped to many.");

            console.log("Popped", popped.toString());
			return popped;
        }

        public pushVariable(v: IVariable): void
        {
            this._varStack.push(v);
            console.log("Pushed", v.toString());
        }

        public step(): boolean
        {
            var node: IAstNode;
            //var exitNode: boolean = this._current.return.value.constructor !== Type.Null;
            var exitNode: boolean = false;

            if (!exitNode)
            {
				node = this._current.node.getChild(this._current.index, this);
                exitNode = node === null;
            }

            if (exitNode)
            {
                var v: IVariable = this._current.node.exit(this);
                
                console.log("Exit(" + this._current.stackLength + ") `" + this._current.node.constructor["name"] + "`");

                while (this._varStack.length > this._current.stackLength)
                {
                    console.log("Clean up pop after return");
                    this.popVariable();
                }
                    
                if (v !== null)
                {
					this.pushVariable(v);
                    console.log("=> " + v.toString());
                }

                this._current = this._current.parent;

                if (this._current === null)
                {
                    console.log("Stack", this._varStack.map(v => v.toString()));
                    return false;
                }

                this._current.index++;
            }
            else
            {
                var pushScope: IInterpreterScope = {
					index: 0,
                    node: node,
                    parent: this._current,
                    return: this._current.return,
                    stackLength: this._varStack.length
                };

                console.log("Enter(" + pushScope.stackLength + ") `" + node.constructor["name"] + "`");
                this._current = pushScope;
            }

            console.log("Stack", this._varStack.map(v => v.toString()));
            return true;
        }
    }
} 