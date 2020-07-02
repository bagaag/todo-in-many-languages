package main

import (
  "fmt"
  "todo/model"
  //"time"
)

func main() {
  item := model.Item{Description: "Hey now."}
  fmt.Println(item)
  if item.Completed.IsZero() {
    fmt.Println("not completed")
  } else {
    fmt.Printf("Completed on {}", item.Completed)
  }
  items := model.List{}
  items.Add("new list item")
  fmt.Println(items)
  items.Complete(0)
  fmt.Println(items)
  fmt.Println("I am main.")
}
