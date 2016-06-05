THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
PERFORMANCE OF THIS SOFTWARE.



untrusted.rs
============

Safe, fast, zero-panic, zero-crashing, zero-allocation parsing of untrusted
inputs in Rust.

untrusted.rs is 100% Rust with no use of `unsafe`. It is `#![no_std]` and so
it works perfectly with both libcore- and libstd- based projects. It never uses
the heap. No part of untrusted.rs's API will ever panic or cause a crash.



Documentation
-------------

See the documentation at
https://briansmith.org/rustdoc/untrusted/.

To use untrusted.rs in your project, add a dependency to your
Cargo.toml like this:

```
[dependencies]
rust-untrusted = { git = "https://github.com/briansmith/untrusted" }
```



Examples
--------

[*ring*](https://github.com/briansmith/ring)'s parser for the subset of ASN.1
DER it needs to understand,
[`ring::der`](https://github.com/briansmith/ring/blob/master/src/der.rs), is
built on top of untrusted.rs. *ring* also uses untrusted.rs to parse ECC public
keys, RSA PKCS#1 1.5 padding, and everything else.

All of [webpki](https://github.com/briansmith/webpki)'s parsing of X.509
certificates (also ASN.1 DER) is done using untrusted.rs.



Contributing
------------

Patches welcome!

When contributing changes, state that you agree to license your contribution
under the same terms as the existing code by putting this at the bottom of your
commit message:

```

I agree to license my contributions to each file under the terms given
at the top of each file I changed.
```

Currently, the biggest needs for this library are:

* Unit tests.
* Documentation.
* More examples.
* Static analysis and fuzzing.



Online Automated Testing
------------------------

Travis CI is used for Android, Linux, and Mac OS X. Appveyor is used for
Windows. The tests are run in debug and release configurations, for the current
release of each Rust channel (Stable, Beta, Nightly), for each configuration
listed in the table below.

<table>
<tr><th>OS</th><th>Status</th>
<tr><td >Linux</td>
    <td rowspan=3><a title="Build Status" href=https://travis-ci.org/briansmith/unrusted><img src=https://travis-ci.org/briansmith/unrusted.svg?branch=master></a>
</tr>
<tr><td>Android</td></tr>
<tr><td>Mac&nbsp;OS&nbsp;X</td></tr>
<tr><td>Windows</td>
    <td><a title="Build Status" href=https://ci.appveyor.com/project/briansmith/unrusted/branch/master><img src="TODO"></a>
</tr>
</table>



Bug Reporting
-------------

Please report bugs either as pull requests or as issues in [the issue
tracker](https://github.com/briansmith/untrusted/issues). untrusted.rs has a
**full disclosure** vulnerability policy. **Please do NOT attempt to report
any security vulnerability in this code privately to anybody.**



License
-------

See [LICENSE.txt](LICENSE.txt), an ISC-style (simplified MIT) license.
