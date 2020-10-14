export class Item {
    description: string;
    _completed: Date = new Date(0);

    constructor(descr: string) {
        this.description = descr;
    }

    complete(): void {
        this._completed = new Date();
    }

    completed(): boolean {
        return this._completed.getTime() > new Date(0).getTime();
    }
}
