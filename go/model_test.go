package model

func testArray() []Item {
  a := []Item{ Item{Description:"item 1"}, Item{Description:"item 2"}, Item{Description:"item 3"} }
  return a
}

func TestRemoveItem(t *testing.T) {
  a = testArray()
  (a i) = removeItem(a, 0)
  if len(a) != 2 {
    t.Errorf("removed first, want len(a)=2, got %s", len(a))
  }
  if i.Description != "item 1" {
    t.Errorf("want 'item 1', got %s", i.Description)
  }
}
