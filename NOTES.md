# Dev Notes
Modify Zeep to output SOAP 1.2 stubs.

# Key Changes
* [ ] XML Namespace `http://schemas.xmlsoap.org/soap/envelope/` -> `http://www.w3.org/2003/05/soap-envelope`
* [ ] Encoding Style `http://schemas.xmlsoap.org/soap/encoding/` -> `http://www.w3.org/2003/05/soap-encoding` or `http://www.w3.org/2003/05/soap-envelope/encoding/none`
  * Maybe remove it altogether, as it's not required.
* [ ] Remove soapAction header?
* [ ] Content Type `application/soap+xml`
* [ ] SOAP Fault structure
  * Emitted by `FileWriter::fault_soap_wrapper` and `print_binding_operation`.

## Not Changing
Potentially interesting feature additions, but outside the scope of this task:
* Toggle between SOAP 1.1 and 1.2
* Rework to use Quote?
* Parse args with Clap
* Use custom Reqwest client
  * See: `zeep-lib/src/writer.rs:1575`

# Program Structure
Main entry point is zeep's very minimal `main.rs`, which gathers input and hands control to `zeep_lib::FileWriter::process_file`, which does most of the actual work.

The `process_file` function emits boilerplate (`print_global_header` and `print_common_structs`), sets up elements for the generated Rust modules (`init_modules`), and processes files.

The generated file contains a static header and common structs, followed by modules that correspond to the WSDL's messages, types, ports, bindings, and services. These are handled by the `Element` structure, which encapsulates XML and its related Rust code.

# Reference
## Soap
* [Latest SOAP Versions - W3C](https://www.w3.org/TR/soap/): Last updated in 2007.
  * [SOAP Version 1.2 Part0: Primer](https://www.w3.org/TR/2007/REC-soap12-part0-20070427/)
  * [SOAP Version 1.2 Part1: Messaging Framework](https://www.w3.org/TR/2007/REC-soap12-part1-20070427/)
    * [5. SOAP Message Construct](https://www.w3.org/TR/soap12/#soapenv): Describes the Envelope, Header, Body, and Fault.
  * [SOAP Version 1.2 Part2: Adjuncts](https://www.w3.org/TR/2007/REC-soap12-part2-20070427/)
  * [SOAP Version 1.2 Part3: Specification Assertions and Test Collections](https://www.w3.org/TR/2007/REC-soap12-testcollection-20070427/)
  * [SOAP 1.1](https://www.w3.org/TR/2000/NOTE-SOAP-20000508/#_Toc478383494)
    * [4. SOAP Envelope](https://www.w3.org/TR/2000/NOTE-SOAP-20000508/#_Toc478383494)
    * [A. SOAP Envelope Examples](https://www.w3.org/TR/2000/NOTE-SOAP-20000508/#_Toc478383539)
* [From SOAP/1.1 to SOAP Version 1.2 in 9 points - W3C](https://www.w3.org/2003/06/soap11-soap12)
* [Differences in SOAP versions - IBM](https://www.ibm.com/docs/en/was-nd/9.0.5?topic=soap-differences-in-versions)
* [SOAP - Wikipedia](https://en.wikipedia.org/wiki/SOAP)

## Rust
* [Calling SOAP APIs from Rust](https://127.io/2024/08/10/calling-soap-apis-from-rust/):
  Generally recommends against using SOAP from Rust.
  * Uses `zeep` to handle VMware's extremely large and complex WSDL (with manual tweaks).
* [mibes404/zeep](https://github.com/mibes404/zeep): SOAP Stub Generator.
* [yaserde](https://crates.io/crates/yaserde): Yet Another Serializer/Deserializer specialized for XML.
  * [examples/src/generic.rs](https://github.com/media-io/yaserde/blob/main/examples/src/generic.rs): Good example of
    minimal SOAP envelope.

## Miscellaneous

