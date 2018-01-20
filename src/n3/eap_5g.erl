%% 5G Core Network - Untrusted non-3GPP Access control plane module
%% SoC
%%   3GPP TS 23.501 V15.0.0 (2017-12)
%%   TS 33.402 V8.2.1 (2008-12)
%%   EAP-AKA' https://tools.ietf.org/html/draft-arkko-eap-rfc5448bis-00

-define(PRINT(Var), io:format("~p:~p - ~p~n", [?MODULE, ?LINE, Var])).

-module(eap_5g).
