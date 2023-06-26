# BoxStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**power_state** | Option<**i32**> | Power state. | [optional]
**battery_level** | Option<**i32**> | Battery level. | [optional]
**fsk_level** | Option<**i32**> | FSK Level. | [optional]
**ble_level** | Option<**i32**> | BLE Level. | [optional]
**wifi_level** | Option<**i32**> | Wifi Level. | [optional]
**wifi_level_percent** | Option<**i32**> | Wifi Level Percent. | [optional]
**last_online_at** | Option<**String**> | Last online status. | [optional]
**last_offline_at** | Option<**String**> | Last ofline status. | [optional]
**power_mode** | Option<[**serde_json::Value**](.md)> |  | [optional]
**device_lost** | Option<**bool**> | Is device lost ? (Pir, Siren, OutdoorSiren, Remote, Tag, Extender). | [optional]
**last_status_at** | Option<**String**> | Last status date (Box, AIO, AIO+, Pir, Siren, OutdoorSiren, Remote, Tag, Extender). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


