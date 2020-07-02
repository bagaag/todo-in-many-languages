package model

import (
  //"fmt"
  "time"
)

type Item struct {
  Description string
  Completed time.Time // defaults to Time.Zero
}

type List struct {
  items []Item
  completed []Item
}

// go does not provide this very basic function for some reason
func removeItem(a []Item, ix int) (out []Item, item Item) {
  item = a[ix]
  copy(a[ix:], a[ix+1:])
  out = a[:len(a)-1]
  return
}

//func (list *List) Load(filename string) {
//}

func (list *List) Add(desc string) {
  // create item
  newItem := Item{Description: desc}
  list.items = append(list.items, newItem)
}

func (list *List) Complete(ix int) {
  items, item := removeItem(list.items, ix)
  list.items = items
  item.Completed = time.Now()
  list.completed = append(list.completed, item)
}




