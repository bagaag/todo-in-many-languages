require 'json'
require 'date'

# represents an item in the to do list 
class Item
  attr_accessor :description
  attr_accessor :completed
  attr_accessor :index
  def initialize( description = '', completed = nil )
    @description = description
    @completed = completed
  end
  def to_s
    "[ description: #{@description}, index: #{@index}, completed: #{@completed} ]"
  end
end


# represents the to do list and provides crud methods
class Items

  # transforms json data into an in-memory to do list
  def initialize( json = '{"items":[]}' )
    @all = []
    json_obj = JSON.parse( json )
    json_list = json_obj['items']
    json_list.each do |json_item|
      # Note: JSON doesn't use the ruby convention of :key for hash keys
      item = Item.new( json_item['description'], json_item['completed'] )
      @all.push( item )
    end
    reindex
  end

  # returns a hashmap representation of the list
  def to_h
    ret = { :items => [] }
    @all.each do |item|
      ret[:items].push({
        :description => item.description,
        :completed => item.completed
      })
    end
    return ret
  end

  # returns a json representation of the list
  def to_json
    JSON.generate( self.to_h )
  end

  # updates the index - should be called when list items change
  def reindex
    index = 1
    @all.each {|item|
      if not item.completed
        item.index = index
        index = index + 1
      end
    }
  end

  # adds an item to the list
  def add( description )
    @all.push( Item.new( description ) )
    reindex
  end

  # edits an existing item in the list at the specified 1-based index
  def edit( index, description )
    item = at(index)
    if item
      item.description = description
    else
      raise IndexError
    end
  end 

  # completes the list item specified by the 1-based index
  def complete( index )
    item = at(index)
    if item
      # Ruby does not handle date round-tripping in JSON well :(
      # https://stackoverflow.com/questions/13594289/datetime-serialization-and-deserialization
      item.completed = DateTime.now.to_s
      item.index = nil
    end
    reindex
  end

  # removes comleted items from the list
  def clear
    count = @all.length
    @all.delete_if {|item| item.completed} 
    return count - @all.length
  end

  # returns all list items
  def all
    @all
  end

  # returns list item at the specified 1-based index
  def at (ix) 
    items = @all.filter { |item| item.index == ix.to_i }
    items[0]
  end

  # returns list of completed items
  def completed
    @all.filter { |item| item.completed }.sort_by(&:completed)
  end

  # returns list of active list items
  def list
    @all.filter { |item| not item.completed }
  end
end

