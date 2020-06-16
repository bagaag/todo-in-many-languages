# Console To Do List in different programming languages

Hi there! If you landed here looking for a command line driven task manager, this probably isn't for you. Check out [Task Warrior](https://taskwarrior.org/docs/start.html) for a serious implementation of this idea. If you're interested in the differences between various programming languages, read on.

This page started with the idea of writing a simple program in lots of different languages. This is not a new idea; some Googling reveals many such projects. But much of the value in such a project is to actually write the code in different languages and the learning that results. So to hell with all the other similar projects. There are many like it, but this one is mine.

Many such projects do something too simplistic, like "Hello world!" or [FizzBuzz](http://wiki.c2.com/?FizzBuzzTest). I wanted something that might actually be useful, something complex enough that it will test important features of the language, but not something so complex that would require rewriting tons of code for each version. 

I've landed on a console to-do list manager. It will support the following interactions:

```
   $ todo add This is a new list item
    1. This is a new list item
   $ todo a This is another list item
    1. This is a new list item
    2. This is another list item
   $ todo
    1. This is a new list item
    2. This is another list item
   $ todo edit 1 Edited first item
    1. Edited first item
    2. This is another list item
   $ todo complete 1
    1. This is another list item
   $ todo history
    04/20/2020 04:20 pm: Edited first item
   $ todo recycle
    1 completed item(s) recycled
```

As you can see, it is a console command that keeps a to-do list. It supports adding, listing, editing, and completing tasks, as well as listing and recycling completed tasks. Basic CRUD with persistence. Items will be stored in a text file. It's a fairly trivial program, but one that I suspect will use a pretty good range of language features without getting too lofty. I'll start with Ruby and see how that goes.

I have an idea to write a chronological journaling command line tool - a sort of memory map database for the command line - and will choose the language to write that in based on which seems most at home in this project. They're somewhat similar, but the journal project will be quite a bit more complex.

