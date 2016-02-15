/// <reference path="References.ts"/>

namespace Abitvin.Calculator
{
    export class Index
    {
        constructor()
        {
            /*
            const tfExprEl = <HTMLInputElement>document.getElementById("tf-expr");
            const answerEl = <HTMLDivElement>document.getElementById("answer");

            tfExprEl.onchange = () =>
            {
                if (tfExprEl.value.trim() === "")
                    return;

                console.log("Result", Expression.evaluate(tfExprEl.value));
            };

            tfExprEl.focus();
            */
            
            
            
            let buffer: string = "";
            const screen = new Screen();
            
            window.addEventListener("keypress", e =>
            {
                switch (e.which)
                {
                    case 8: 
                        screen.backspace();
                        e.preventDefault(); 
                        break;
                        
                    case 13: 
                    {
                        try
                        {
                            screen.writeAnswer(Expression.evaluate(buffer));
                            buffer = "";
                        }
                        catch (e)
                        {
                            screen.writeAnswer(Number.NaN);
                            buffer = "";
                        }
                        
                        break;
                    }
                    
                    default:
                    {
                        const char: string = String.fromCharCode(e.which);
                    
                        if (screen.write(char))
                            buffer += char;    
                    }
                }
                    
                
            });
            
            
        }
    }
}