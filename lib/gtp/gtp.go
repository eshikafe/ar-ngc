=======
// Copyright (c) 2020 ngc project
// Reference documents:
// 3GPP TS 29.281 version 15.3.0 Release 15 - GTPv1-U
// 3GPP TS 29.274 version 15.4.0 Release 15 - GTPv2-C
// https://www.etsi.org/deliver/etsi_ts/129200_129299/129281/15.03.00_60/ts_129281v150300p.pdf
// https://www.etsi.org/deliver/etsi_ts/129200_129299/129274/15.04.00_60/ts_129274v150400p.pdf
package gtp

func GTPVersion() string {
	return "User plane: GTPv1-U (3GPP TS 29.281)\nControl plane: GTPv2-C (3GPP TS 29.274)."
}
