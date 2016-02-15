namespace Abitvin.ByteScript.Type
{
	export class List extends BaseType implements IVariable
	{
        private _items: IVariable[];

		constructor(items: IVariable[])
		{
            this._items = items;
			super();
		}

        public assign(v: IVariable): void
        {
            if (v.constructor !== List)
                return super.assign(v);

            // TODO: I don't like this toList(), do we have a new List or do we share the reference now?
            // TODO: Yes I know by reading the toList() implementation below. But I think to make 'friends'
            // TODO: is better.
            this._items = v.toList();
        }

        public atIndex(rhs: IVariable): IVariable
        {
            switch(rhs.constructor)
            {
                case Boolean:
                case Number:
                    return this._items[rhs.toNumber()];
            }

            return super.atIndex( rhs );
        }

        public assignAtIndex(index: IVariable, v: IVariable): void
        {
            switch( index.constructor )
            {
                case Boolean:
                case Number:
                    let i: number = index.toNumber();

                    if (i < 0 || i >= this._items.length)
                        throw new Error("Runtime out-of-bounds assignment at index " + i + " error.");

                    this._items[i].assign(v);
                    return;
            }

            return super.assignAtIndex( index, v );
        }

        public range(start: IVariable, end: IVariable): IVariable
        {
            const startIndex: number = start.toNumber();
            const endIndex: number = end.toNumber();

            this.checkBounds(startIndex, endIndex);

            return new List(this._items.slice(startIndex, endIndex + 1));
        }

        public rangeFromExpr(start: IVariable): IVariable
        {
            const startIndex: number = start.toNumber();
            const endIndex: number = this._items.length - 1;

            this.checkBounds(startIndex, endIndex);

            return new List(this._items.slice(startIndex, endIndex + 1));
        }

        public rangeToExpr(end: IVariable): IVariable
        {
            const startIndex: number = 0;
            const endIndex: number = end.toNumber();

            this.checkBounds(startIndex, endIndex);

            return new List(this._items.slice(startIndex, endIndex + 1));
        }

        public toList(): IVariable[]
        {
            return this._items.slice(0);
        }

        public toString(): string
        {
            return "[" + this._items.map(i => i.toString()).join(", ") + "]";
        }

        // Helpers.

        private checkBounds(start: number, end: number): void
        {
            if (start < 0 || start >= this._items.length)
                throw new Error("Runtime out-of-bounds start index at " + start + " error.");

            if (end < 0 || end >= this._items.length)
                throw new Error("Runtime out-of-bounds end index at " + end + " error.");

            if (end < start)
                throw new Error("Runtime error. End index of " + end + " smaller then start index of " + start + ".");
        }
	}
}