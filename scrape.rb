require 'ffi'

module Scrape
  extend FFI::Library
  ffi_lib './target/debug/libscrape.dylib'

  attach_function :get_body, [:string], :string
end

body = Scrape.get_body("http://doc.rust-lang.org")[191..230]

puts body.strip == "<title>Rust Documentation</title>" ? "Gone Dun It!" : "Uhhhh????"
