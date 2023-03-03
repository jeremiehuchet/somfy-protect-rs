# DeviceSettingsGlobal

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | Option<**String**> | User associated to this device (for remote). | [optional]
**wifi_ssid** | Option<**String**> | Wifi SSID (for plug and camera). | [optional]
**detection_enabled** | Option<**bool**> | Enable detection (for camera). | [optional]
**sound_enabled** | Option<**bool**> | Enable sound (for siren and keypad). | [optional]
**light_enabled** | Option<**bool**> | Enable lights (for siren and pir). | [optional]
**test_reminder_enabled** | Option<**bool**> | Test reminder (for smoke detector). | [optional]
**extended_alarm_enabled** | Option<**bool**> | Extended alarm (for smoke detector). | [optional]
**code_required_to_arm** | Option<**bool**> | Keypad security parameter. | [optional]
**sensitivity** | Option<**i32**> | Sensitivity (for IntelliTAG, Camera). | [optional]
**sensitivity_level** | Option<**String**> | Sensitivity level (for PIR). | [optional]
**support_type** | Option<**String**> | Support type (for IntelliTAG). | [optional]
**night_vision** | Option<**String**> | Night vision (for camera). | [optional]
**led_enabled** | Option<[**serde_json::Value**](.md)> | LED Enabled (for camera). | [optional]
**hdvideo_enabled** | Option<**bool**> | HD Video Enabled (for camera). | [optional]
**video_mode** | Option<**String**> | Video modes (for AIO). | [optional]
**detection_regions** | Option<**bool**> | Detection Regions (for camera). | [optional]
**enabled** | Option<**bool**> | Device Enabled (for remote). | [optional]
**prealarm_enabled** | Option<**bool**> | Enable-Disable pre-alarm (for IntelliTAG, Camera and Pir). | [optional]
**auto_protect_enabled** | Option<**bool**> | Enable-Disable auto-protection (for Keypad, Pir and Siren). | [optional]
**threshold_acc** | Option<**i32**> | Threshold acceleration (for Outdoor Siren). | [optional]
**smoke_alarm_listening_enabled** | Option<**bool**> | Enable-Disable smoke alarm listening (for Camera). | [optional]
**smoke_alarm_state** | Option<**String**> | Define smoke alarm setting (for SomfyOne and SomfyOne+). | [optional]
**night_mode_enabled** | Option<**bool**> | Enable-Disable use in night mode (for TAG, Camera and Pir). | [optional]
**sound_recording_enabled** | Option<**bool**> | Enable-Disable sound recording (for MyfoxCamera). | [optional]
**siren_on_camera_detection_disabled** | Option<**bool**> | Enable siren on camera detection (for AIO). | [optional]
**siren_disabled** | Option<**bool**> | Enable outdoor camera's siren. | [optional]
**auto_rotate_enabled** | Option<**bool**> | Enable-Disable video rotation (for OutdoorCamera). | [optional]
**auto_rotate_orientation** | Option<**bool**> | OutdoorCamera auto rotate . | [optional]
**smart_alarm_duration** | Option<**i32**> | Smart alarm duration (for OutdoorCamera). | [optional]
**sound_alarm** | Option<**bool**> | Enable-Disable siren sound pre-alarm (true:Siren, false:Voice) (for OutdoorCamera). | [optional]
**lighting_trigger** | Option<**String**> | Trigger for lighting (for OutdoorCamera). | [optional]
**ambient_light_threshold** | Option<**i32**> | Ambient light threshold (for OutdoorCamera). | [optional]
**lighting_duration** | Option<**i32**> | Lighting duration when detection (for OutdoorCamera). | [optional]
**lighting_wired** | Option<**bool**> | Is lighting is wired to device ? (for OutdoorCamera). | [optional]
**voice_alarm_locale** | Option<**String**> | Locale voice for pre-alarm ? (for OutdoorCamera). | [optional]
**hdr_enabled** | Option<**bool**> | HDR video (for AllInOne, AllInOne+, IndoorCamera, OutdoorCamera). | [optional]
**human_detect_enabled** | Option<**bool**> | Human Detect Enabled (for OutdoorCamera). | [optional]
**latch_wired** | Option<[**serde_json::Value**](.md)> | Is latch wired? | [optional]
**gate_wired** | Option<[**serde_json::Value**](.md)> | Is gate wired? | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


