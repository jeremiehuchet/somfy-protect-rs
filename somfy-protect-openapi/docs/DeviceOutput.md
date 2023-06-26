# DeviceOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device_id** | **String** | The device unique identifier. | 
**box_id** | Option<**String**> | The device box identifier. | [optional]
**site_id** | **String** | The device site identifier. | 
**version** | Option<**String**> | The device firmware version. | [optional]
**mac** | Option<**String**> | The device MAC address. | [optional]
**device_definition** | [**crate::models::DeviceDefinition**](DeviceDefinition.md) |  | 
**status** | [**crate::models::DeviceStatus**](DeviceStatus.md) |  | 
**into_subscription** | Option<**bool**> | Device is into a subscription (uninstall will be forbidden). | [optional]
**push_to_talk_available** | Option<**bool**> | Camera has push to talk available. | [optional]
**provider_id** | Option<**String**> | Provider ID. | [optional]
**update_available** | Option<**bool**> |  | [optional]
**master** | Option<**bool**> | Is it master device. | [optional]
**somfy_one_type** | Option<**String**> | Somfy One Type. | [optional]
**video_backend** | Option<**String**> | Video Backend. | [optional]
**diagnosis** | Option<[**serde_json::Value**](.md)> | The device diagnosis. | [optional]
**is_full_gsm** | Option<**bool**> |  | [optional]
**label** | Option<**String**> | The device label. | [optional]
**room_id** | Option<**String**> | The room id. | [optional]
**settings** | Option<[**crate::models::DeviceSettings**](DeviceSettings.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


