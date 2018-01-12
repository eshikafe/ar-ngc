%% ngc - test case module
%%
%% https://tools.ietf.org/html/draft-arkko-eap-rfc5448bis-00

-module(test).

%% EAP Test Cases
eap_5g_tc1(Id, Network, RAND, AUTN, IK, CK, RES) ->
	CK_p = derived_ck(),
	IK_p = derived_ik(),
	K_encr = k(),
	K_aut = Ka(),
	K_re = Kr(),
	MSK = msk(),
	EMSK = emsk(),
	{CK_p, IK_p, K_encr, K_aut, K_re, MSK, EMSK}.

