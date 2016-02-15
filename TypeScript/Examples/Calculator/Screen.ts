/// <reference path="References.ts"/>

namespace Abitvin.Calculator
{
    enum CharType
    {
        Awnser = 1,
        Expression
    }
    
    interface IChar
    {
        char: string;
        type: CharType;
    }
    
    export class Screen
    {
        private _fontMap: {[char: string]: string};
        private _lineOffset: number;
        private _screenBuffer: IChar[];
        private _screenChars: HTMLDivElement[];
        
        constructor()
        {
            this._lineOffset = 0;
            this._screenBuffer = [];
            this._screenChars = [];
            
            this.initFontMap();
            this.loadImageFont();
            
            const drawCharFn = (c: string) =>
            {
                const marginEl = document.createElement("div");
                
                const charEl = document.createElement("div");
                charEl.className = `char ${this._fontMap[c]}`;
                charEl.appendChild(marginEl); 
                
                for (let i: number = 0; i < 5 * 7; i++)
                    marginEl.appendChild(document.createElement("div"));
                
                document.body.appendChild(charEl);
                return charEl;
            }
            
            
            //let input = "2*3/(100-98)+5";
            
            //for (let i: number = 0; i < input.length; i++)
                //drawCharFn(input[i]);
                
            
            const randomCharFn = () =>
            {
                let r: number = Math.round(Math.random() * 17)
                
                for (let char in this._fontMap)
                    if (--r === 0)
                        return char;
            }
            
            
            //setInterval(() => drawCharFn(randomCharFn()), 500);
            
            
            
            for (let i: number = 0; i < 16 * 8; i++)
                this._screenChars.push(drawCharFn(" "));
            
            
            
            
//            setInterval(() => this.write(randomCharFn()), 500);
            
        }
        
        public backspace(): boolean
        {
            if (this._screenBuffer.length === 0)
                return false;
            
            this._screenBuffer.pop();
            this._screenChars[this._screenBuffer.length].className = "char space";
            this.redraw();
            return true;
        }
        
        public write(char: string): boolean
        {
            if (this._fontMap[char] == null)
                return false;
            
            this._screenBuffer.push({ char: char, type: CharType.Awnser });
            
            if ((this._screenBuffer.length >= 16 * 8) && (this._screenBuffer.length % 16 === 1))
                this._lineOffset++;
            
            this.redraw();
            return true;
        }
        
        public writeAnswer(value: number): void
        {
            let space: number = 16 - (this._screenBuffer.length % 16);
            const answer: string = value.toString(10).substr(0, 16);
            
            while (space-- > 0)
                this.write(" ");
                
            space = 16 - answer.length;
            
            while (space-- > 0)
                this.write(" ");
                
            for (let i: number = 0; i < answer.length; i++)
                this.write(answer[i]);
                
            this.redraw();
        }
        
        
        private redraw(): void
        {
            const chars = this._screenBuffer.slice(this._lineOffset * 16);
            const numChars: number = Math.min(chars.length, 16 * 8);
            let i: number;
            
            for (i = 0 ; i < numChars; i++)
                this._screenChars[i].className = `char ${this._fontMap[chars[i].char]} ${chars[i].type === CharType.Awnser ? "answer" : ""}`;
                
            for (; i < 16 * 8; i++)
                this._screenChars[i].className = `char space`;
        }
        
        
        
        private initFontMap(): void
        {
            this._fontMap = {
                " ": "space",
                "(": "group-open",
                ")": "group-close",
                "*": "star",
                "+": "plus",
                "-": "minus",
                ".": "dot",
                "/": "slash",
                "=": "equal",
                "0": "num-0",
                "1": "num-1",
                "2": "num-2",
                "3": "num-3",
                "4": "num-4",
                "5": "num-5",
                "6": "num-6",
                "7": "num-7",
                "8": "num-8",
                "9": "num-9",
            };
        }
        
        private initStyleSheets(font: HTMLImageElement): void
        {
            const canvas: HTMLCanvasElement = document.createElement("canvas");
            canvas.width = 5;
            canvas.height = 7;
            
            const ctx: CanvasRenderingContext2D = canvas.getContext("2d");
            
            const map: {[char: string]: number[]} = {
                " ": [0, 0],
                "(": [8, 2],
                ")": [9, 2],
                "*": [10, 2],
                "+": [11, 2],
                "-": [13, 2],
                ".": [14, 2],
                "/": [15, 2],
                "=": [13, 3],
                "0": [0, 3],
                "1": [1, 3],
                "2": [2, 3],
                "3": [3, 3],
                "4": [4, 3],
                "5": [5, 3],
                "6": [6, 3],
                "7": [7, 3],
                "8": [8, 3],
                "9": [9, 3]
            };
            
            const marginLeft: number = 2;
            const marginTop: number = 2;
            let css: string = "";
            
            for (const c in map)
            {
                const x = map[c][0] * 7 + marginLeft;
                const y = map[c][1] * 9 + marginTop;
                
                ctx.clearRect(0, 0, 5, 7);
                ctx.drawImage(font, -x, -y);
                css += this.toStyleSheet(this._fontMap[c], ctx);
            }
            
            const style = document.createElement("style");
            style.innerHTML = css;
            
            document.head.appendChild(style);
        }
        
        private loadImageFont(): void
        {
            const font = new Image();
            font.onload = () => this.initStyleSheets(font);
            font.src = "TI-83 font.png";
        }
        
        private toStyleSheet(className: string, ctx: CanvasRenderingContext2D): string
        {
            const imageData: ImageData = ctx.getImageData(0, 0, 5, 7);
            const prefix: string = `div.char.${className} > div > div:nth-child`;
            let css: string = "";
            let index: number = 1;
            let offset: number = 0;
            
            for (let y: number = 0; y < 7; y++)
            for (let x: number = 0; x < 5; x++)
            {
                if (imageData.data[offset] === 0)
                    //css += `${prefix}(${index}){ background-color: black }\n`;
                    css += `${prefix}(${index}){ transform: rotateX(0); }\n`;
                else
                    css += `${prefix}(${index}){ transform: rotateX(180deg); }\n`;
                
                index++;
                offset += 4;
            }
            
            return css;
        }
    }
}