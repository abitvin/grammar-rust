/// <reference path="References.ts"/>

namespace Abitvin
{
    interface IScanContext
    {
        ini?: IIni;
        name?: string;
        value?: string;
    }
    
    interface IIni
    {
        [sectionOrProp: string]: string|IIni;
    }
    
    export class IniReader
    {
        private static _root: Rule<IScanContext>;
        private static _initialized: boolean = false;
        
        public static initialize(): void
        {
            const ini: IIni = {};
            let currentScope = ini;
            
            // Common
            const ws = new Rule<IScanContext>().anyOf([" ", "\t"]);
            
            // Comment
            const commentChar = new Rule<IScanContext>().allExcept(["\r", "\n"]);
            const comment = new Rule<IScanContext>().literal(";").noneOrMany(commentChar);
            
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
            
            const propNameChar = new Rule<IScanContext>().allExcept(["[", "]", "\r", "\n", "="]); 
            const propName = new Rule<IScanContext>(propNameFn).atLeastOne(propNameChar);
            const propValueChar = new Rule<IScanContext>().allExcept(["\r", "\n"]); 
            const propValue = new Rule<IScanContext>(propValueFn).atLeastOne(propValueChar);
            const prop = new Rule<IScanContext>(propFn).one(propName).literal("=").one(propValue);
            
            // Section
            const sectionRootFn = (b, l) => { currentScope = ini; return []; };
            
            const sectionScopeFn = (b, l: string) =>
            {
                if (typeof currentScope[l] === "string")
                    throw new Error(`Section by name '${l}' already used by a property.`);
                
                currentScope = currentScope[l] == null ? currentScope[l] = {} : <IIni>currentScope[l];
                return [];
            };
            
            const sectionChar = new Rule<IScanContext>().allExcept(["[", "]", "\r", "\n", " ", "."]);
            const sectionScope = new Rule<IScanContext>(sectionScopeFn).atLeastOne(sectionChar);
            const sectionScopeLoop = new Rule<IScanContext>().literal(".").one(sectionScope);
            const sectionRoot = new Rule<IScanContext>(sectionRootFn).literal("[");
            const section = new Rule<IScanContext>().one(sectionRoot).one(sectionScope).noneOrMany(sectionScopeLoop).literal("]");
            
            // Content
            const content = new Rule<IScanContext>().anyOf([comment, prop, section]);
            const eof = new Rule<IScanContext>().eof();
            const nl = new Rule<IScanContext>().maybe("\r").literal("\n");
            const line = new Rule<IScanContext>().noneOrMany(ws).maybe(content).anyOf([nl, eof]);
            
            // Root
            this._root = new Rule<IScanContext>(() => [{ ini: ini }]).noneOrMany(line);
        }
        
        public static read(input: string): IIni
        {
            if (!this._initialized)
                this.initialize();
            
            return this._root.scan(input)[0].ini;
        }
    }
}