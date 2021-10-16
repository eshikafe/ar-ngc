# New Generation Core (NGC)

![Github CI](https://github.com/eshikafe/ngc/workflows/CI/badge.svg)

An experimental [Ligato](https://ligato.io/) based 4G/5G cloud native core network.

**Experimental work**

- Each Cloud Native Function (CNF) will be based on the Ligato framework
- User plane will be based on `VPP`

## Hardware Configuration

![NGC hardware configuration](ngc_hw.png)

### Contributions

Please feel free to contribute via a pull request or [Issues](https://github.com/eshikafe/ngc/issues)

Support required:

- Design specification for each CNF based on the Ligato framework
- UPF implementation based on Ligato and VPP
- PFCP Ligato Agent implementation in Go
- 4G/5G mobility and session management implementation in Erlang
- Common services implementation in Go.
