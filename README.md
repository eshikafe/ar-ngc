# New Generation Core (NGC) 

NGC is an EPC/5G core network experiment
**Experimental**

## Motivation
- [Open5gs](https://github.com/acetcom/open5gs).
- [Free5gc](https://github.com/free5gc/free5gc).

## Why "new generation"?
Because it is written in new generation programming languages (Go and Rust). 
- Control plane/OAM/CLI in Go
- User plane in Rust/NFF-Go
- Platform: Docker (container)
- Hardware: Intel/AMD x86 


## Hardware Configuration
![NGC hardware configuration](ngc_hw.png)

### Contributions
Please feel free to contribute via a pull request or [Issues](https://github.com/eshikafe/ngc/issues)

Support required:
- Documentation of design specifications for each VNF
- Design and implementation of a robust and scalable SBA preferably a microservice architecture.
- VNF implementation in Golang


