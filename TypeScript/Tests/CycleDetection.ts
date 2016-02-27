///<reference path="References.ts"/>

namespace Abitvin.Tests
{
    interface IEmpty {}
    //interface ITest {}
    
    let unique: number = 0;
    
    class CdRule extends Rule<string, IEmpty>
    {
        private _id: number;
        
        constructor(b?, l?)
        {
            super(b, l);
            this._id = ++unique;
        }
    }
    
    export class CycleDetection
    {
        private _rootRule: CdRule;
        
        constructor()
        {
            const dash = new CdRule(() => ["-"]).literal("-");
            const star = new CdRule(() => ["*"]).literal("*");
            const eof = new CdRule().eof();
            
            const beginOfStmt = new CdRule().noneOrMany(dash);
            const endOfStmt = new CdRule().noneOrMany(dash);
            
            const fooStmt1 = new CdRule().one(beginOfStmt).literal("foo").one(endOfStmt);
            const fooStmt2 = new CdRule().noneOrMany(dash).literal("foo").noneOrMany(dash);
            const fooStmt3 = new CdRule().noneOrMany(star).literal("foo").noneOrMany(dash);
            
            const rootRule1 = new CdRule().noneOrMany(fooStmt1);
            const rootRule2 = new CdRule().noneOrMany(fooStmt2);
            const rootRule3 = new CdRule().noneOrMany(fooStmt3);
            
            for (const code of ["foofoo", "foo-foo", "foo-foo-"])
                if (!rootRule1.scan(code).isSuccess) 
                    console.error(`Test "${code}" failed`);
                    
            for (const code of ["foofoo", "foo-foo", "foo-foo-"])
                if (!rootRule2.scan(code).isSuccess) 
                    console.error(`Test "${code}" failed`);
            
            for (const code of ["foo", "*foo", "**foo", "foo--", "*foo-"])
                if (!fooStmt3.scan(code).isSuccess) 
                    console.error(`Test "${code}" failed`);
            
            const dashes = new CdRule((b) => [`#${b.join()}#`]).noneOrMany(dash);
            const someDashes = new CdRule((b) => [`[${b.join()}]`]).between(2, 4, dashes);
            
            if (!someDashes.scan("").isSuccess) 
                console.error(`Test "" failed`);
            
            for (const code of ["-", "--", "---", "----", "-----"])
                if (someDashes.scan(code).isSuccess) 
                    console.error(`Test "${code}" failed`);
                    

            const someDashes2 = new CdRule((b) => [`{${b.join()}}`]).noneOrMany(dashes);
            
            if (!someDashes2.scan("").isSuccess) 
                console.error(`Test "" failed`);
            
            for (const code of ["-", "--", "---"])
                if (someDashes2.scan(code).isSuccess) 
                    console.error(`Test "${code}" failed`);
                    
            const someDashes3 = new CdRule((b) => [`{${b.join()}}`]).noneOrMany(someDashes2);
            
            if (!someDashes3.scan("").isSuccess) 
                console.error(`Test "" failed`);
            
            for (const code of ["-", "--", "---"])
                if (someDashes3.scan(code).isSuccess) 
                    console.error(`Test "${code}" failed`);
                    
            this._rootRule = someDashes3;
        }
        
        public test(code: string): RuleResult<string, IEmpty>
        {
            //return this._rootRule.scan(code);
            //return this._rootRule.scan("foo");
            return this._rootRule.scan(code);
        }
    }
}