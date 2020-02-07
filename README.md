# JSON Gatherer
Gathers up JSON files in a directory, puts them all in a `HashMap` and spits 
out a serialized `serde_json::Value`, which you can then do whatever you want
with.

Still needs a couple of tweaks but it's almost done.

TODO:
- Only read .json files
