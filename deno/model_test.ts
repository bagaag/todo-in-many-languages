import { assertEquals } from "https://deno.land/std@0.74.0/testing/asserts.ts";
import { Item, Items } from "./model.ts";
import { pathExists } from "./util.ts";

Deno.test("Item", function (): void {
    let item = new Item("test");
    assertEquals(item.description, "test");
    assertEquals(item.isComplete(), false);
    item.complete();
    assertEquals(item.isComplete(), true);
});

Deno.test("Items", function (): void {
    try {
        let items = new Items();
        items.add("test");
        items.add("test 2");
        items.complete(1);
        assertEquals(items.list().length, 1);
        assertEquals(items.completed().length, 1);
        assertEquals(items.list()[0].description, "test");
        let items2 = new Items();
        assertEquals(items2.completed().length, 1);
        assertEquals(items2.list()[0].description, "test");
        items.clear();
        assertEquals(items.completed().length, 0);
    } finally {
        if (pathExists("todo.json")) {
            Deno.removeSync("todo.json");
        }
    }
});