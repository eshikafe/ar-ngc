%% 5G Core
%% Generic Routing Encapsulation module
%%
%% Use third party libraries if available
%% 
%% userplane for untrusted non 3GPP Access
%% GRE over IPsec
%%
%% SoC
%%   RFC2784
%%   RFC1701
%%   3GPP TS 23.501 V15.0.0 (2017-12)
%%   Key and Sequence Number Extensions to GRE - RFC2890
-module(gre).

-record(gre_header, {
	c,
	reserved0,
	ver,
	proto_type,
	checksum,
	reserved1
	}).