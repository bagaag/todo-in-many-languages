require 'json'
require 'date'

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

class Items
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

  def to_json
    JSON.generate( self.to_h )
  end

  def reindex
    index = 1
    @all.each {|item|
      if not item.completed
        item.index = index
        index = index + 1
      end
    }
  end

  def add( description )
    @all.push( Item.new( description ) )
    reindex
  end

  def edit( index, description )
    item = at(index)
    if item
      item.description = description
    else
      raise IndexError
    end
  end 

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

  def clear
    @all.delete_if {|item| item.completed }
  end

  def all
    @all
  end

  def at (ix) 
    items = @all.filter { |item| item.index == ix }
    items[0]
  end

  def completed
    @all.filter { |item| item.completed }
  end

  def list
    @all.filter { |item| not item.completed }
  end
end

