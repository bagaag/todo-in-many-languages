import { assertEquals } from "https://deno.land/std@0.74.0/testing/asserts.ts";
import { Item } from "./model.ts";

Deno.test("Item", function (): void {
    let item = new Item("test");
    assertEquals(item.description, "test");
    assertEquals(item.completed(), false);
    item.complete();
    assertEquals(item.completed(), true);
});