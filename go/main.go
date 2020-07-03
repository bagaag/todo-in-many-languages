package main

import (
  "errors"
  "fmt"
  "os"
  "strconv"
  "strings"
  "todo/model"
)

const SaveTo = ".todo"

func main() {
  // initialize list and load from storage
  items := model.List{}
  if _, err := os.Stat(SaveTo); !os.IsNotExist(err) {
    items.Load(SaveTo)
  }
  // get command from args and shift cmd out of args
  args := os.Args[1:]
  cmd := "list"
  if len(args) > 0 {
    cmd = args[0]
    args = args[1:]
  }
  switch cmd {
  case "list":
    display(&items)
  case "add":
    add(&items, args)
  case "edit":
    edit(&items, args)
  case "complete":
    complete(&items, args)
  case "completed":
    completed(&items)
  case "clear":
    clear(&items)
  default:
    help()
  }
}

func add(list *model.List, args []string) {
  desc := strings.Join(args, " ")
  list.Add(desc)
  list.Save(SaveTo)
  display(list)
}

func display(list *model.List) {
  for ix, item := range list.Items {
    fmt.Printf("%2d. %s\n", ix+1, item.Description)
  }
}

func completed(list *model.List) {
  for _, item := range list.Completed {
    // go has an odd but friendly way to indicate d/t formats
    when := item.Completed.Format("2006-01-02 03:04 PM")
    fmt.Printf("%s - %s\n", when, item.Description)
  }
}

// shifts index arg off args slice
func shiftIndex(args[] string) (ix int, err error, shiftedArgs []string) {
  if len(args) < 1 {
    return 0, errors.New("An item number is required."), args
  }
  i, err := strconv.ParseInt(args[0], 10, 0)
  ix = int(i) // ParseInt returns int64
  if err != nil {
    return 0, errors.New("Argument must be a number."), args
  }
  shiftedArgs = args[1:]
  return
}

func complete(list *model.List, args []string) {
  ix, err, _ := shiftIndex(args)
  if err != nil {
    fmt.Println(err)
    return
  }
  list.Complete(int(ix - 1))
  list.Save(SaveTo)
  display(list)
}

func edit(list *model.List, args[]string) {
  ix, err, descArgs := shiftIndex(args)
  if err != nil {
    fmt.Println(err)
    return
  }
  desc := strings.Join(descArgs, " ")
  ix = ix - 1
  list.Edit(ix, desc)
  list.Save(SaveTo)
  display(list)
}

func clear(list *model.List) {
  count := list.Clear()
  list.Save(SaveTo)
  fmt.Printf("%d items cleared.\n", count)
}

func help() {
  fmt.Println(`Usage: todo [cmd] [args]
Commands:
  list               - Lists current items
  add <text>         - Adds a new item
  edit <num> <text>  - Replaces an existing item
  complete <num>     - Completes an item
  completed          - Lists completed items
  clear              - Removes completed items`)
}
