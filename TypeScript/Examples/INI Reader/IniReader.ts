/// <reference path="References.ts"/>

namespace Abitvin
{
    interface IEmpty {}
    
    interface IIni
    {
        [sectionOrProp: string]: string|IIni;
    }
    
    interface IScanContext
    {
        ini?: IIni;
        name?: string;
        value?: string;
    }
    
    class IniRule extends Rule<IScanContext, IEmpty>
    {
        private _ws = new Rule<IScanContext, IEmpty>().anyOf(" ", "\t");
        
        public ws(): this
        {
            return this.noneOrMany(this._ws);
        }
    } 
    
    export class IniReader
    {
        private static _root: IniRule;
        private static _initialized: boolean = false;
        
        public static initialize(): void
        {
            const ini: IIni = {};
            let currentScope = ini;
            
            // Comment
            const commentChar = new IniRule().allExcept("\r", "\n");
            const comment = new IniRule().literal(";").noneOrMany(commentChar);
            
            // Property
            const propNameFn = (b, l) => [{ name: l }];
            const propValueFn = (b, l) => [{ value: l }];
            const propFn = (b, l) => 
            {
                const name: string = b[0].name;
                const value: string = b[1].value;
                
                if (typeof currentScope[name] === "object")
                    throw new Error(`Section already exists with propertyname '${name}'.`);
                    
                currentScope[name] = value;
                return [];
            };
            
            const propNameChar = new IniRule().allExcept("[", "]", "\r", "\n", "="); 
            const propName = new IniRule(propNameFn).atLeast(1, propNameChar);
            const propValueChar = new IniRule().allExcept("\r", "\n"); 
            const propValue = new IniRule(propValueFn).atLeast(1, propValueChar);
            const prop = new IniRule(propFn).one(propName).literal("=").one(propValue);
            
            // Section
            const sectionRootFn = (b, l) => { currentScope = ini; return []; };
            
            const sectionScopeFn = (b, l: string) =>
            {
                if (typeof currentScope[l] === "string")
                    throw new Error(`Section by name '${l}' already used by a property.`);
                
                currentScope = currentScope[l] == null ? currentScope[l] = {} : <IIni>currentScope[l];
                return [];
            };
            
            const sectionChar = new IniRule().allExcept("[", "]", "\r", "\n", " ", ".");
            const sectionScope = new IniRule(sectionScopeFn).atLeast(1, sectionChar);
            const sectionScopeLoop = new IniRule().literal(".").one(sectionScope);
            const sectionRoot = new IniRule(sectionRootFn).literal("[");
            const section = new IniRule().one(sectionRoot, sectionScope).noneOrMany(sectionScopeLoop).literal("]");
            
            // Content
            const content = new IniRule().anyOf(comment, prop, section);
            const eof = new IniRule().eof();
            const nl = new IniRule().maybe("\r").literal("\n");
            const line = new IniRule().ws().maybe(content).anyOf(nl, eof);
            
            // Root
            this._root = new IniRule(() => [{ ini: ini }]).noneOrMany(line);
        }
        
        public static read(input: string): IIni
        {
            if (!this._initialized)
                this.initialize();
            
            return this._root.scan(input).branches[0].ini;
        }
    }
}