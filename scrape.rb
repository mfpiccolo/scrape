require 'ffi'

module Scrape
  extend FFI::Library
  ffi_lib './target/debug/libscrape.dylib'

  class TwoNumbers < FFI::Struct
    layout :first, :uint32,
           :second, :uint32
  end

  class FromRustArray < FFI::Struct
    layout :len,    :size_t, # dynamic array layout
           :data,   :pointer #

    def to_a
      self[:data].get_array_of_string(0, self[:len]).compact
    end
  end

  attach_function :add_struct_vals, [TwoNumbers.by_value], :int32
  attach_function :add_one_to_vals, [TwoNumbers.by_value], TwoNumbers.by_value
  attach_function :number_to_char_array, [], FromRustArray.by_value
  attach_function :print_chars, [], :void
end

tn = Scrape::TwoNumbers.new
tn[:first] = 10
tn[:second] = 20

puts Scrape.add_struct_vals(tn)          == 30 ? "Wooooo!" : "Booooooo!"
puts Scrape.add_one_to_vals(tn)[:first]  == 11 ? "Wooooo!" : "Booooooo!"
puts Scrape.add_one_to_vals(tn)[:second] == 21 ? "Wooooo!" : "Booooooo!"
p Scrape.number_to_char_array.to_a
Scrape.print_chars
