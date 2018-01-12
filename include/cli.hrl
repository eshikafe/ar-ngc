%% cli.hrl - CLI shell
%% Copyright 2017 Aigbe Research


% sub command
-record(cmd, {
	name,         % command name
	description,  % command description
	func_name,    % function to execute command
	func_param,    % function parameter
	sub_command
	}).

%% port sub command
-record(port_cmd, {
				p_type, % ethernet
				p_slot, % integer [0..4]
				portno, % integer
				p_state % up or down
	}).

%% chassis sub command
-record(chassis, {
		slot
	}).