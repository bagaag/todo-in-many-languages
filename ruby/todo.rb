require './item.rb'

# settings
file = File.expand_path('~') + '/.todo'

# read saved list
if File.exist?(file)
  list = File.readlines(file)
else
  list = []
end

# see if the first argument is a valid list index
ix = ARGV[0]
if ix and Float(ix) and list[ix.to_i]
  
  # if remaining args form a string, then edit, otherwise, delete
  item = ARGV[1..].join(' ')
  if item != ''
    list[ix] = item
  else
    list.delete_at(ix)
  end

# if args contain an item to add to the list
elsif ARGV.length > 0
  item = ARGV[0..].join(' ')
  list.push(item)
end

# print the list
list.each_index do |ix|
  puts "% 2d. %s" % [ix+1, list[ix]] 
end

# persist the list
File.write(file, list.join("\n"))
