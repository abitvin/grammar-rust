///<reference path="References.ts"/>

namespace Abitvin
{
    class Index
    {
        constructor()
        {
            const ini = "a=2\nb=3\n";
            
            const inputEl = <HTMLTextAreaElement>document.getElementById("inp-ini");
            inputEl.value = ini;
            inputEl.oninput = () => console.log(IniReader.read(inputEl.value));
            
            console.log(window["test"] = IniReader.read(inputEl.value));
        }
    }
    
    new Index();
}