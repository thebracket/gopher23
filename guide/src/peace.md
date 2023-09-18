# I Come in Peace!

I'm not here to start a language fight! Go is a great language, and I'm absolutely not here as an evangelist. I find the constant replies of "rewrite it in Rust!" irritating, too!

(graphic)

GopherCon is an odd place to be talking about Rust! 

The background is that Ardan Labs brought me on board a few months ago, following a surge in interest in providing Rust content. Miguel & Bill asked me to give an introductory workshop at Gopher Con (as well as more advanced content at RustConf and Rust Nations UK - more natural homes). I'm hoping to satisfy some of the interest. Here's my reasoning for agreeing to give the workshop:

* Open source projects really should stick together, not fight. We can all learn from one another!
* Go is fantastic for a lot of projects - particularly highly concurrent (as in green thread/coroutine/async-await oriented) services. Rust is quite amazing when it comes to highly parallel (CPU intensive, system thread, close-to-the-metal) computation, or for highly critical systems that absolutely need Rust's safety guarantees for data races and memory safety. (Rust is also good at coroutines, but doesn't have as established an ecosystem for the types of app in which Go shines).
* There's really no reason to go either/or. In a service-oriented architecture, Rust & Go talk to one another just fine. Even in a single binary, you can benefit from both through FFI/CGo (with some caveats of needing to make sure you're doing enough work to justify the - increasingly small - CGo performance hit at the boundary). Most of the bigger projects I've helped with have that "one system that just won't go fast enough" - and that's where Rust can help the most.
* Rust's reputation for a vertical learning curve isn't really warranted, but it's a different mindset. It's good for everyone to learn more than one language!

> So let's keep it positive and enjoy learning some Rust!