package model

import "testing"

// ----------------
// removeItem Tests

func getArray() []Item {
  a := []Item{ Item{Description:"item 1"}, Item{Description:"item 2"}, Item{Description:"item 3"} }
  return a
}

func removeItemTest(t *testing.T, ix int, desc string) {
  a := getArray()
  a, i := removeItem(a, ix)
  if len(a) != 2 {
    t.Errorf("removed ix %d, want len(a)=2, got %d", ix, len(a))
  }
  if i.Description != desc {
    t.Errorf("want %s, got %s", desc, i.Description)
  }
}

func TestRemoveItem(t *testing.T) {
  // tests removing first, middle and last element
  removeItemTest(t, 0, "item 1")
  removeItemTest(t, 1, "item 2")
  removeItemTest(t, 2, "item 3")
}

func TestAdd(t *testing.T) {
  list := List{}
  list.Add("item 1")
  if len(list.items) != 1 {
    t.Errorf("want len = 1 after add, got %d", len(list.items))
  }
  item := list.items[0]
  if item.Description != "item 1" {
    t.Errorf("want 'item 1', got %s", item.Description)
  }
  if ! item.Completed.IsZero() {
    t.Errorf("want Zero, got %s", item.Completed)
  }
}

func TestComplete(t *testing.T) {
  list := List{}
  if len(list.items) != 0 || len(list.completed) != 0 {
    t.Errorf("want 0,0, got %d,%d", len(list.items), len(list.completed))
  }
  list.Add("item 1")
  if len(list.items) != 1 || len(list.completed) != 0 {
    t.Errorf("want 1,0, got %d,%d", len(list.items), len(list.completed))
  }
  list.Complete(0)
  if len(list.items) != 0 || len(list.completed) != 1 {
    t.Errorf("want 0,1, got %d,%d", len(list.items), len(list.completed))
  }
  time := list.completed[len(list.completed)-1].Completed
  if time.IsZero() {
    t.Errorf("want Zero time, got %s", time)
  }
}
