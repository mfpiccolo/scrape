require 'ffi'

module Scrape
  extend FFI::Library
  ffi_lib './target/debug/libscrape.dylib'

  attach_function :run_threads, [], :void
end

Scrape.run_threads()
