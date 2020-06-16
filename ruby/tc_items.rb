require_relative 'items'
require 'test/unit'

# I don't normally write test cases, but Ruby devs
# are nuts about their test coverage, so I figured
# I'd give it a shot. 66 lines of code to test 100
# lines of code feels like a lot, but I ended spend-
# ing more time troubleshooting the bugs these tests
# found than I did writing the fairly trival tests.
# Had I waited to find these bugs testing real code,
# the bugs would have been a lot harder to pin down
# since I wouldn't have such a simple context to
# work within. 

class TestItems < Test::Unit::TestCase
  def setup
    @items = Items.new()
  end

  def teardown
    # nope
  end

  def test_items_new
    assert @items.all.length == 0
  end

  def test_items_at
    assert @items.at(1) == nil
    @items.add('item 1')
    assert @items.at(1) != nil
  end

  def test_items_add
    @items.add('item 1')
    assert @items.at(1).description == 'item 1'
    assert @items.list.length == 1
  end

  def test_items_edit
    @items.add('item 1')
    @items.edit(1, 'edited item')
    assert @items.at(1).description == 'edited item'
  end

  def test_items_complete
    @items.add('item 1')
    @items.complete(1)
    assert @items.at(1) == nil
    assert @items.completed.length == 1
    assert @items.completed[0].completed != nil
  end

  def test_items_clear
    @items.add('item 1')
    @items.complete(1)
    @items.clear
    assert @items.list.length == 0
    assert @items.completed.length == 0
  end

  def test_items_json
    @items.add('item 1')
    @items.add('item 2')
    @items.complete(1)
    json = @items.to_json
    puts json
    other = Items.new(json)
    assert other.at(1).description == 'item 2'
    assert other.completed[0].description == 'item 1'
  end
end

