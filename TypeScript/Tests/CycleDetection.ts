///<reference path="References.ts"/>

namespace Abitvin.Tests
{
    interface IEmpty {}
    interface ITest {}
    
    let unique: number = 0;
    
    class CdRule extends Rule<ITest, IEmpty>
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
            const dash = new CdRule().literal("-");
            const star = new CdRule().literal("*");
            const eof = new CdRule().eof();
            
            //const beginOfStmt = new CdRule().noneOrMany(star);
            const beginOfStmt = new CdRule().noneOrMany(dash);
            //const endOfStmt = new CdRule().noneOrMany(dash).maybe(eof);
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
            
            //if (!rootRule1.scan("foofoo").isSuccess) console.error("Test failed");
            //if (!rootRule1.scan("foo-foo").isSuccess) console.error("Test failed");
            //if (!rootRule1.scan("foo-foo-").isSuccess) console.error("Test failed");
            //if (!rootRule2.scan("foofoo").isSuccess) console.error("Test failed");
            //if (!rootRule2.scan("foo-foo").isSuccess) console.error("Test failed");
            //if (!rootRule2.scan("foo-foo-").isSuccess) console.error("Test failed");
            
            for (const code of ["foo", "*foo", "**foo", "foo--", "*foo-"])
                if (!fooStmt3.scan(code).isSuccess) 
                    console.error(`Test "${code}" failed`);
            
            
            //this._rootRule = rootRule3;
            //this._rootRule = fooStmt3;
            
            
            const a = new CdRule().noneOrMany(star);
            const b = new CdRule().noneOrMany(a);
            
            this._rootRule = b;
        }
        
        public test(code: string): RuleResult<ITest, IEmpty>
        {
            //return this._rootRule.scan(code);
            //return this._rootRule.scan("foo");
            
            return this._rootRule.scan(code);
        }
    }
}