require 'ffi'

module Scrape
  extend FFI::Library
  ffi_lib './target/debug/libscrape.dylib'

  attach_function :sum, [:int32, :int32], :int32
end

puts Scrape.sum(2, 3) == 5 ? "Gone Dun It!" : "Uhhhh????"
