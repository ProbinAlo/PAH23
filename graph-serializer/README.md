# Graph Serializer-Deserializer

This package contains utilities to serialize and 
deserialize given graph. This follows the specs defined
[here](../README.md#representing-graphs).

## Serializing

The `serialize` function takes `n_array` and `e_array` and serializes
them. The return value is a `Uint8Array`.
```js
serialize(["a", "b", "c", "d", "e"], 
          [[1n, 2n, 3n, 4n], [0n, 4n], [0n, 3n, 4n], 
          [0n, 2n], [0n, 1n, 2n]])
```

## Deserializing

The `deserialize` function takes a `Uint8Array` and returns
deserialized value. The returned value has two getters: `n_array`
and `e_array`.
```js
deserialize(serialized)
```
