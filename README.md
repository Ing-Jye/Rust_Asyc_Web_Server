This is a simulated web server created by Rust.

The goal is to use Tokio to deal with incoming requests asynchronously.

There are two threads - one is the main thread which only used to accept incoming requests, another is used to respond those requests.

If the URI of request is "/", then the server will respond "OK" immediately, otherwise it will respond "NOT FOUND" after sleeping for 20 seconds.

I only use ONE thread to respond, but the respond with "OK" will not to be waitted even though we sent a request that will cause a sleep first.

The reason is that I use asynchronous programming, so that when a request causes sleeping, my server leaves it along, and deals with other requests.
