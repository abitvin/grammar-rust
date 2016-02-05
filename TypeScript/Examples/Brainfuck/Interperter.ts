/// <reference path="References.ts"/>

namespace Abitvin.Brainfuck
{
    export class Interperter
    {
        private _input: string;
        private _memory: number[];
        private _memoryOffset: number;
        private _program: Token[];
        private _programOffset: number;
        private _output: HTMLElement;
        private _stack: number[];
        private _userInput: boolean;
        private _tick: number;

        public run(program: Token[], input: string, output: HTMLElement): void
        {
            this._input = input.replace("\r", "");
            this._memory = [];
            this._memoryOffset = 0;
            this._output = output;
            this._program = program;
            this._programOffset = 0;
            this._stack = [];
            this._userInput = this._input.length === 0;

            this._output.innerHTML = "";
            
            this._tick = new Date().getTime();

            do {
                this.step();
                
                const tock = new Date();
                
                if (tock.getTime() - this._tick > 5000) 
                {
                    if (confirm("Script still running after 5 seconds. Continue?"))
                        this._tick = new Date().getTime();
                    else
                        this._programOffset = this._program.length;
                }
            }
            while(++this._programOffset < this._program.length)
            
            const spanEl = <HTMLSpanElement>document.createElement("span");
            spanEl.textContent = "End-of-program";
            this._output.appendChild(spanEl);
        }

        private getValue(): number
        {
            return this._memory[this._memoryOffset] || 0;
        }
        
        private step(): void
        {
            switch (this._program[this._programOffset])
            {
                case Token.DecrementByte:
                    this._memory[this._memoryOffset] = (this.getValue() - 1) % 256;
                    break;

                case Token.DecrementPointer:
                    this._memoryOffset--;
                    break;

                case Token.EndWhile:
                    this._programOffset = this._stack.pop() - 1;
                    break;

                case Token.IncrementByte:
                    this._memory[this._memoryOffset] = (this.getValue() + 1) % 256;
                    break;

                case Token.IncrementPointer:
                    this._memoryOffset++;
                    break;

                case Token.InputByte:
                {
                    if (this._input.length === 0)
                        this._input = this._userInput ? prompt("//stdin") || "\0" : "\0";
                    
                    if (this._userInput)
                        this._tick = new Date().getTime();
                          
                    this._memory[this._memoryOffset] = this._input.charCodeAt(0) % 256;
                    this._input = this._input.substr(1);
                    break;
                }

                case Token.PrintByte:
                    const spanEl = <HTMLSpanElement>document.createElement("span");
                    spanEl.textContent = String.fromCharCode(this._memory[this._memoryOffset] || 0);
                    this._output.appendChild(spanEl);
                    break;

                case Token.StartWhile:
                {
                    if (this.getValue() !== 0)
                    {
                        this._stack.push(this._programOffset);
                    }
                    else
                    {
                        let loops: number = 0;
                        let found: boolean = false;

                        while (!found)
                        {
                            switch (this._program[++this._programOffset])
                            {
                                case Token.EndWhile: 
                                    found = loops === 0;
                                    loops--;
                                    break;

                                case Token.StartWhile:
                                    loops++; 
                                    break;
                            }
                        }
                    }

                    break;
                }
                
                default:
                    throw new Error("Application error.");
                    break;
            }
        }
    }
}