package model

import (
  "encoding/json"
  "io/ioutil"
  "time"
)

// Holds an item in the list
type Item struct {
  Description string
  Completed time.Time // defaults to Time.Zero
}

// Holds the to do list and provides CRUD methods
type List struct {
  Items []Item
  Completed []Item
}

// go does not provide this very basic function for some reason
func removeItem(a []Item, ix int) (out []Item, item Item) {
  item = a[ix]
  copy(a[ix:], a[ix+1:])
  out = a[:len(a)-1]
  return
}

// returns byte array json representation of list model
func (list *List) ToJson() []byte {
  ba, err := json.Marshal(list)
  if err != nil {
    panic(err)
  }
  return ba
}

// populates model from byte array json data
func (list *List) FromJson(ba []byte) error {
  return json.Unmarshal(ba, &list)
}

// writes json file with model contents
func (list *List) Save(filename string) error {
  ba := list.ToJson()
  return ioutil.WriteFile(filename, ba, 0600)
}

// populates model from json file
func (list *List) Load(filename string) error {
  ba, err := ioutil.ReadFile(filename)
  if err != nil {
    return err
  }
  err = list.FromJson(ba)
  return err
}

// adds an item to the list
func (list *List) Add(desc string) {
  newItem := Item{Description: desc}
  list.Items = append(list.Items, newItem)
}

// updates the description of an item in the list
func (list *List) Edit(ix int, desc string) {
  item := &list.Items[ix]
  item.Description = desc
}

// completes an item in the list
func (list *List) Complete(ix int) {
  items, item := removeItem(list.Items, ix)
  list.Items = items
  item.Completed = time.Now()
  list.Completed = append(list.Completed, item)
}

// deletes completed items and returns count cleared
func (list *List) Clear() int {
  count := len(list.Completed)
  list.Completed = []Item{}
  return count
}


