require 'ffi'

module Scrape
  extend FFI::Library
  ffi_lib './target/debug/libscrape.dylib'

  attach_function :ruby_reverse, [:string], :string
  attach_function :concat, [:string, :string], :string
end

puts Scrape.ruby_reverse("Don't use palindrome") == "emordnilap esu t'noD" ? "Gone Dun It!" : "Uhhhh????"
puts Scrape.concat("This is", " a sentence.") == "This is a sentence." ? "Gone Dun It!" : "Uhhhh????"
