# Wiki2

Wiki2 is a wiki designed with KISS principals, written in Rust. It is built with the intention of porting to use within the SAFE and/or IPFS networks.

## Contributing

I would like some help working on this if anyone is interested. It is a decent project for learning about rust and there are plenty of unimplemented features that I would like to push out. To contribute, just fork the repo, push commits to a new branch and submit a pull request.

## Installation

Quick and dirty
```
cargo install --git https://github.com/waynenilsen/wiki2.git
```


If you don't know what that means, this is for users of Ubuntu and probably OSX.

1. Install multirust `curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh`
2. Install the nightly toolchain `multirust override nightly`
3. Use Cargo nightly to install wiki2 `cargo install --git https://github.com/waynenilsen/wiki2.git`

## What is this?

It is a website generator that parses markdown content. It is very much in the mind of keep it simple stupid. This means that if you change the content of the files, the entire website updates automatically. Unlike static site generators, the generation is done on the fly (it's pretty cheap anyway and can always be cached). 

Eventually the intended purpose of this will be behind the first wikis stored on IPFS and SAFE networks. The framework is very friendly to distributed systems and figuring out hygenic APIs now will help save time when these networks are ready and/or have enough documentation to actually use them.

### See it in action

Run the commands

```
git clone --depth=1 https://github.com/waynenilsen/wiki2.git
cd wiki2
multirust override stable
sudo cargo run -- --help
sudo cargo run -- -d ./tests/test1/ -h 0.0.0.0:80
```

Then open your browser and type localhost into the address bar. You should be able to browse the links. 

## How to use this program

Point it to any directory on your file system. In this directory (and subdirectories) create text files with markdown content. 
Here is an example

```
.
├── hi.md
├── _index.md
└── sub-folder
    ├── _index.md
    └── inner-hi.md
```

this tree can be served to localhost by navigating to the directory and running the `wiki2` command in that directory.

## General Design Principals

* Content should be created in (some variant of) markdown.
* Content should not be stored in rendered form if possible.
* This wiki should feel 1.0 with the hope of having 0% javascript if at all possible
* Avoid creating a user system if at all possible, in stead, choose to use someone elses system
* To the extent that there are permissions, they should be based on UNIX
* All content should be versioned
* Use templates for different sections of the site
* Provide usable default templates
* Provide usable default CSS











