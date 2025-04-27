# About Example

This is how code could look with Ronky, as you can see there is nothing really a
lot that you have to add to make it work. Only the `Exported` derive should be
present. And if you want to use generics they should also be exportable (which
is implicitly done by the `Exportable` derive).

The output of this command would be the JSON located in the `out.json` file.

## Running the example

> Note: this will make/overwrite a file in your current directory called `out.json`

You can run the example by running the following command:

```bash
cargo run -p about
```
