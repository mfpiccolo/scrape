require 'ffi'

module Scrape
  extend FFI::Library
  ffi_lib './target/debug/libscrape.dylib'

  class TwoNumbers < FFI::Struct
    layout :first, :int32,
           :second, :int32
  end

  attach_function :add_struct_vals, [TwoNumbers.by_value], :int32
end

tn = Scrape::TwoNumbers.new
tn[:first] = 10
tn[:second] = 20

puts Scrape.add_struct_vals(tn) == 30 ? "Gone Dun It!" : "Uhhhh????"
