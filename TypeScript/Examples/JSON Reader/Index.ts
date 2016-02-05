///<reference path="References.ts"/>

namespace Abitvin
{
    class Index
    {
        constructor()
        {
            const json = "true";
            
            const inputEl = <HTMLTextAreaElement>document.getElementById("inp-json");
            inputEl.value = json;
            inputEl.oninput = () => console.log(JsonReader.read(inputEl.value));
            
            console.log(JsonReader.read(inputEl.value));
        }
    }
    
    new Index();
}