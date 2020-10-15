import {pathExists} from "./util.ts"

// Item represents a task in the to do list.
export class Item {
    description: string;
    _completed: Date = new Date(0);

    constructor(descr: string) {
        this.description = descr;
    }

    // completes this item
    complete(): void {
        this._completed = new Date();
    }

    // returns date this item was completed
    completed(): Date {
        return this._completed
    }

    // returns true if this item is completed
    isComplete(): boolean {
        return this._completed.getTime() > new Date(0).getTime();
    }
}

// Items represents the to do list.
export class Items {
    private all: Item[] = [];

    constructor() {
        this.load();
    }

    // reads this list from disk
    private load(): void {
        let path = "todo.json";
        if (pathExists(path)) {
            let json = Deno.readTextFileSync("todo.json");
            let all = JSON.parse(json);
            all.forEach((obj: any) => {
                let item = new Item(obj.description);
                let completed = new Date(obj._completed);
                item._completed = completed;
                this.all.push(item);                
            });
        }
    }

    // writes this list to disk
    save(): void {
        let json = JSON.stringify(this.all);
        Deno.writeTextFileSync("todo.json", json);
    }

    // adds an item to the list
    add(descr: string): void {
        this.all.push(new Item(descr));
        this.save();
    }

    // completes an item at the given zero based list() index
    complete(ix: number): void {
        let current = 0;
        this.all.forEach(item => {
            if (!item.isComplete()) {
                if (current == ix) {
                    item.complete();
                    this.save();
                    return;
                }
                current++;
            }
        });
    }

    // removes completed items from the list
    clear(): void {
        this.all = this.list();
        this.save();
    }

    // returns the active list of items
    list(): Item[] {
        let ret: Item[] = [];
        this.all.forEach(item => {
            if (!item.isComplete()) {
                ret.push(item);
            }
        });
        return ret;
    }

    completed(): Item[] {
        let ret: Item[] = [];
        this.all.forEach(item => {
            if (item.isComplete()) {
                ret.push(item);
            }
        });
        return ret;
    }
    
}