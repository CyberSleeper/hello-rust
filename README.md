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

Firstly we create a BufReader variable that reads from the TcpStream. Then we create a Vec with placeholder type (refer [this](https://stackoverflow.com/questions/34363984/what-is-vec)). We then use several methods to process the BufReader as follows.
- `lines()` method to get an iterator over the lines of the BufReader.
- `map(|result| result.unwrap())` method to unwrap the result of the iterator.
- `take_while(|line| !line.is_empty())` method to take the lines while not empty line.
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

The difference in this commit is that we add a response to the client. We created a status line, contents, and its length. Then we format the response with the status line, length, and contents. Finally, we write the response to the stream. We then format the response using the `format!` macro. The response is then written to the stream using the `write_all` method. According to our formatting, the `status_line` and `Content-Length` will be the response header, and the `contents` will be the response body. Below is the screenshot of the response in the browser:
![commit2](/public/commit2.png)

### Commit 3 Reflection notes

We can split the response by adding a conditional statement to check the request. If the request is `GET /`, we will return the `hello.html` page. Otherwise, we will return the `404.html` page. Below is the code snippet:

```rust
    let status_line;
    let contents;

    if http_request[0].starts_with("GET / ") {
        status_line = "HTTP/1.1 200 OK";
        contents = fs::read_to_string("pages/hello.html").unwrap();
    } else {
        status_line = "HTTP/1.1 404 NOT FOUND";
        contents = fs::read_to_string("pages/404.html").unwrap();
    }
```

However, for the sake of maintainability, we can refactor the code by creating a function to handle the response. Below is the code snippet:

```rust
fn handle_response(request: &String) -> (&str, String) {
    if request.starts_with("GET / ") {
        return ("HTTP/1.1 200 OK", fs::read_to_string("pages/hello.html").unwrap());
    }
    return ("HTTP/1.1 404 NOT FOUND", fs::read_to_string("pages/404.html").unwrap());
}
```

We then call the function in the `handle_connection` function as follows:

```rust
    let (status_line, contents) = handle_response(&http_request[0]);
```

Below is the screenshot of the response in the browser:

![commit3](/public/commit3.png)

### Commit 4 Reflection notes

In this commit, we added a sleep endpoint to the server.

```rust
"GET /sleep HTTP/1.1" => {
    thread::sleep(Duration::from_secs(10)); ("HTTP/1.1 200 OK", "pages/hello.html") 
}
```

But in this commit, the server will not be able to handle multiple requests concurrently. This is because the server is single-threaded. In this case, the server will sleep for 10 seconds before responding to the client. If another client sends a request, the server will not be able to handle it until the first request is completed. To handle multiple requests concurrently, we can use the `thread::spawn` method to spawn a new thread for each request. We will talk about this in the next commit.

### Commit 5 Reflection notes

In this commit, we implement ThreadPool to handle each request concurrently. We create a struct `ThreadPool` with a vector of threads. We then implement the `new` method to create a new ThreadPool. We also implement the `execute` method to execute the closure in a thread. Below is the code snippet to create a new ThreadPool:

```rust
for stream in listener.incoming() {
    let stream = stream.unwrap();

    pool.execute(|| {
        handle_connection(stream);
    });
}
```

By implementing ThreadPool, the server can handle multiple requests concurrently. In this implementation, we also limit the number of threads to 4. This is because creating too many threads can cause the server to slow down.

### Commit Bonus Reflection notes

In this commit, we change the implementation of making new ThreadPool from `new` method to `build` method. We are not using assert but condition to check if the number of threads is greater than 0. This is better because assert might lead to unrecoverable error. By using condition instead, the error is still recoverable and more informative. Note that when calling the `build` method, we need to unwrap it first. In the end, we can see that the server can handle multiple requests concurrently just like our former implementation.