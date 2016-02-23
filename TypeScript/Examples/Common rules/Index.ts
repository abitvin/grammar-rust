///<reference path="../../Rule.ts"/>

namespace Abitvin
{
    interface IEmpty {}
    
    class ScanOnlyRule extends Rule<string, IEmpty> {}
    
    export class Index
    {
        constructor()
        {
            const passLexemeFn = (b, lexeme: string) => [lexeme];
            
            // Common
            const zero = new ScanOnlyRule().literal("0");
            const one = new ScanOnlyRule().literal("1");
            const bit = new ScanOnlyRule().between("0", "1");
            const digit = new ScanOnlyRule().between("0", "9");
            const af = new ScanOnlyRule().between("a", "f");
            const AF = new ScanOnlyRule().between("A", "F");
            const hex = new ScanOnlyRule().anyOf(digit, af, AF);
            const nonZeroDigit = new ScanOnlyRule().between("1", "9");
            const newline = new ScanOnlyRule().maybe("\r").literal("\n");
            const ws = new ScanOnlyRule().anyOf(" ", "\t");
            
            // Boolean
            const bool = new ScanOnlyRule(passLexemeFn).anyOf("false", "true");
            
            // Binary
            const binary = new ScanOnlyRule(passLexemeFn).literal("b").atLeast(1, bit);
            
            // Signed Integer
            const nonZeroSignedInt = new ScanOnlyRule(passLexemeFn).maybe("-").one(nonZeroDigit).noneOrMany(digit);
            const signedInt = new ScanOnlyRule().anyOf(zero, nonZeroSignedInt);
            
            // Floating point
            const fraction = new ScanOnlyRule().literal(".").atLeast(1, digit);
			const float = new ScanOnlyRule(passLexemeFn).one(signedInt).maybe(fraction);
            
            // String escape characters
            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String
            // https://msdn.microsoft.com/en-us/library/2yfce773(v=vs.94).aspx
            const combineCharsFn = (branches: string[], lexeme: string) => [branches.join("")];
            const parseCharCodeFn = (branches: string[], lexeme: string) => [String.fromCharCode(parseInt(lexeme.substr(2), 16))];
             
            const strEscapeControl = new ScanOnlyRule(passLexemeFn).alter("\\0", "\0", "\\b", "\b", "\\f", "\f", "\\n", "\n", "\\r", "\r", "\\t", "\t", "\\v", "\v", "\\\"", "\"");
            const strEscapeLatin1 = new ScanOnlyRule(parseCharCodeFn).literal("\\x").exact(2, hex);
            const strEscapeUTF16 = new ScanOnlyRule(parseCharCodeFn).literal("\\u").exact(4, hex);
            const strEscapeUnknown = new ScanOnlyRule(passLexemeFn).literal("\\");
            const strAllExceptBs = new ScanOnlyRule(passLexemeFn).allExcept(["\""]);
            const strChar = new ScanOnlyRule().anyOf(strEscapeControl, strEscapeLatin1, strEscapeUTF16, strEscapeUnknown, strAllExceptBs);
            const strValue = new ScanOnlyRule(combineCharsFn).noneOrMany(strChar);
            const str = new ScanOnlyRule().literal("\"").one(strValue).literal("\"");
            
            // Array of the different types above
            const arrItem = new ScanOnlyRule().anyOf(bool, binary, float, signedInt, str);    // Note that `float` must be before `signedInt`
            const arrManyItems = new ScanOnlyRule().noneOrMany(ws).literal(",").noneOrMany(ws).one(arrItem);
            const arr = new ScanOnlyRule(passLexemeFn).literal("[").noneOrMany(ws).one(arrItem).noneOrMany(arrManyItems).noneOrMany(ws).literal("]");
            
            
            this.createExample("Boolean", "- TODO -", "true", bool);
            this.createExample("Binary", "new Rule().atLeastOne(bit)", "b00110110", binary);
            this.createExample("Signed integer", "- TODO -", "-1234567", signedInt);
            this.createExample("Floating point", "- TODO -", "1234.567", float);
            this.createExample("String with strict escape characters", "- TODO -", '"\\u201cAdapt what is useful, reject what is useless, and add what is specifically your own.\\u201d\\n\\u2013 Bruce Lee"', str);
            this.createExample("Array of the different types above", "- TODO -", '[      777, false,     b10011001, "Hello, world",    2323.600   ]', arr);
        }
        
        private createExample(title: string, code: string, value: string, rule: ScanOnlyRule)
        {
            const sectionEl = document.createElement("section");
            const titleEl = document.createElement("h2");
            const codeEl = document.createElement("code");
            const inputEl = document.createElement("input");
            const outputEl = document.createElement("p");
            
            titleEl.textContent = title;
            codeEl.textContent = code;
            inputEl.value = value;
            inputEl.oninput = () => this.redraw(this.scan(rule, inputEl.value), sectionEl, outputEl);
            
            sectionEl.appendChild(titleEl);
            sectionEl.appendChild(codeEl);
            sectionEl.appendChild(inputEl);
            sectionEl.appendChild(outputEl);
            
            document.body.appendChild(sectionEl);
            
            this.redraw(this.scan(rule, inputEl.value), sectionEl, outputEl);
        }
        
        private scan(rule: ScanOnlyRule, text: string): string
        {
            try {
                return rule.scan(text).branches[0]; 
            }
            catch(e) {
                return null;
            }
        }
        
        private redraw(result: string, sectionEl: HTMLElement, outputEl: HTMLParagraphElement): void
        {
            if (result !== null)
            {
                sectionEl.className = "is-valid";
                outputEl.innerHTML = `Result:<br/>${result.replace(/\n/g, "<br/>").replace(/ /g, "&nbsp;")}`;
            }
            else
            {
                sectionEl.className = "is-invalid";
                outputEl.textContent = "Not valid.";
            } 
        }
    }
    
    new Index();
}