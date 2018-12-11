%% EPC/5G Core
%% Copyright 2018 Aigbe Research
%% 
%% MME S1 service
%%
%% Reference: 
%% 3GPP TS 36.413 - S1 Application Protocol 
%% 3GPP TS 36.412 - S1 signalling transport  

-module(mme_s1).


-export([start/2, stop/1, stop_all/0, update/2]).

-include_lib("mme_s1.hrl").


%%----------------------------------------------------------------
%% @doc Starts the MME S1 Service.
%%
%% Example(s) Starting a process 
%% ```
%% > mme_s1:start(server, #{
%%         mme_context => "lgmme01"
%%         callback => mme_s1_callbacks,
%%         plugins => [mme_s1_plugins],
%%         ip4_addr => ["10.23.24.1","10.23.24.2"],
%%         ip6_addr => []
%%     }).
start(SvcName, ConfigOpts) ->
	ConfigOpts1 = nklib_util:to_map(ConfigOpts),
	ConfigOpts2 = ConfigOpts1#{
		sctp_port = ?SCTP_PORT,
		transport = sctp,
	},
	nkservice:start(SvcName, ConfigOpts2).
