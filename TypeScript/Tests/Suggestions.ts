///<reference path="References.ts"/>

namespace Abitvin.Tests
{
    class AcRule extends Rule<string, string>
    {
        private _eof: Rule<string, string>;
        private _nl: Rule<string, string>;
        private _ws: Rule<string, string>;
     
        constructor(branchFn?: BranchFn<string>, meta?: string)
        {
            super(branchFn, meta);
            
            this._eof = new Rule<string, string>().eof();
            this._nl = new Rule<string, string>().maybe("\r").literal("\n");
            this._ws = new Rule<string, string>().anyOf(" ", "\t");
        }
        
        public endStmt(): AcRule
        {
            return this.anyOf(this._nl, this._eof); 
        }
        
        public ows(): AcRule
        {
            return this.atLeast(1, this._ws);
        }
        
        public ws(): AcRule
        {
            return this.noneOrMany(this._ws);
        } 
    }
    
    export class Suggestions
    {
        private _grammer: AcRule;
        
        constructor()
        {
            this.initGrammer();
            this.initView();
        }
        
        private initGrammer(): void
        {
            const letter = new AcRule(null, "letter").between("a", "z");
            const digit = new AcRule(null, "digit").between("0", "9");
            const alphaNum = new AcRule(null, "alphaNum").anyOf(letter, digit);
            
            const identifier = new AcRule(null, "identifier").one(letter).noneOrMany(alphaNum);
            const integer = new AcRule(null, "integer").atLeast(1, digit);
            const expression = new AcRule(null, "expression").anyOf(identifier, integer);
            
            const noStmt = new AcRule(null, "noStmt");
            const alertStmt = new AcRule(null, "alertStmt").literal("alert").ows().one(expression);
            const assignmentStmt = new AcRule(null, "assignmentStmt").one(identifier).ws().literal("=").one(expression).ws().literal("+").ws().one(expression);
            const printStmt = new AcRule(null, "printStmt").literal("print").ows().one(expression);
            const varStmt = new AcRule(null, "varStmt").literal("var").ows().one(identifier).ws().literal("=").ws().one(expression);
            const stmt = new AcRule(null, "stmt").ws().anyOf(alertStmt, assignmentStmt, printStmt, varStmt, noStmt).ws().endStmt();
            
            this._grammer = new AcRule((b, l) => [l], "root").noneOrMany(stmt);
        }
        
        private initView(): void
        {
            const suggestionsEl = document.createElement("div");
            
            const inputEl = document.createElement("textarea");
            inputEl.oninput = () =>
            {
                suggestionsEl.innerHTML = "";
                
                try {
                    console.log(this._grammer.scan(inputEl.value))
                }
                catch(e) {
                    e.forEach(error =>
                    {
                        const suggestionEl = document.createElement("div");
                        let content: string = "";
                        
                        error.metaTrail.forEach(meta => {
                            content += meta + " => ";
                        });
                        
                        suggestionEl.innerText = content.substr(0, content.length - 4) + ": " + error.errorMsg;
                        suggestionsEl.appendChild(suggestionEl);
                    });
                }
                
            };
            
            const outputEl = document.createElement("div");
            
            document.body.appendChild(inputEl);
            document.body.appendChild(outputEl);
            document.body.appendChild(suggestionsEl);
            
            inputEl.focus();
        }
    }
}