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

### Commit 2 Reflection notes

```rust
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap();
}
```

The difference in this commit is that we add a response to the client. We created a status line, contents, and its length. Then we format the response with the status line, length, and contents. Finally, we write the response to the stream. According to our formatting, the `status_line` and `Content-Length` will be the response header, and the `contents` will be the response body. Below is the screenshot of the response in the browser:
![commit2](/commit2.png)