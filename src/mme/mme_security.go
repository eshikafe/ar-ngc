-module(mme_security).


% A.3 KeNB derivation function
k_enb_df(KASME, Uplink_NAS_COUNT) ->
	FC = <<11:8>>,
	P0 = Uplink_NAS_COUNT,
	L0 = <<4:16>>,
	S = <<FC/binary, P0/binary, L0/binary>>,
	crypto:hmac(sha256, KASME, S).

% A.2 Kasme derivation function
% CK, IK - 256 bits
k_asme_df(CK, IK, MCC, MNC) ->
    MCC_String = integer_to_list(MCC),
    MNC_String = integer_to_list(MNC),
    [MCC3, MCC2, MCC1] = MCC_String,
    [MNC3, MNC2, MNC1] = MNC_String, % 2 or 3

    case length(MNC_String) of
    	2 ->
    		[MNC2, MNC1] = MNC_String,
    		MNC3 = "0";
    	3 ->
    		[MNC3, MNC2, MNC1] = MNC_String
    end,
    SN_id = <<(list_to_integer([MCC2])):4, (list_to_integer([MCC1])):4, 
              (list_to_integer([MCC3])):4, (list_to_integer([MNC3])):4,
              (list_to_integer([MNC2])):4, (list_to_integer([MNC1])):4>>,
	FC = <<10:8>,
	P0 = SN_id,
	L0 = <<3:16>>,
	P1 = SQN xor AK,
	L1 = <<6:16>>,
	S = <<FC/binary, P0/binary, L0/binary, P1/binary, L1/binary>>,
	Key = <<CK/binary, IK/binary>>,
	hmac_sha256(Key, S),
	SN_id.
