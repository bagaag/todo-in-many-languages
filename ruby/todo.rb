#!/usr/bin/ruby
#
require './items.rb'

class ToDoCli

  def initialize (argv) 
    @ARGV = argv
  end

  # process command line execution
  def run 
    open_list
    process_command
  end

  # opens list from file storage
  def open_list 
    # items are stored in the ~/.todo file
    @file = File.expand_path('~') + '/.todo'

    # read saved list data
    if File.exist?(@file)
      json = IO.read(@file)
      @items = Items.new(json)
    else
      @items = Items.new()
    end
  end

  # parse arguments and act accordingly
  def process_command
    cmd = @ARGV[0]

    # the first arg, if any, is always the command, which defaults to "list"
    if cmd == nil or 'list'.start_with? cmd 
      self.print_list

    elsif 'add'.start_with? cmd 
      self.add

    elsif 'edit'.start_with? cmd 
      self.edit

    elsif 'complete'.start_with? cmd 
      self.complete

    elsif 'history'.start_with? cmd 
      self.history

    elsif 'recycle'.start_with? cmd 
      self.recycle

    elsif cmd == '?' or cmd == 'help' or cmd == '/?' or cmd == '--help'
      self.help

    else
      puts "'#{cmd}' is not a valid command. Try 'help'."
    end
  end

  # prints help
  def help
    puts ""
    puts "Usage: todo [command] [args]"    
    puts ""
    puts "Commands can be abbreviated to their first character."
    puts ""
    puts "Commands:"
    puts "  list"
    puts "    - Default command, displays to do list."
    puts "  add [item text]"
    puts "    - Adds an item to the list."
    puts "  edit [number] [item text]"
    puts "    - Edit/replace the numbered list item."
    puts "  complete [number]"
    puts "    - Mark tne numbered item completed."
    puts "  history"
    puts "    - Print completed items."
    puts "  recycle"
    puts "    - Delete completed items."
    puts "  help"
    puts "    - Prints this message."
    puts ""
  end

  # adds a list item and prints updated list
  def add
    desc = @ARGV[1..].join(' ')
    @items.add(desc)
    self.save_list
    self.print_list
  end

  # gets and validates the item index provided in a cmd line argument
  def get_index(arg_ix)
    ix = @ARGV[arg_ix]
    if ix and is_int?(ix) and @items.at(ix.to_i)
      return ix.to_i
    else 
      puts "Invalid item index: #{ix}"
      return 0
    end
  end

  # replaces an item with text provided
  def edit
    ix = self.get_index(1)
    if ix > 0
      desc = @ARGV[2..].join(' ').chomp
      @items.edit(ix, desc)
    end
    self.save_list
    self.print_list
  end

  # marks an item completed and prints updated list
  def complete 
    ix = self.get_index(1)
    if ix > 0
      @items.complete(ix)
    end
    self.save_list
    self.print_list
  end

  # print the list
  def print_list
    @items.list.each do |item|
      puts "% 2d. %s" % [item.index, item.description] 
    end
  end

  # persist the list
  def save_list
    File.write(@file, @items.to_json)
  end

  # prints completed items
  def history
    @items.completed.each do |item|
      d = DateTime.parse(item.completed)
      puts d.strftime("%m/%d/%Y %I:%M %p") + " - " + item.description
    end
  end

  # deletes completed items
  def recycle
    puts "Cleared #{@items.clear} completed item(s)."
    self.save_list
  end

  def is_int?(s)
    if s != nil and s.match(/[0-9]+/) then
      return true
    else 
      return false
    end
  end

end

# Calls the "run" method of this class if this file is invoked by the ruby
# interpreter directly
#
if __FILE__ == $0
  x = ToDoCli.new(ARGV)
  x.run 
end

