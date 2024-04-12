# lfmt

`lfmt` is a very simple tool that can be used to extract values from strings in logfmt format. It is written in Rust and is available as a standalone binary.

## Usage

### Input from stdin

```sh
$ echo 'key1=value1 key2=value2 key3=value3' | lfmt get key1
value1
```

### Input from file 

```sh
$ echo 'key1=value1 key2=value2 key3=value3' > file.log 
$ lfmt get -f file.log key2
value2
```

### Input as argument 

```sh
$ lfmt get key3 'key1=value1 key2=value2 key3=value3'
value3
```
