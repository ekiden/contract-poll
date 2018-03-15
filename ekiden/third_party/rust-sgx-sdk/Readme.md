# Rust SGX SDK
Rust SGX SDK helps developers write Intel SGX applications in Rust programming language. [[Paper pdf]](documents/ccsp17.pdf)

## v0.9.7 Release
This version provides a new namespace: `sgx_tstd::untrusted`, including `sgx_tstd::untrusted::fs` `sgx_tstd::untrusted::time` and `sgx_tstd::untrusted::path`, providing supports to operation to ocalls in a **untrusted** namespace. The **untrusted** namespace is always enabled no matter `untrusted_*` is set or not. We **urge** the developers to use the `sgx_tstd::untrusted` namespace to port their crates, instead of enabling the `untrusted_` series of features. Also, we renamed the `untrusted_net` feature to `net` for feature name unification. Please refer to [release_notes](release_notes.md) for further details.

## v0.9.6 Release
This version provides security enhancement for untrusted IO and additional support for monotonic counter. Untrusted IO operations in `sgx_tstd::fs` `sgx_tstd::net` and `sgx_tstd::time` are **DISABLED by default** to reduce the untrusted surface, and can be enabled by features. Trusted time support is moved to `sgx_tservice::sgxtime` and monotonic counter is provided by `sgx_tservice::sgxcounter`. Please refer to [release_notes](release_notes.md) for further details.

## v0.9.5 Release
This is a **milestone version**, and may be the last version before 1.0.0. It provides supports of network access, TLS connection, trusted/untrusted file system access, trusted/untrusted time, and environment variable operations. Most important, it supports `xargo`! Now `x86_64-unknown-linux-sgx` is the new platform target. All of the code samples and third-party libraries could be built by `xargo` via `XARGO_SGX=1 make` (cargo also supported by `make`). What's more, we provide a pair of TLS client/server [sample](samplecode/tls) as a complete stack of secure! Please refer to [release_notes](release_notes.md) for further details.

## Run Rust SGX applications in Mesalock Linux
[MesaLock Linux](https://github.com/mesalock-linux/mesalock-distro) is a general purpose Linux distribution which aims to provide a safe and secure user space environment. Now we can run Rust SGX applications in Mesalock Linux within a few steps. Please refer to the [tutorial](documents/sgx_in_mesalock_linux.md) for details.

## v0.9.1 Release
This version supports the recently released [Intel SGX SDK 2.0](https://download.01.org/intel-sgx/linux-2.0/), and provides the most popular machine learning library [rusty-machine](https://github.com/AtheMathmo/rusty-machine/) in Intel SGX! Please refer to [release_notes](https://github.com/baidu/rust-sgx-sdk/blob/master/release_notes.md) for further details.


## v0.9.0 Release
Almost there! Rust SGX SDK v0.9.0 is coming up as a beta version of the future v1.0.0, with the most desired `sgx::tstd` as well as many new features! Also we added support for programming SGX untrusted part in Rust using `sgx::urts`. Now it's easy to port Rust crates to the SGX trust execution environment and write the whole SGX application in Rust! Please refer to [release_notes](https://github.com/baidu/rust-sgx-sdk/blob/master/release_notes.md) for further details.

Good news! Our poster 'Rust SGX SDK: Towards Memory Safety in Intel SGX Enclave' [[pdf]](https://github.com/baidu/rust-sgx-sdk/blob/master/documents/ccsp17.pdf) has been accepted by CCS'17. Please kindly cite our poster if you like Rust SGX SDK!

## v0.2.0 Release
We are proud to have our v0.2.0 Rust SGX SDK released. It is now providing more threading functions, thread local storages, exception handling routines and supports unwind mechanism, as well as support of **LARGE ENCLAVE MEMORY** with the help of Intel SGX v1.9 (**31.75 GB enclave memory tested**). Please refer to [release notes](https://github.com/baidu/rust-sgx-sdk/blob/master/release_notes.md) for further details. And we are working on a white paper for technical details about this project.

**Attention** Rust has recently merged a patch [(rustc: Implement the #[global_allocator] attribute)](https://github.com/rust-lang/rust/commit/695dee063bcd40f154bb27b7beafcb3d4dd775ac#diff-28f2fd684ad47d385427678d96d2dcd4) which significantly changes the behavior of liballoc. Thus `set_oom_handler` is no longer available since nightly-2017-07-07. Due to its instability, v0.2.0 Rust SGX SDK keeps using the old liballoc instead of new liballoc_system. As a result, nightly version rustc after 2017-07-06 will not successfully compile `sgx_trts`.

## Requirement

Ubuntu 16.04

[Intel SGX SDK 2.0 for Linux](https://01.org/intel-software-guard-extensions/downloads) installed

Docker (Recommended)


## Configuration

The docker image now supports Intel ME. If you need it, please refer to the sgxtime [readme](documents/sgxtime.md) for instructions.

### Using docker (Recommended) without ME support
First, make sure Intel SGX Driver 2.0 is installed and functions well. `/dev/isgx` should be appeared.

Second, pull the docker image

`$ docker pull baiduxlab/sgx-rust`

Third, start a docker with sgx device support and the Rust SGX SDK.

`$ docker run -v /your/path/to/rust-sgx:/root/sgx -ti --device /dev/isgx baiduxlab/sgx-rust`

Next, start the aesm service inside the docker

`root@docker:/# /opt/intel/sgxpsw/aesm/aesm_service &`

Finally, check if the sample code works

`root@docker:~/sgx/samplecode/helloworld# make`

`root@docker:~/sgx/samplecode/helloworld# cd bin`

`root@docker:~/sgx/samplecode/helloworld/bin# ./app`

### Native without docker (Not recommended)

Install Intel SGX driver and SDK first. And refer to Dockerfile for detail.

## Build the docker image by yourself

Make sure Intel SGX SDK is properly installed and service started on the host
OS. Then `cd dockerfile` and run `docker build -t rust-sgx-docker` to build.

# Documents

The online documents for SDK crates can be found
[here](https://dingelish.github.io/).

We recommend to use `cargo doc` to generate documents for each crate in this
SDK by yourself.  The auto generated documents are easy to read and search.

# Sample Codes

We provide five sample codes to help developers understand how to write Enclave
codes in Rust. These codes are located at `samplecode` directory.

* `helloworld` is a very simple app. It shows some basic usages of argument
passing, Rust string and ECALL/OCALLs.

* `crypto` shows the usage of crypto APIs provided by Intel SGX libraries. It
does some crypto calculations inside the enclave, which is recommended in most
circumstances.

* `localattestation` is a sample ported from the original Intel SGX SDK. It
shows how to do local attestation in Rust programming language.

* `sealeddata` sample shows how to seal secret data in an enclave and how to
verify the sealed data.

* `thread` sample is a sample ported from the original Intel SGX SDK, showing
some basic usages of threading APIs.

* `remoteattestation` sample shows how to make remote attestation with
Rust SGX SDK. The sample is forked from [linux-sgx-attestation](https://github.com/svartkanin/linux-sgx-remoteattestation)
and credits to Blackrabbit (blackrabbit256@gmail.com). The enclave in Rust
is shipped in this sample and Makefiles are modified accordingly.

* `hugemem` sample shows how to use huge mem in SGX enclave. In this
sample, we allocate reserve 31.75GB heap space and allocate 31.625GB buffers!

* `file` sample shows how to read/write files in SGX enclave.

* `hello-rust` is the helloworld sample writtin in pure Rust.

* `backtrace` is a sample showing how to enabling backtrace mechanism inside the enclave.

* `unit-test` shows the way of writing unit tests and conduct unit testing.

* `zlib-lazy-static-sample` shows how to use ported third party crates inside enclave.

* `machine-learning` shows how to use [rusty-machine](https://github.com/AtheMathmo/rusty-machine) for machine learning inside Intel SGX enclave.
* New! `tls` contains a pair of TLS client/server runs perfectly in SGX enclave!
* New! `sgxtime` shows how to acquire trusted timestamp via Intel ME. Please refer to this [instruction](documents/sgxtime.md) for detail.

# Samples of ported third-party libraries

As of v0.9.5, we provide 25 ported third-party libraries. All of them could be compiled using xargo (`XARGO_SGX=1` make) or cargo (`make`).

# Tips for writing enclaves in Rust

## Writing EDL

* For fixed-length array in ECALL/OCALL definition, declare it as an array.  For
dynamic-length array, use the keyword `size=` to let the Intel SGX knows how
many bytes should be copied.

## ECALL Function Naming

* Add `#[no_mangle]` for every ECALL function.

## Passing/returning arrays

* For dynamic-length array, the only way is to use raw pointers in Rust. There
are several functions to get/set data using raw pointers such as
[`offset`](https://doc.rust-lang.org/1.9.0/std/primitive.pointer.html#method.offset)
method. One can also use
[`slice::from_raw_parts`](https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html)
to convert the array to a slice.

* For Fixed-length array, the above method is acceptable. And according to
discussions in [issue 30382](https://github.com/rust-lang/rust/issues/30382)
and [issue 31227](https://github.com/rust-lang/rust/issues/31227),
thin-pointers (such as fixed-length array) are FFI-safe for now, but
undocumented. In the sample codes, we use fixed-length arrays for passing and
returning some fixed-length data.

# License

Baidu Rust-SGX SDK is provided under the BSD license. Please refer to the [License file](LICENSE)
for details.

# Authors

Ran Duan, Long Li, Shi Jia, Yu Ding, Lenx Wei, Tanghui Chen

![Baidu X-Lab Logo](https://raw.githubusercontent.com/baidu/rust-sgx-sdk/master/logo_25.png)

# Acknowledgement

Thanks to [Prof. Jingqiang Lin](http://people.ucas.ac.cn/~0010268) for his contribution to this project.

# Contacts

Yu Ding, dingelish@gmail.com

