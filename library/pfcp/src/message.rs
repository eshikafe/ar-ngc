// Copyright (c) 2022 VPlane Telecoms

// 5.1 General format
pub const PFCP_HEADER_LEN: u8 = 16;
pub const PFCP_SEID_LEN: u8 = 8;

// 7.3 Message Types
pub const PFCP_HEARTBEAT_REQUEST: u8 = 1;
pub const PFCP_HEARTBEAT_RESPONSE: u8 = 2;
pub const PFCP_PFD_MANAGEMENT_REQUEST: u8 = 3;
pub const PFCP_PFD_MANAGEMENT_RESPONSE: u8 = 4;
pub const PFCP_ASSOCIATION_SETUP_REQUEST: u8 = 5;
pub const PFCP_ASSOCIATION_SETUP_RESPONSE: u8 = 6;
pub const PFCP_ASSOCIATION_UPDATE_REQUEST: u8 = 7;
pub const PFCP_ASSOCIATION_UPDATE_RESPONSE: u8 = 8;
pub const PFCP_ASSOCIATION_RELEASE_REQUEST: u8 = 9;
pub const PFCP_ASSOCIATION_RELEASE_RESPONSE: u8 = 10;
pub const PFCP_VERSION_NOT_SUPPORTED_RESPONSE: u8 = 11;
pub const PFCP_NODE_REPORT_REQUEST: u8 = 12;
pub const PFCP_NODE_REPORT_RESPONSE: u8 = 13;
pub const PFCP_SESSION_SET_DELETION_REQUEST: u8 = 14;
pub const PFCP_SESSION_SET_DELETION_RESPONSE: u8 = 15;
pub const PFCP_SESSION_ESTABLISHMENT_REQUEST: u8 = 50;
pub const PFCP_SESSION_ESTABLISHMENT_RESPONSE: u8 = 51;
pub const PFCP_SESSION_MODIFICATION_REQUEST: u8 = 52;
pub const PFCP_SESSION_MODIFICATION_RESPONSE: u8 = 53;
pub const PFCP_SESSION_DELETION_REQUEST: u8 = 54;
pub const PFCP_SESSION_DELETION_RESPONSE: u8 = 55;
pub const PFCP_SESSION_REPORT_REQUEST: u8 = 56;
pub const PFCP_SESSION_REPORT_RESPONSE: u8 = 57;


// /* Information Element TLV Descriptor */
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_cause;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_source_interface;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_f_teid;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_network_instance;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_sdf_filter;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_application_id;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_gate_status;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_mbr;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_gbr;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_qer_correlation_id;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_precedence;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_transport_level_marking;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_volume_threshold;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_time_threshold;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_monitoring_time;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_subsequent_volume_threshold;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_subsequent_time_threshold;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_inactivity_detection_time;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_reporting_triggers;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_redirect_information;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_report_type;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_offending_ie;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_forwarding_policy;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_destination_interface;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_up_function_features;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_apply_action;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_downlink_data_service_information;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_downlink_data_notification_delay;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_dl_buffering_duration;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_dl_buffering_suggested_packet_count;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_pfcpsmreq_flags;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_pfcpsrrsp_flags;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_sequence_number;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_metric;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_timer;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_pdr_id;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_f_seid;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_node_id;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_pfd_contents;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_measurement_method;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_usage_report_trigger;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_measurement_period;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_fq_csid;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_volume_measurement;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_duration_measurement;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_time_of_first_packet;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_time_of_last_packet;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_quota_holding_time;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_dropped_dl_traffic_threshold;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_volume_quota;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_time_quota;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_start_time;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_end_time;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_urr_id;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_linked_urr_id;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_outer_header_creation;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_bar_id;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_cp_function_features;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_usage_information;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_application_instance_id;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_flow_information;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_ue_ip_address;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_packet_rate;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_outer_header_removal;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_recovery_time_stamp;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_dl_flow_level_marking;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_header_enrichment;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_measurement_information;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_node_report_type;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_remote_gtp_u_peer;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_ur_seqn;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_activate_predefined_rules;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_deactivate_predefined_rules;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_far_id;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_qer_id;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_oci_flags;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_pfcp_association_release_request;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_graceful_release_period;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_pdn_type;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_failed_rule_id;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_time_quota_mechanism;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_user_plane_ip_resource_information;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_user_plane_inactivity_timer;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_aggregated_urrs;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_multiplier;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_aggregated_urr_id;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_subsequent_volume_quota;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_subsequent_time_quota;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_rqi;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_qfi;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_query_urr_reference;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_additional_usage_reports_information;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_update_traffic_endpoint;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_traffic_endpoint_id;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_mac_address;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_c_tag;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_s_tag;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_ethertype;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_proxying;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_ethernet_filter_id;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_ethernet_filter_properties;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_suggested_buffering_packets_count;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_user_id;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_ethernet_pdu_session_information;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_mac_addresses_detected;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_mac_addresses_removed;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_ethernet_inactivity_timer;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_additional_monitoring_time;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_event_quota;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_event_threshold;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_subsequent_event_quota;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_subsequent_event_threshold;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_trace_information;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_framed_route;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_framed_routing;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_framed_ipv6_route;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_event_time_stamp;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_averaging_window;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_paging_policy_indicator;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_apn_dnn;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc__interface_type;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_pfcpsrreq_flags;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_pfcpaureq_flags;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_activation_time;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_deactivation_time;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_mar_id;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_steering_functionality;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_steering_mode;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_weight;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_priority;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_ue_ip_address_pool_identity;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_alternative_smf_ip_address;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_packet_replication_and_detection_carry_on_information;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_smf_set_id;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_quota_validity_time;

// /* Group Information Element TLV Descriptor */
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_ethernet_packet_filter;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_pdi;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_create_pdr;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_forwarding_parameters;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_duplicating_parameters;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_create_far;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_update_forwarding_parameters;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_update_duplicating_parameters;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_update_far;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_pfd_context;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_application_id_s_pfds;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_ethernet_traffic_information;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_access_forwarding_action_information_1;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_access_forwarding_action_information_2;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_update_access_forwarding_action_information_1;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_update_access_forwarding_action_information_2;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_create_urr;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_create_qer;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_created_pdr;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_update_pdr;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_update_bar_pfcp_session_report_response;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_update_urr;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_update_qer;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_remove_pdr;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_remove_far;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_remove_urr;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_remove_qer;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_load_control_information;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_overload_control_information;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_application_detection_information;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_query_urr;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_usage_report_session_modification_response;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_usage_report_session_deletion_response;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_usage_report_session_report_request;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_downlink_data_report;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_create_bar;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_update_bar_session_modification_request;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_remove_bar;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_error_indication_report;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_user_plane_path_failure_report;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_create_traffic_endpoint;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_created_traffic_endpoint;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_remove_traffic_endpoint;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_create_mar;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_remove_mar;
// extern ogs_tlv_desc_t ogs_pfcp_tlv_desc_update_mar;

// /* Message Descriptor */
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_heartbeat_request;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_heartbeat_response;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_pfd_management_request;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_pfd_management_response;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_association_setup_request;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_association_setup_response;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_association_update_request;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_association_update_response;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_association_release_request;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_association_release_response;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_version_not_supported_response;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_node_report_request;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_node_report_response;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_session_set_deletion_request;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_session_set_deletion_response;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_session_establishment_request;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_session_establishment_response;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_session_modification_request;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_session_modification_response;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_session_deletion_request;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_session_deletion_response;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_session_report_request;
// extern ogs_tlv_desc_t ogs_pfcp_msg_desc_pfcp_session_report_response;

// /* Structure for Information Element */
// typedef ogs_tlv_uint8_t ogs_pfcp_tlv_cause_t;
// typedef ogs_tlv_uint8_t ogs_pfcp_tlv_source_interface_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_f_teid_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_network_instance_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_sdf_filter_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_application_id_t;
// typedef ogs_tlv_uint8_t ogs_pfcp_tlv_gate_status_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_mbr_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_gbr_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_qer_correlation_id_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_precedence_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_transport_level_marking_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_volume_threshold_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_time_threshold_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_monitoring_time_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_subsequent_volume_threshold_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_subsequent_time_threshold_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_inactivity_detection_time_t;
// typedef ogs_tlv_uint24_t ogs_pfcp_tlv_reporting_triggers_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_redirect_information_t;
// typedef ogs_tlv_uint8_t ogs_pfcp_tlv_report_type_t;
// typedef ogs_tlv_uint16_t ogs_pfcp_tlv_offending_ie_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_forwarding_policy_t;
// typedef ogs_tlv_uint8_t ogs_pfcp_tlv_destination_interface_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_up_function_features_t;
// typedef ogs_tlv_uint8_t ogs_pfcp_tlv_apply_action_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_downlink_data_service_information_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_downlink_data_notification_delay_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_dl_buffering_duration_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_dl_buffering_suggested_packet_count_t;
// typedef ogs_tlv_uint8_t ogs_pfcp_tlv_pfcpsmreq_flags_t;
// typedef ogs_tlv_uint8_t ogs_pfcp_tlv_pfcpsrrsp_flags_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_sequence_number_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_metric_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_timer_t;
// typedef ogs_tlv_uint16_t ogs_pfcp_tlv_pdr_id_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_f_seid_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_node_id_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_pfd_contents_t;
// typedef ogs_tlv_uint8_t ogs_pfcp_tlv_measurement_method_t;
// typedef ogs_tlv_uint24_t ogs_pfcp_tlv_usage_report_trigger_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_measurement_period_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_fq_csid_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_volume_measurement_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_duration_measurement_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_time_of_first_packet_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_time_of_last_packet_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_quota_holding_time_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_dropped_dl_traffic_threshold_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_volume_quota_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_time_quota_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_start_time_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_end_time_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_urr_id_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_linked_urr_id_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_outer_header_creation_t;
// typedef ogs_tlv_uint8_t ogs_pfcp_tlv_bar_id_t;
// typedef ogs_tlv_uint8_t ogs_pfcp_tlv_cp_function_features_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_usage_information_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_application_instance_id_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_flow_information_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_ue_ip_address_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_packet_rate_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_outer_header_removal_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_recovery_time_stamp_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_dl_flow_level_marking_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_header_enrichment_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_measurement_information_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_node_report_type_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_remote_gtp_u_peer_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_ur_seqn_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_activate_predefined_rules_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_deactivate_predefined_rules_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_far_id_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_qer_id_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_oci_flags_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_pfcp_association_release_request_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_graceful_release_period_t;
// typedef ogs_tlv_uint8_t ogs_pfcp_tlv_pdn_type_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_failed_rule_id_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_time_quota_mechanism_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_user_plane_ip_resource_information_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_user_plane_inactivity_timer_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_aggregated_urrs_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_multiplier_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_aggregated_urr_id_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_subsequent_volume_quota_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_subsequent_time_quota_t;
// typedef ogs_tlv_uint8_t ogs_pfcp_tlv_rqi_t;
// typedef ogs_tlv_uint8_t ogs_pfcp_tlv_qfi_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_query_urr_reference_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_additional_usage_reports_information_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_update_traffic_endpoint_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_traffic_endpoint_id_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_mac_address_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_c_tag_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_s_tag_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_ethertype_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_proxying_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_ethernet_filter_id_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_ethernet_filter_properties_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_suggested_buffering_packets_count_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_user_id_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_ethernet_pdu_session_information_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_mac_addresses_detected_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_mac_addresses_removed_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_ethernet_inactivity_timer_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_additional_monitoring_time_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_event_quota_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_event_threshold_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_subsequent_event_quota_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_subsequent_event_threshold_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_trace_information_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_framed_route_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_framed_routing_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_framed_ipv6_route_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_event_time_stamp_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_averaging_window_t;
// typedef ogs_tlv_uint8_t ogs_pfcp_tlv_paging_policy_indicator_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_apn_dnn_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv__interface_type_t;
// typedef ogs_tlv_uint8_t ogs_pfcp_tlv_pfcpsrreq_flags_t;
// typedef ogs_tlv_uint8_t ogs_pfcp_tlv_pfcpaureq_flags_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_activation_time_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_deactivation_time_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_mar_id_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_steering_functionality_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_steering_mode_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_weight_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_priority_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_ue_ip_address_pool_identity_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_alternative_smf_ip_address_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_packet_replication_and_detection_carry_on_information_t;
// typedef ogs_tlv_octet_t ogs_pfcp_tlv_smf_set_id_t;
// typedef ogs_tlv_uint32_t ogs_pfcp_tlv_quota_validity_time_t;

// /* Structure for Group Information Element */
// typedef struct ogs_pfcp_tlv_ethernet_packet_filter_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_ethernet_filter_id_t ethernet_filter_id;
//     ogs_pfcp_tlv_ethernet_filter_properties_t ethernet_filter_properties;
//     ogs_pfcp_tlv_mac_address_t mac_address;
//     ogs_pfcp_tlv_ethertype_t ethertype;
//     ogs_pfcp_tlv_c_tag_t c_tag;
//     ogs_pfcp_tlv_s_tag_t s_tag;
//     ogs_pfcp_tlv_sdf_filter_t sdf_filter[8];
// } ogs_pfcp_tlv_ethernet_packet_filter_t;

// typedef struct ogs_pfcp_tlv_pdi_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_source_interface_t source_interface;
//     ogs_pfcp_tlv_f_teid_t local_f_teid;
//     ogs_pfcp_tlv_network_instance_t network_instance;
//     ogs_pfcp_tlv_ue_ip_address_t ue_ip_address;
//     ogs_pfcp_tlv_traffic_endpoint_id_t traffic_endpoint_id;
//     ogs_pfcp_tlv_sdf_filter_t sdf_filter[8];
//     ogs_pfcp_tlv_application_id_t application_id;
//     ogs_pfcp_tlv_ethernet_pdu_session_information_t ethernet_pdu_session_information;
//     ogs_pfcp_tlv_ethernet_packet_filter_t ethernet_packet_filter;
//     ogs_pfcp_tlv_qfi_t qfi;
//     ogs_pfcp_tlv_framed_route_t framed_route;
//     ogs_pfcp_tlv_framed_routing_t framed_routing;
//     ogs_pfcp_tlv_framed_ipv6_route_t framed_ipv6_route;
//     ogs_pfcp_tlv__interface_type_t source_interface_type;
// } ogs_pfcp_tlv_pdi_t;

// typedef struct ogs_pfcp_tlv_create_pdr_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_pdr_id_t pdr_id;
//     ogs_pfcp_tlv_precedence_t precedence;
//     ogs_pfcp_tlv_pdi_t pdi;
//     ogs_pfcp_tlv_outer_header_removal_t outer_header_removal;
//     ogs_pfcp_tlv_far_id_t far_id;
//     ogs_pfcp_tlv_urr_id_t urr_id[8];
//     ogs_pfcp_tlv_qer_id_t qer_id;
//     ogs_pfcp_tlv_activate_predefined_rules_t activate_predefined_rules;
//     ogs_pfcp_tlv_activation_time_t activation_time;
//     ogs_pfcp_tlv_deactivation_time_t deactivation_time;
//     ogs_pfcp_tlv_mar_id_t mar_id;
//     ogs_pfcp_tlv_packet_replication_and_detection_carry_on_information_t packet_replication_and_detection_carry_on_information;
// } ogs_pfcp_tlv_create_pdr_t;

// typedef struct ogs_pfcp_tlv_forwarding_parameters_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_destination_interface_t destination_interface;
//     ogs_pfcp_tlv_network_instance_t network_instance;
//     ogs_pfcp_tlv_redirect_information_t redirect_information;
//     ogs_pfcp_tlv_outer_header_creation_t outer_header_creation;
//     ogs_pfcp_tlv_transport_level_marking_t transport_level_marking;
//     ogs_pfcp_tlv_forwarding_policy_t forwarding_policy;
//     ogs_pfcp_tlv_header_enrichment_t header_enrichment;
//     ogs_pfcp_tlv_traffic_endpoint_id_t linked_traffic_endpoint_id;
//     ogs_pfcp_tlv_proxying_t proxying;
//     ogs_pfcp_tlv__interface_type_t destination_interface_type;
// } ogs_pfcp_tlv_forwarding_parameters_t;

// typedef struct ogs_pfcp_tlv_duplicating_parameters_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_destination_interface_t destination_interface;
//     ogs_pfcp_tlv_outer_header_creation_t outer_header_creation;
//     ogs_pfcp_tlv_transport_level_marking_t transport_level_marking;
//     ogs_pfcp_tlv_forwarding_policy_t forwarding_policy;
// } ogs_pfcp_tlv_duplicating_parameters_t;

// typedef struct ogs_pfcp_tlv_create_far_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_far_id_t far_id;
//     ogs_pfcp_tlv_apply_action_t apply_action;
//     ogs_pfcp_tlv_forwarding_parameters_t forwarding_parameters;
//     ogs_pfcp_tlv_duplicating_parameters_t duplicating_parameters;
//     ogs_pfcp_tlv_bar_id_t bar_id;
// } ogs_pfcp_tlv_create_far_t;

// typedef struct ogs_pfcp_tlv_update_forwarding_parameters_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_destination_interface_t destination_interface;
//     ogs_pfcp_tlv_network_instance_t network_instance;
//     ogs_pfcp_tlv_redirect_information_t redirect_information;
//     ogs_pfcp_tlv_outer_header_creation_t outer_header_creation;
//     ogs_pfcp_tlv_transport_level_marking_t transport_level_marking;
//     ogs_pfcp_tlv_forwarding_policy_t forwarding_policy;
//     ogs_pfcp_tlv_header_enrichment_t header_enrichment;
//     ogs_pfcp_tlv_pfcpsmreq_flags_t pfcpsmreq_flags;
//     ogs_pfcp_tlv_traffic_endpoint_id_t linked_traffic_endpoint_id;
//     ogs_pfcp_tlv__interface_type_t destination_interface_type;
// } ogs_pfcp_tlv_update_forwarding_parameters_t;

// typedef struct ogs_pfcp_tlv_update_duplicating_parameters_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_destination_interface_t destination_interface;
//     ogs_pfcp_tlv_outer_header_creation_t outer_header_creation;
//     ogs_pfcp_tlv_transport_level_marking_t transport_level_marking;
//     ogs_pfcp_tlv_forwarding_policy_t forwarding_policy;
// } ogs_pfcp_tlv_update_duplicating_parameters_t;

// typedef struct ogs_pfcp_tlv_update_far_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_far_id_t far_id;
//     ogs_pfcp_tlv_apply_action_t apply_action;
//     ogs_pfcp_tlv_update_forwarding_parameters_t update_forwarding_parameters;
//     ogs_pfcp_tlv_update_duplicating_parameters_t update_duplicating_parameters;
//     ogs_pfcp_tlv_bar_id_t bar_id;
// } ogs_pfcp_tlv_update_far_t;

// typedef struct ogs_pfcp_tlv_pfd_context_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_pfd_contents_t pfd_contents;
// } ogs_pfcp_tlv_pfd_context_t;

// typedef struct ogs_pfcp_tlv_application_id_s_pfds_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_application_id_t application_id;
//     ogs_pfcp_tlv_pfd_context_t pfd_context;
// } ogs_pfcp_tlv_application_id_s_pfds_t;

// typedef struct ogs_pfcp_tlv_ethernet_traffic_information_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_mac_addresses_detected_t mac_addresses_detected;
//     ogs_pfcp_tlv_mac_addresses_removed_t mac_addresses_removed;
// } ogs_pfcp_tlv_ethernet_traffic_information_t;

// typedef struct ogs_pfcp_tlv_access_forwarding_action_information_1_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_far_id_t far_id;
//     ogs_pfcp_tlv_weight_t weight;
//     ogs_pfcp_tlv_priority_t priority;
//     ogs_pfcp_tlv_urr_id_t urr_id[8];
// } ogs_pfcp_tlv_access_forwarding_action_information_1_t;

// typedef struct ogs_pfcp_tlv_access_forwarding_action_information_2_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_far_id_t far_id;
//     ogs_pfcp_tlv_weight_t weight;
//     ogs_pfcp_tlv_priority_t priority;
//     ogs_pfcp_tlv_urr_id_t urr_id[8];
// } ogs_pfcp_tlv_access_forwarding_action_information_2_t;

// typedef struct ogs_pfcp_tlv_update_access_forwarding_action_information_1_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_far_id_t far_id;
//     ogs_pfcp_tlv_weight_t weight;
//     ogs_pfcp_tlv_priority_t priority;
//     ogs_pfcp_tlv_urr_id_t urr_id;
// } ogs_pfcp_tlv_update_access_forwarding_action_information_1_t;

// typedef struct ogs_pfcp_tlv_update_access_forwarding_action_information_2_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_far_id_t far_id;
//     ogs_pfcp_tlv_weight_t weight;
//     ogs_pfcp_tlv_priority_t priority;
//     ogs_pfcp_tlv_urr_id_t urr_id;
// } ogs_pfcp_tlv_update_access_forwarding_action_information_2_t;

// typedef struct ogs_pfcp_tlv_create_urr_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_urr_id_t urr_id;
//     ogs_pfcp_tlv_measurement_method_t measurement_method;
//     ogs_pfcp_tlv_reporting_triggers_t reporting_triggers;
//     ogs_pfcp_tlv_measurement_period_t measurement_period;
//     ogs_pfcp_tlv_volume_threshold_t volume_threshold;
//     ogs_pfcp_tlv_volume_quota_t volume_quota;
//     ogs_pfcp_tlv_event_threshold_t event_threshold;
//     ogs_pfcp_tlv_event_quota_t event_quota;
//     ogs_pfcp_tlv_time_threshold_t time_threshold;
//     ogs_pfcp_tlv_time_quota_t time_quota;
//     ogs_pfcp_tlv_quota_holding_time_t quota_holding_time;
//     ogs_pfcp_tlv_dropped_dl_traffic_threshold_t dropped_dl_traffic_threshold;
//     ogs_pfcp_tlv_quota_validity_time_t quota_validity_time;
//     ogs_pfcp_tlv_monitoring_time_t monitoring_time;
//     ogs_pfcp_tlv_subsequent_volume_threshold_t subsequent_volume_threshold;
//     ogs_pfcp_tlv_subsequent_time_threshold_t subsequent_time_threshold;
//     ogs_pfcp_tlv_subsequent_volume_quota_t subsequent_volume_quota;
//     ogs_pfcp_tlv_subsequent_time_quota_t subsequent_time_quota;
//     ogs_pfcp_tlv_subsequent_event_threshold_t subsequent_event_threshold;
//     ogs_pfcp_tlv_subsequent_event_quota_t subsequent_event_quota;
//     ogs_pfcp_tlv_inactivity_detection_time_t inactivity_detection_time;
//     ogs_pfcp_tlv_linked_urr_id_t linked_urr_id;
//     ogs_pfcp_tlv_measurement_information_t measurement_information;
//     ogs_pfcp_tlv_time_quota_mechanism_t time_quota_mechanism;
//     ogs_pfcp_tlv_aggregated_urrs_t aggregated_urrs;
//     ogs_pfcp_tlv_far_id_t far_id_for_quota_action;
//     ogs_pfcp_tlv_ethernet_inactivity_timer_t ethernet_inactivity_timer;
//     ogs_pfcp_tlv_additional_monitoring_time_t additional_monitoring_time;
// } ogs_pfcp_tlv_create_urr_t;

// typedef struct ogs_pfcp_tlv_create_qer_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_qer_id_t qer_id;
//     ogs_pfcp_tlv_qer_correlation_id_t qer_correlation_id;
//     ogs_pfcp_tlv_gate_status_t gate_status;
//     ogs_pfcp_tlv_mbr_t maximum_bitrate;
//     ogs_pfcp_tlv_gbr_t guaranteed_bitrate;
//     ogs_pfcp_tlv_packet_rate_t packet_rate;
//     ogs_pfcp_tlv_dl_flow_level_marking_t dl_flow_level_marking;
//     ogs_pfcp_tlv_qfi_t qos_flow_identifier;
//     ogs_pfcp_tlv_rqi_t reflective_qos;
//     ogs_pfcp_tlv_paging_policy_indicator_t paging_policy_indicator;
//     ogs_pfcp_tlv_averaging_window_t averaging_window;
// } ogs_pfcp_tlv_create_qer_t;

// typedef struct ogs_pfcp_tlv_created_pdr_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_pdr_id_t pdr_id;
//     ogs_pfcp_tlv_f_teid_t local_f_teid;
//     ogs_pfcp_tlv_ue_ip_address_t ue_ip_address;
// } ogs_pfcp_tlv_created_pdr_t;

// typedef struct ogs_pfcp_tlv_update_pdr_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_pdr_id_t pdr_id;
//     ogs_pfcp_tlv_outer_header_removal_t outer_header_removal;
//     ogs_pfcp_tlv_precedence_t precedence;
//     ogs_pfcp_tlv_pdi_t pdi;
//     ogs_pfcp_tlv_far_id_t far_id;
//     ogs_pfcp_tlv_urr_id_t urr_id;
//     ogs_pfcp_tlv_qer_id_t qer_id;
//     ogs_pfcp_tlv_activate_predefined_rules_t activate_predefined_rules;
//     ogs_pfcp_tlv_deactivate_predefined_rules_t deactivate_predefined_rules;
//     ogs_pfcp_tlv_activation_time_t activation_time;
//     ogs_pfcp_tlv_deactivation_time_t deactivation_time;
// } ogs_pfcp_tlv_update_pdr_t;

// typedef struct ogs_pfcp_tlv_update_bar_pfcp_session_report_response_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_bar_id_t bar_id;
//     ogs_pfcp_tlv_downlink_data_notification_delay_t downlink_data_notification_delay;
//     ogs_pfcp_tlv_dl_buffering_duration_t dl_buffering_duration;
//     ogs_pfcp_tlv_dl_buffering_suggested_packet_count_t dl_buffering_suggested_packet_count;
//     ogs_pfcp_tlv_suggested_buffering_packets_count_t suggested_buffering_packets_count;
// } ogs_pfcp_tlv_update_bar_pfcp_session_report_response_t;

// typedef struct ogs_pfcp_tlv_update_urr_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_urr_id_t urr_id;
//     ogs_pfcp_tlv_measurement_method_t measurement_method;
//     ogs_pfcp_tlv_reporting_triggers_t reporting_triggers;
//     ogs_pfcp_tlv_measurement_period_t measurement_period;
//     ogs_pfcp_tlv_volume_threshold_t volume_threshold;
//     ogs_pfcp_tlv_volume_quota_t volume_quota;
//     ogs_pfcp_tlv_time_threshold_t time_threshold;
//     ogs_pfcp_tlv_time_quota_t time_quota;
//     ogs_pfcp_tlv_event_threshold_t event_threshold;
//     ogs_pfcp_tlv_event_quota_t event_quota;
//     ogs_pfcp_tlv_quota_holding_time_t quota_holding_time;
//     ogs_pfcp_tlv_dropped_dl_traffic_threshold_t dropped_dl_traffic_threshold;
//     ogs_pfcp_tlv_quota_validity_time_t quota_validity_time;
//     ogs_pfcp_tlv_monitoring_time_t monitoring_time;
//     ogs_pfcp_tlv_subsequent_volume_threshold_t subsequent_volume_threshold;
//     ogs_pfcp_tlv_subsequent_time_threshold_t subsequent_time_threshold;
//     ogs_pfcp_tlv_subsequent_volume_quota_t subsequent_volume_quota;
//     ogs_pfcp_tlv_subsequent_time_quota_t subsequent_time_quota;
//     ogs_pfcp_tlv_subsequent_event_threshold_t subsequent_event_threshold;
//     ogs_pfcp_tlv_subsequent_event_quota_t subsequent_event_quota;
//     ogs_pfcp_tlv_inactivity_detection_time_t inactivity_detection_time;
//     ogs_pfcp_tlv_linked_urr_id_t linked_urr_id;
//     ogs_pfcp_tlv_measurement_information_t measurement_information;
//     ogs_pfcp_tlv_time_quota_mechanism_t time_quota_mechanism;
//     ogs_pfcp_tlv_aggregated_urrs_t aggregated_urrs;
//     ogs_pfcp_tlv_far_id_t far_id_for_quota_action;
//     ogs_pfcp_tlv_ethernet_inactivity_timer_t ethernet_inactivity_timer;
//     ogs_pfcp_tlv_additional_monitoring_time_t additional_monitoring_time;
// } ogs_pfcp_tlv_update_urr_t;

// typedef struct ogs_pfcp_tlv_update_qer_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_qer_id_t qer_id;
//     ogs_pfcp_tlv_qer_correlation_id_t qer_correlation_id;
//     ogs_pfcp_tlv_gate_status_t gate_status;
//     ogs_pfcp_tlv_mbr_t maximum_bitrate;
//     ogs_pfcp_tlv_gbr_t guaranteed_bitrate;
//     ogs_pfcp_tlv_packet_rate_t packet_rate;
//     ogs_pfcp_tlv_dl_flow_level_marking_t dl_flow_level_marking;
//     ogs_pfcp_tlv_qfi_t qos_flow_identifier;
//     ogs_pfcp_tlv_rqi_t reflective_qos;
//     ogs_pfcp_tlv_paging_policy_indicator_t paging_policy_indicator;
//     ogs_pfcp_tlv_averaging_window_t averaging_window;
// } ogs_pfcp_tlv_update_qer_t;

// typedef struct ogs_pfcp_tlv_remove_pdr_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_pdr_id_t pdr_id;
// } ogs_pfcp_tlv_remove_pdr_t;

// typedef struct ogs_pfcp_tlv_remove_far_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_far_id_t far_id;
// } ogs_pfcp_tlv_remove_far_t;

// typedef struct ogs_pfcp_tlv_remove_urr_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_urr_id_t urr_id;
// } ogs_pfcp_tlv_remove_urr_t;

// typedef struct ogs_pfcp_tlv_remove_qer_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_qer_id_t qer_id;
// } ogs_pfcp_tlv_remove_qer_t;

// typedef struct ogs_pfcp_tlv_load_control_information_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_sequence_number_t load_control_sequence_number;
//     ogs_pfcp_tlv_metric_t load_metric;
// } ogs_pfcp_tlv_load_control_information_t;

// typedef struct ogs_pfcp_tlv_overload_control_information_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_sequence_number_t overload_control_sequence_number;
//     ogs_pfcp_tlv_metric_t overload_reduction_metric;
//     ogs_pfcp_tlv_timer_t period_of_validity;
//     ogs_pfcp_tlv_oci_flags_t overload_control_information_flags;
// } ogs_pfcp_tlv_overload_control_information_t;

// typedef struct ogs_pfcp_tlv_application_detection_information_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_application_id_t application_id;
//     ogs_pfcp_tlv_application_instance_id_t application_instance_id;
//     ogs_pfcp_tlv_flow_information_t flow_information;
// } ogs_pfcp_tlv_application_detection_information_t;

// typedef struct ogs_pfcp_tlv_query_urr_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_urr_id_t urr_id;
// } ogs_pfcp_tlv_query_urr_t;

// typedef struct ogs_pfcp_tlv_usage_report_session_modification_response_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_urr_id_t urr_id;
//     ogs_pfcp_tlv_ur_seqn_t ur_seqn;
//     ogs_pfcp_tlv_usage_report_trigger_t usage_report_trigger;
//     ogs_pfcp_tlv_start_time_t start_time;
//     ogs_pfcp_tlv_end_time_t end_time;
//     ogs_pfcp_tlv_volume_measurement_t volume_measurement;
//     ogs_pfcp_tlv_duration_measurement_t duration_measurement;
//     ogs_pfcp_tlv_time_of_first_packet_t time_of_first_packet;
//     ogs_pfcp_tlv_time_of_last_packet_t time_of_last_packet;
//     ogs_pfcp_tlv_usage_information_t usage_information;
//     ogs_pfcp_tlv_query_urr_reference_t query_urr_reference;
//     ogs_pfcp_tlv_ethernet_traffic_information_t ethernet_traffic_information;
// } ogs_pfcp_tlv_usage_report_session_modification_response_t;

// typedef struct ogs_pfcp_tlv_usage_report_session_deletion_response_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_urr_id_t urr_id;
//     ogs_pfcp_tlv_ur_seqn_t ur_seqn;
//     ogs_pfcp_tlv_usage_report_trigger_t usage_report_trigger;
//     ogs_pfcp_tlv_start_time_t start_time;
//     ogs_pfcp_tlv_end_time_t end_time;
//     ogs_pfcp_tlv_volume_measurement_t volume_measurement;
//     ogs_pfcp_tlv_duration_measurement_t duration_measurement;
//     ogs_pfcp_tlv_time_of_first_packet_t time_of_first_packet;
//     ogs_pfcp_tlv_time_of_last_packet_t time_of_last_packet;
//     ogs_pfcp_tlv_usage_information_t usage_information;
//     ogs_pfcp_tlv_ethernet_traffic_information_t ethernet_traffic_information;
// } ogs_pfcp_tlv_usage_report_session_deletion_response_t;

// typedef struct ogs_pfcp_tlv_usage_report_session_report_request_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_urr_id_t urr_id;
//     ogs_pfcp_tlv_ur_seqn_t ur_seqn;
//     ogs_pfcp_tlv_usage_report_trigger_t usage_report_trigger;
//     ogs_pfcp_tlv_start_time_t start_time;
//     ogs_pfcp_tlv_end_time_t end_time;
//     ogs_pfcp_tlv_volume_measurement_t volume_measurement;
//     ogs_pfcp_tlv_duration_measurement_t duration_measurement;
//     ogs_pfcp_tlv_application_detection_information_t application_detection_information;
//     ogs_pfcp_tlv_ue_ip_address_t ue_ip_address;
//     ogs_pfcp_tlv_network_instance_t network_instance;
//     ogs_pfcp_tlv_time_of_first_packet_t time_of_first_packet;
//     ogs_pfcp_tlv_time_of_last_packet_t time_of_last_packet;
//     ogs_pfcp_tlv_usage_information_t usage_information;
//     ogs_pfcp_tlv_query_urr_reference_t query_urr_reference;
//     ogs_pfcp_tlv_event_time_stamp_t event_time_stamp;
//     ogs_pfcp_tlv_ethernet_traffic_information_t ethernet_traffic_information;
// } ogs_pfcp_tlv_usage_report_session_report_request_t;

// typedef struct ogs_pfcp_tlv_downlink_data_report_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_pdr_id_t pdr_id;
//     ogs_pfcp_tlv_downlink_data_service_information_t downlink_data_service_information;
// } ogs_pfcp_tlv_downlink_data_report_t;

// typedef struct ogs_pfcp_tlv_create_bar_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_bar_id_t bar_id;
//     ogs_pfcp_tlv_downlink_data_notification_delay_t downlink_data_notification_delay;
//     ogs_pfcp_tlv_suggested_buffering_packets_count_t suggested_buffering_packets_count;
// } ogs_pfcp_tlv_create_bar_t;

// typedef struct ogs_pfcp_tlv_update_bar_session_modification_request_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_bar_id_t bar_id;
//     ogs_pfcp_tlv_downlink_data_notification_delay_t downlink_data_notification_delay;
//     ogs_pfcp_tlv_suggested_buffering_packets_count_t suggested_buffering_packets_count;
// } ogs_pfcp_tlv_update_bar_session_modification_request_t;

// typedef struct ogs_pfcp_tlv_remove_bar_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_bar_id_t bar_id;
// } ogs_pfcp_tlv_remove_bar_t;

// typedef struct ogs_pfcp_tlv_error_indication_report_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_f_teid_t remote_f_teid;
// } ogs_pfcp_tlv_error_indication_report_t;

// typedef struct ogs_pfcp_tlv_user_plane_path_failure_report_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_remote_gtp_u_peer_t remote_gtp_u_peer_;
// } ogs_pfcp_tlv_user_plane_path_failure_report_t;

// typedef struct ogs_pfcp_tlv_create_traffic_endpoint_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_traffic_endpoint_id_t traffic_endpoint_id;
//     ogs_pfcp_tlv_f_teid_t local_f_teid;
//     ogs_pfcp_tlv_network_instance_t network_instance;
//     ogs_pfcp_tlv_ue_ip_address_t ue_ip_address;
//     ogs_pfcp_tlv_ethernet_pdu_session_information_t ethernet_pdu_session_information;
//     ogs_pfcp_tlv_framed_route_t framed_route;
//     ogs_pfcp_tlv_framed_routing_t framed_routing;
//     ogs_pfcp_tlv_framed_ipv6_route_t framed_ipv6_route;
//     ogs_pfcp_tlv_qfi_t qfi;
// } ogs_pfcp_tlv_create_traffic_endpoint_t;

// typedef struct ogs_pfcp_tlv_created_traffic_endpoint_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_traffic_endpoint_id_t traffic_endpoint_id;
//     ogs_pfcp_tlv_f_teid_t local_f_teid;
//     ogs_pfcp_tlv_ue_ip_address_t ue_ip_address;
// } ogs_pfcp_tlv_created_traffic_endpoint_t;

// typedef struct ogs_pfcp_tlv_remove_traffic_endpoint_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_traffic_endpoint_id_t traffic_endpoint_id;
// } ogs_pfcp_tlv_remove_traffic_endpoint_t;

// typedef struct ogs_pfcp_tlv_create_mar_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_mar_id_t mar_id;
//     ogs_pfcp_tlv_steering_functionality_t steering_functionality;
//     ogs_pfcp_tlv_steering_mode_t steering_mode;
//     ogs_pfcp_tlv_access_forwarding_action_information_1_t access_forwarding_action_information_1;
//     ogs_pfcp_tlv_access_forwarding_action_information_2_t access_forwarding_action_information_2;
// } ogs_pfcp_tlv_create_mar_t;

// typedef struct ogs_pfcp_tlv_remove_mar_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_mar_id_t mar_id;
// } ogs_pfcp_tlv_remove_mar_t;

// typedef struct ogs_pfcp_tlv_update_mar_s {
//     ogs_tlv_presence_t presence;
//     ogs_pfcp_tlv_mar_id_t mar_id;
//     ogs_pfcp_tlv_steering_functionality_t steering_functionality;
//     ogs_pfcp_tlv_steering_mode_t steering_mode;
//     ogs_pfcp_tlv_update_access_forwarding_action_information_1_t update_access_forwarding_action_information_1;
//     ogs_pfcp_tlv_update_access_forwarding_action_information_2_t update_access_forwarding_action_information_2;
//     ogs_pfcp_tlv_access_forwarding_action_information_1_t access_forwarding_action_information_1;
//     ogs_pfcp_tlv_access_forwarding_action_information_2_t access_forwarding_action_information_2;
// } ogs_pfcp_tlv_update_mar_t;

// /* Structure for Message */
// typedef struct ogs_pfcp_heartbeat_request_s {
//     ogs_pfcp_tlv_recovery_time_stamp_t recovery_time_stamp;
// } ogs_pfcp_heartbeat_request_t;

// typedef struct ogs_pfcp_heartbeat_response_s {
//     ogs_pfcp_tlv_recovery_time_stamp_t recovery_time_stamp;
// } ogs_pfcp_heartbeat_response_t;

// typedef struct ogs_pfcp_pfd_management_request_s {
//     ogs_pfcp_tlv_application_id_s_pfds_t application_id_s_pfds;
// } ogs_pfcp_pfd_management_request_t;

// typedef struct ogs_pfcp_pfd_management_response_s {
//     ogs_pfcp_tlv_cause_t cause;
//     ogs_pfcp_tlv_offending_ie_t offending_ie;
// } ogs_pfcp_pfd_management_response_t;

// typedef struct ogs_pfcp_association_setup_request_s {
//     ogs_pfcp_tlv_node_id_t node_id;
//     ogs_pfcp_tlv_recovery_time_stamp_t recovery_time_stamp;
//     ogs_pfcp_tlv_up_function_features_t up_function_features;
//     ogs_pfcp_tlv_cp_function_features_t cp_function_features;
//     ogs_pfcp_tlv_user_plane_ip_resource_information_t user_plane_ip_resource_information[4];
//     ogs_pfcp_tlv_ue_ip_address_t ue_ip_address_pool_identity;
//     ogs_pfcp_tlv_alternative_smf_ip_address_t alternative_smf_ip_address;
//     ogs_pfcp_tlv_smf_set_id_t smf_set_id;
// } ogs_pfcp_association_setup_request_t;

// typedef struct ogs_pfcp_association_setup_response_s {
//     ogs_pfcp_tlv_node_id_t node_id;
//     ogs_pfcp_tlv_cause_t cause;
//     ogs_pfcp_tlv_recovery_time_stamp_t recovery_time_stamp;
//     ogs_pfcp_tlv_up_function_features_t up_function_features;
//     ogs_pfcp_tlv_cp_function_features_t cp_function_features;
//     ogs_pfcp_tlv_user_plane_ip_resource_information_t user_plane_ip_resource_information[4];
//     ogs_pfcp_tlv_alternative_smf_ip_address_t alternative_smf_ip_address;
// } ogs_pfcp_association_setup_response_t;

// typedef struct ogs_pfcp_association_update_request_s {
//     ogs_pfcp_tlv_node_id_t node_id;
//     ogs_pfcp_tlv_up_function_features_t up_function_features;
//     ogs_pfcp_tlv_cp_function_features_t cp_function_features;
//     ogs_pfcp_tlv_pfcp_association_release_request_t pfcp_association_release_request;
//     ogs_pfcp_tlv_graceful_release_period_t graceful_release_period;
//     ogs_pfcp_tlv_user_plane_ip_resource_information_t user_plane_ip_resource_information[4];
//     ogs_pfcp_tlv_pfcpaureq_flags_t pfcpaureq_flags;
//     ogs_pfcp_tlv_alternative_smf_ip_address_t alternative_smf_ip_address;
// } ogs_pfcp_association_update_request_t;

// typedef struct ogs_pfcp_association_update_response_s {
//     ogs_pfcp_tlv_node_id_t node_id;
//     ogs_pfcp_tlv_cause_t cause;
//     ogs_pfcp_tlv_up_function_features_t up_function_features;
//     ogs_pfcp_tlv_cp_function_features_t cp_function_features;
// } ogs_pfcp_association_update_response_t;

// typedef struct ogs_pfcp_association_release_request_s {
//     ogs_pfcp_tlv_node_id_t node_id;
// } ogs_pfcp_association_release_request_t;

// typedef struct ogs_pfcp_association_release_response_s {
//     ogs_pfcp_tlv_node_id_t node_id;
//     ogs_pfcp_tlv_cause_t cause;
// } ogs_pfcp_association_release_response_t;

// typedef struct ogs_pfcp_version_not_supported_response_s {
// } ogs_pfcp_version_not_supported_response_t;

// typedef struct ogs_pfcp_node_report_request_s {
//     ogs_pfcp_tlv_node_id_t node_id;
//     ogs_pfcp_tlv_node_report_type_t node_report_type;
//     ogs_pfcp_tlv_user_plane_path_failure_report_t user_plane_path_failure_report;
// } ogs_pfcp_node_report_request_t;

// typedef struct ogs_pfcp_node_report_response_s {
//     ogs_pfcp_tlv_node_id_t node_id;
//     ogs_pfcp_tlv_cause_t cause;
//     ogs_pfcp_tlv_offending_ie_t offending_ie;
// } ogs_pfcp_node_report_response_t;

// typedef struct ogs_pfcp_session_set_deletion_request_s {
//     ogs_pfcp_tlv_node_id_t node_id;
//     ogs_pfcp_tlv_fq_csid_t sgw_c_fq_csid;
//     ogs_pfcp_tlv_fq_csid_t pgw_c_fq_csid;
//     ogs_pfcp_tlv_fq_csid_t sgw_u_fq_csid;
//     ogs_pfcp_tlv_fq_csid_t pgw_u_fq_csid;
//     ogs_pfcp_tlv_fq_csid_t twan_fq_csid;
//     ogs_pfcp_tlv_fq_csid_t epdg_fq_csid;
//     ogs_pfcp_tlv_fq_csid_t mme_fq_csid;
// } ogs_pfcp_session_set_deletion_request_t;

// typedef struct ogs_pfcp_session_set_deletion_response_s {
//     ogs_pfcp_tlv_node_id_t node_id;
//     ogs_pfcp_tlv_cause_t cause;
//     ogs_pfcp_tlv_offending_ie_t offending_ie;
// } ogs_pfcp_session_set_deletion_response_t;

// typedef struct ogs_pfcp_session_establishment_request_s {
//     ogs_pfcp_tlv_node_id_t node_id;
//     ogs_pfcp_tlv_f_seid_t cp_f_seid;
//     ogs_pfcp_tlv_create_pdr_t create_pdr[8];
//     ogs_pfcp_tlv_create_far_t create_far[8];
//     ogs_pfcp_tlv_create_urr_t create_urr[2];
//     ogs_pfcp_tlv_create_qer_t create_qer[4];
//     ogs_pfcp_tlv_create_bar_t create_bar;
//     ogs_pfcp_tlv_create_traffic_endpoint_t create_traffic_endpoint;
//     ogs_pfcp_tlv_pdn_type_t pdn_type;
//     ogs_pfcp_tlv_fq_csid_t sgw_c_fq_csid;
//     ogs_pfcp_tlv_fq_csid_t mme_fq_csid;
//     ogs_pfcp_tlv_fq_csid_t pgw_c_fq_csid;
//     ogs_pfcp_tlv_fq_csid_t epdg_fq_csid;
//     ogs_pfcp_tlv_fq_csid_t twan_fq_csid;
//     ogs_pfcp_tlv_user_plane_inactivity_timer_t user_plane_inactivity_timer;
//     ogs_pfcp_tlv_user_id_t user_id;
//     ogs_pfcp_tlv_trace_information_t trace_information;
//     ogs_pfcp_tlv_apn_dnn_t apn_dnn;
//     ogs_pfcp_tlv_create_mar_t create_mar;
// } ogs_pfcp_session_establishment_request_t;

// typedef struct ogs_pfcp_session_establishment_response_s {
//     ogs_pfcp_tlv_node_id_t node_id;
//     ogs_pfcp_tlv_cause_t cause;
//     ogs_pfcp_tlv_offending_ie_t offending_ie;
//     ogs_pfcp_tlv_f_seid_t up_f_seid;
//     ogs_pfcp_tlv_created_pdr_t created_pdr[8];
//     ogs_pfcp_tlv_load_control_information_t load_control_information;
//     ogs_pfcp_tlv_overload_control_information_t overload_control_information;
//     ogs_pfcp_tlv_fq_csid_t sgw_u_fq_csid;
//     ogs_pfcp_tlv_fq_csid_t pgw_u_fq_csid;
//     ogs_pfcp_tlv_failed_rule_id_t failed_rule_id;
//     ogs_pfcp_tlv_created_traffic_endpoint_t created_traffic_endpoint;
// } ogs_pfcp_session_establishment_response_t;

// typedef struct ogs_pfcp_session_modification_request_s {
//     ogs_pfcp_tlv_f_seid_t cp_f_seid;
//     ogs_pfcp_tlv_remove_pdr_t remove_pdr[8];
//     ogs_pfcp_tlv_remove_far_t remove_far[8];
//     ogs_pfcp_tlv_remove_urr_t remove_urr[2];
//     ogs_pfcp_tlv_remove_qer_t remove_qer[4];
//     ogs_pfcp_tlv_remove_bar_t remove_bar;
//     ogs_pfcp_tlv_remove_traffic_endpoint_t remove_traffic_endpoint;
//     ogs_pfcp_tlv_create_pdr_t create_pdr[8];
//     ogs_pfcp_tlv_create_far_t create_far[8];
//     ogs_pfcp_tlv_create_urr_t create_urr[2];
//     ogs_pfcp_tlv_create_qer_t create_qer[4];
//     ogs_pfcp_tlv_create_bar_t create_bar;
//     ogs_pfcp_tlv_create_traffic_endpoint_t create_traffic_endpoint;
//     ogs_pfcp_tlv_update_pdr_t update_pdr[8];
//     ogs_pfcp_tlv_update_far_t update_far[8];
//     ogs_pfcp_tlv_update_urr_t update_urr[2];
//     ogs_pfcp_tlv_update_qer_t update_qer[4];
//     ogs_pfcp_tlv_update_bar_session_modification_request_t update_bar;
//     ogs_pfcp_tlv_update_traffic_endpoint_t update_traffic_endpoint;
//     ogs_pfcp_tlv_pfcpsmreq_flags_t pfcpsmreq_flags;
//     ogs_pfcp_tlv_query_urr_t query_urr;
//     ogs_pfcp_tlv_fq_csid_t pgw_c_fq_csid;
//     ogs_pfcp_tlv_fq_csid_t sgw_c_fq_csid;
//     ogs_pfcp_tlv_fq_csid_t mme_fq_csid;
//     ogs_pfcp_tlv_fq_csid_t epdg_fq_csid;
//     ogs_pfcp_tlv_fq_csid_t twan_fq_csid;
//     ogs_pfcp_tlv_user_plane_inactivity_timer_t user_plane_inactivity_timer;
//     ogs_pfcp_tlv_query_urr_reference_t query_urr_reference;
//     ogs_pfcp_tlv_trace_information_t trace_information;
//     ogs_pfcp_tlv_remove_mar_t remove_mar;
//     ogs_pfcp_tlv_update_mar_t update_mar;
//     ogs_pfcp_tlv_create_mar_t create_mar;
//     ogs_pfcp_tlv_node_id_t node_id;
// } ogs_pfcp_session_modification_request_t;

// typedef struct ogs_pfcp_session_modification_response_s {
//     ogs_pfcp_tlv_cause_t cause;
//     ogs_pfcp_tlv_offending_ie_t offending_ie;
//     ogs_pfcp_tlv_created_pdr_t created_pdr[8];
//     ogs_pfcp_tlv_load_control_information_t load_control_information;
//     ogs_pfcp_tlv_overload_control_information_t overload_control_information;
//     ogs_pfcp_tlv_usage_report_session_modification_response_t usage_report[8];
//     ogs_pfcp_tlv_failed_rule_id_t failed_rule_id;
//     ogs_pfcp_tlv_additional_usage_reports_information_t additional_usage_reports_information;
//     ogs_pfcp_tlv_created_traffic_endpoint_t created_updated_traffic_endpoint;
// } ogs_pfcp_session_modification_response_t;

// typedef struct ogs_pfcp_session_deletion_request_s {
// } ogs_pfcp_session_deletion_request_t;

// typedef struct ogs_pfcp_session_deletion_response_s {
//     ogs_pfcp_tlv_cause_t cause;
//     ogs_pfcp_tlv_offending_ie_t offending_ie;
//     ogs_pfcp_tlv_load_control_information_t load_control_information;
//     ogs_pfcp_tlv_overload_control_information_t overload_control_information;
//     ogs_pfcp_tlv_usage_report_session_deletion_response_t usage_report[8];
// } ogs_pfcp_session_deletion_response_t;

// typedef struct ogs_pfcp_session_report_request_s {
//     ogs_pfcp_tlv_report_type_t report_type;
//     ogs_pfcp_tlv_downlink_data_report_t downlink_data_report;
//     ogs_pfcp_tlv_usage_report_session_report_request_t usage_report[8];
//     ogs_pfcp_tlv_error_indication_report_t error_indication_report;
//     ogs_pfcp_tlv_load_control_information_t load_control_information;
//     ogs_pfcp_tlv_overload_control_information_t overload_control_information;
//     ogs_pfcp_tlv_additional_usage_reports_information_t additional_usage_reports_information;
//     ogs_pfcp_tlv_pfcpsrreq_flags_t pfcpsrreq_flags;
//     ogs_pfcp_tlv_f_seid_t old_cp_f_seid;
// } ogs_pfcp_session_report_request_t;

// typedef struct ogs_pfcp_session_report_response_s {
//     ogs_pfcp_tlv_cause_t cause;
//     ogs_pfcp_tlv_offending_ie_t offending_ie;
//     ogs_pfcp_tlv_update_bar_pfcp_session_report_response_t update_bar;
//     ogs_pfcp_tlv_pfcpsrrsp_flags_t pfcpsrrsp_flags;
//     ogs_pfcp_tlv_f_seid_t cp_f_seid;
//     ogs_pfcp_tlv_f_teid_t n4_u_f_teid;
//     ogs_pfcp_tlv_alternative_smf_ip_address_t alternative_smf_ip_address;
// } ogs_pfcp_session_report_response_t;

#[derive(Debug)]
pub enum Message {
    ASReq(AssociationSetupRequest),
    ASResp(AssociationSetupResponse),
    AUReq(AssociationUpdateRequest),
    AUResp(AssociationUpdateResponse),
    ARReq(AssociationReleaseRequest),
    ARResp(AssociationReleaseResponse),
    //NodeReportResponse,
    SEReq(SessionEstablishmentRequest),
    SEResp(SessionEstablishmentResponse),
    SMReq(SessionModificationRequest),
    SMResp(SessionModificationResponse),
    SDReq(SessionDeletionRequest),
    SDResp(SessionDeletionResponse),
    HReq(HeartbeatRequest),
    HResp(HeartbeatResponse),
}

pub struct PfcpMessage {
   pub header: PfcpHeader,
   pub ie: Message,  
}

// int ogs_pfcp_parse_msg(ogs_pfcp_message_t *pfcp_message, ogs_pkbuf_t *pkbuf);
// ogs_pkbuf_t *ogs_pfcp_build_msg(ogs_pfcp_message_t *pfcp_message);
