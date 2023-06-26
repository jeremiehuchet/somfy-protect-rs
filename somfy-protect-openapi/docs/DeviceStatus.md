# DeviceStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_status_at** | Option<**String**> | Last status date (AIO, AIO+, Box, Extender, OutdoorSiren, Pir, Remote, Siren, Tag). | [optional]
**rlink_quality** | Option<**i32**> | rLink Quality (AIO, AIO+, Camera, Extender, OutdoorSiren, Pir, Remote, Siren, Tag). | [optional]
**rlink_quality_percent** | Option<**i32**> | rLink Quality Percent (AIO, AIO+, Camera, Extender, OutdoorSiren, Pir, Siren, Tag). | [optional]
**battery_level** | Option<**i32**> | Level of battery (AIO+, Box, OutdoorSiren, Pir, Remote, Siren, Tag). | [optional]
**device_lost** | Option<**bool**> | Is device lost ? (Extender, OutdoorSiren, Pir, Remote, Siren, Tag). | [optional]
**power_mode** | Option<**String**> | Power mode (AIO, AIO+, Box, Camera, Extender). | [optional][default to null]
**last_online_at** | Option<**String**> | Time of last online (AIO, AIO+, Box, Camera). | [optional]
**last_offline_at** | Option<**String**> | Time of last offline (AIO, AIO+, Box, Camera). | [optional]
**wifi_level** | Option<**i32**> | Level of wifi signal (AIO, AIO+, Box, Camera). | [optional]
**wifi_level_percent** | Option<**i32**> | Wifi Level Percent (AIO, AIO+, Box, Camera). | [optional]
**power_state** | Option<**i32**> | Power state (AIO, AIO+, Box). | [optional]
**mfa_quality_percent** | Option<**i32**> | mfa Quality Percent (AIO, AIO+, Box). | [optional]
**mfa_last_test_at** | Option<**String**> | Last myfox around test (AIO, AIO+, Box). | [optional]
**mfa_last_test_success_at** | Option<**String**> | Last myfox around test success (AIO, AIO+, Box). | [optional]
**mfa_last_online_at** | Option<**String**> | Last MFA online status (AIO, AIO+, Box). | [optional]
**mfa_last_offline_at** | Option<**String**> | Last MFA ofline status (AIO, AIO+, Box). | [optional]
**mfa_last_connected_at** | Option<**String**> | Last MFA connected status (AIO, AIO+, Box). | [optional]
**mfa_last_disconnected_at** | Option<**String**> | Last MFA disconnected status (AIO, AIO+, Box). | [optional]
**shutter_state** | Option<**String**> | Shutter state (AIO, AIO+, Camera). | [optional]
**last_shutter_opened_at** | Option<**String**> | Last shutter opened (AIO, AIO+, Camera). | [optional]
**last_shutter_closed_at** | Option<**String**> | Last shutter closed (AIO, AIO+, Camera). | [optional]
**rlink_state** | Option<**i32**> | rLink state (Remote, Siren, Tag). | [optional]
**lora_quality_percent** | Option<**i32**> | Lorawan Quality Percent (AIO+, Box). | [optional]
**lora_last_test_at** | Option<**String**> | Last LoraWan test (AIO+, Box). | [optional]
**lora_last_test_success_at** | Option<**String**> | Last LoraWan success test (AIO+, Box). | [optional]
**lora_last_online_at** | Option<**String**> | Last LoraWan online status (AIO+, Box). | [optional]
**lora_last_offline_at** | Option<**String**> | Last LoraWan offline status (AIO+, Box). | [optional]
**lora_last_connected_at** | Option<**String**> | Last LoraWan connected status (AIO+, Box). | [optional]
**lora_last_disconnected_at** | Option<**String**> | Last LoraWan disconnected status (AIO+, Box). | [optional]
**temperature** | Option<**f32**> | Temperature (Pir, OutdoorSiren). | [optional]
**temperature_at** | Option<**String**> | Time of last temperature (OutdoorSiren, Pir). | [optional]
**fsk_level** | Option<**i32**> | FSK Level (Box). | [optional]
**ble_level** | Option<**i32**> | BLE Level (Box). | [optional]
**battery_status** | Option<**i32**> | Battery status (Extender). | [optional]
**mounted_at** | Option<**String**> | Time of outdoor siren installation (OutdoorSiren). | [optional]
**battery_low** | Option<**bool**> | Is battery low (Pir). | [optional]
**battery_level_state** | Option<**String**> | Remote battery level state (Remote). | [optional]
**keep_alive** | Option<**i32**> | Remote keep alive (Remote). | [optional]
**last_check_in_state** | Option<**String**> | Last check in from remote (Remote). | [optional]
**last_check_out_state** | Option<**String**> | Last check out from remote (Remote). | [optional]
**recalibration_required** | Option<**bool**> | Is Tag recalibration required (Tag). | [optional]
**recalibrateable** | Option<**bool**> | Is Tag can be recalibrate (Tag). | [optional]
**cover_present** | Option<**bool**> | Is Tag cover present (Tag). | [optional]
**homekit_capable** | Option<**bool**> | Is Homekit capable ? (SOC). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


