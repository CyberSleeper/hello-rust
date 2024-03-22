###  Commit 1 Reflection notes

```rust
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}
```

Firstly we create a BufReader variable that reads from the TcpStream. Then we create a Vec with placeholder type (refer [this](https://stackoverflow.com/questions/34363984/what-is-vec)). We then use several methods to process the BufReader as follows:
- `lines()` method to get an iterator over the lines of the BufReader;
- `map(|result| result.unwrap())` method to unwrap the result of the iterator;
- `take_while(|line| !line.is_empty())` method to take the lines while not empty line;
- `collect()` method to collect the lines into a vec.

The `println!` macro is used to print the http request.