/// <reference path="References.ts"/>

namespace Abitvin.Pages
{
    export class Index
    {
        constructor()
        {
            const tfExprEl = <HTMLInputElement>document.getElementById("tf-expr");
            const answerEl = <HTMLDivElement>document.getElementById("answer");

            tfExprEl.onchange = () =>
            {
                if (tfExprEl.value.trim() === "")
                    return;

                console.log("Result", Expression.evaluate(tfExprEl.value));
            };

            tfExprEl.focus();
        }
    }
}