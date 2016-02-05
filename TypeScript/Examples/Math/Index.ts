/// <reference path="References.ts"/>

namespace Abitvin.Pages
{
    export class Index
    {
        constructor()
        {
            var tfExpr = <HTMLInputElement>document.getElementById("tf-expr");
            var answer = <HTMLDivElement>document.getElementById("answer");

            tfExpr.onchange = () =>
            {
                if (tfExpr.value.trim() === "")
                    return;

                var program: Calc.IAstNode = Calc.Compiler.compile(tfExpr.value);
                var interperter: Calc.Interpreter = new Calc.Interpreter(program);
                var timePerFrame: number = 1000 / 80;       // More then 60hz so we have a some room for other computations.

                var step = () =>
                {
                    var start: number = performance.now();

                    while (interperter.step())
                    {
                        if (performance.now() - timePerFrame < start)
                            continue;

                        setTimeout(step, 0);
                        break;
                    }
                };

                step();
            };

            tfExpr.focus();
        }
    }
}