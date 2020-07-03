package model

import (
  "testing"
  "os"
)

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
  if len(list.Items) != 1 {
    t.Errorf("want len = 1 after add, got %d", len(list.Items))
  }
  item := list.Items[0]
  if item.Description != "item 1" {
    t.Errorf("want 'item 1', got %s", item.Description)
  }
  if ! item.Completed.IsZero() {
    t.Errorf("want Zero, got %s", item.Completed)
  }
}

func TestComplete(t *testing.T) {
  list := List{}
  if len(list.Items) != 0 || len(list.Completed) != 0 {
    t.Errorf("want 0,0, got %d,%d", len(list.Items), len(list.Completed))
  }
  list.Add("item 1")
  if len(list.Items) != 1 || len(list.Completed) != 0 {
    t.Errorf("want 1,0, got %d,%d", len(list.Items), len(list.Completed))
  }
  list.Complete(0)
  if len(list.Items) != 0 || len(list.Completed) != 1 {
    t.Errorf("want 0,1, got %d,%d", len(list.Items), len(list.Completed))
  }
  time := list.Completed[len(list.Completed)-1].Completed
  if time.IsZero() {
    t.Errorf("want Zero time, got %s", time)
  }
}

func TestToJson(t *testing.T) {
  list := List{}
  list.Add("item")
  expect := `{"Items":[{"Description":"item","Completed":"0001-01-01T00:00:00Z"}],"Completed":null}`
  got := string(list.ToJson())
  if expect != got {
    t.Errorf("expected: %s\ngot: %s", expect, got)
  }
}

func TestFromJson(t *testing.T) {
  list := List{}
  ba := []byte(`{"Items":[{"Description":"item","Completed":"0001-01-01T00:00:00Z"}],"Completed":null}`)
  err := list.FromJson(ba)
  if err != nil {
    t.Errorf("%s", err)
  }
  desc := list.Items[0].Description
  if desc != "item" {
    t.Errorf("expected 'item', got: %s", desc)
  }
}

func TestPersistence(t *testing.T) {
  path := "test_data"
  listA := List{}
  listA.Add("item")
  listA.Save(path)
  listB := List{}
  listB.Load(path)
  desc := listB.Items[0].Description
  if desc != "item" {
    t.Errorf("expected 'item', got: %s", desc)
  }
  if err := os.Remove(path); err != nil {
    t.Errorf("could not delete test file %s", err)
  }
}
