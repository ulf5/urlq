# urlq

A command line tool for percent decoding or encoding strings as different parts of an url.

## Examples of use

One example can be to append strings to an url and call the urls with curl:
```
cat a_list_of_strings.txt | urlq | awk '{print "http://example.com/search?q="$0}' | xargs -n 1 curl -O

```

Sometimes you might have "urls" that work when entered into a browser, but you can't curl them.
Maybe they contain spaces or other characters which the browser handles.

```
curl $(urlq -u "http://example.com/this is an example/")
```

It can also decode encoded strings:

```
urlq -d "my%20encoded%20string"
```

Handling of spaces as plus (and vice versa) is also available in the relevant parts of the url.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
