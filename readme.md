Kuber Roles
==========

A program to read the 'roles' and 'cluster roles' from a kubernetes cluster via 'kubectl' and allow a user to see the roles certain subjects have access to via a restful HTTP API.

While the final project should be runnable in a container.  The challenge I was unable to resolve in the time I had was how to give a container in a cluster access to the roles in effect on that cluster.



Installation
-----------

This project is built in "Rust" and therefor requires "Rust" and it's package manager "Cargo" To compile.

It also requires a kubernetes cluster running and 'kubectl' available on the system.

Clone this repo and then call ```cargo install``` or ```cargo run```.

It should take a few minutes to download it's dependencies and run.

Usage
-----

Make sure a Kubernetes Cluster is running and kubectl works before running the program.  Once it is running it should serve to localhost:8086

To get the information needed, connect to localhost:8086 either using curl or in browser.

eg: 

```curl localhost:8086```

```curl localhost:8086?name=kube```

```curl localhost:8086?namex=\[a-m\]:&output=yaml```

The parameters you can send include
*    name: Filter Subjects whose name contain the given text substring
*    namex: Filter Subjects whose name matches the given regex
*    kind: Filter Subjects by whos type is in a Comma separeted list of Types "User,Group,ServiceAccount"
        Default is to include all.
*    output: The type of outpuy "yaml" or "json" default "json"
*    sort : "alpha" "length" (default no sort)







Dependencies
------------

Here is a list of Library dependencies and Why I have included them

* anyhow = "1.0.44"
    Makes it easier to pass Errors around a system.
* err_tools = "0.1.1"
    This is my own library, it makes it easier to build errors from static str's and combine them.

* serde = "1.0.130"
    Serde is an intermediary between various data formats and data structures. A structure that implements SERialize can be automagically converted to a large number of different data formats.

* serde_json = "1.0.67"
    Works with Serde and can read and write JSON
serde_yaml = "0.8.21"
    Works with Serde and can read and write Yaml
serde_derive = "1.0.130"
    Automatically Generates code to make rust structures work with Serde
actix-web = "3.3.2"
    A Capable HTTP Server framework, that runs fast and async.  I chose this one because it is easy to work with and runs on stable Rust.
regex = "1.5.4"
    Builds and tests regex patterns.







