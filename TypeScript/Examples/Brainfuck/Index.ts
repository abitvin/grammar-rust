/// <reference path="References.ts"/>

namespace Abitvin
{
    const bc = new Abitvin.Brainfuck.Compiler();

    document.getElementById("btn-run").onclick = () =>
    {
        const input: string = (<HTMLTextAreaElement>document.getElementById("input")).value;
        const code: string = (<HTMLTextAreaElement>document.getElementById("txt-code")).value;
        const program: Brainfuck.Token[] = bc.compile(code);

        const interperter = new Brainfuck.Interperter();
        interperter.run(program, input, document.getElementById("output"));
    };
} 