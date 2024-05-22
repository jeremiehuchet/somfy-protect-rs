# \DeviceApi

All URIs are relative to */v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**answer_call**](DeviceApi.md#answer_call) | **POST** /site/{site_id}/device/{device_id}/access/answer_call | Ask WebRTC Answer Call
[**device_action**](DeviceApi.md#device_action) | **POST** /site/{site_id}/device/{device_id}/action | Run an action on a device
[**device_calibration_start**](DeviceApi.md#device_calibration_start) | **POST** /site/{site_id}/device/{device_id}/calibration | Start calibration of a device
[**device_calibration_stop**](DeviceApi.md#device_calibration_stop) | **POST** /site/{site_id}/device/{device_id}/calibration/stop | Stop calibration of a device
[**device_get_details**](DeviceApi.md#device_get_details) | **GET** /site/{site_id}/device/{device_id} | Get details of a device
[**device_homekit_installation_start**](DeviceApi.md#device_homekit_installation_start) | **POST** /site/{site_id}/device-install-homekit/{model} | Start installation of a device
[**device_homekit_updates_get**](DeviceApi.md#device_homekit_updates_get) | **GET** /site/{site_id}/device/{device_id}/homekit/qrcode | List of mandatory updates
[**device_installation_extend**](DeviceApi.md#device_installation_extend) | **POST** /site/{site_id}/device-install/{model}/extend | Extend installation duration of a device
[**device_installation_start**](DeviceApi.md#device_installation_start) | **POST** /site/{site_id}/device-install/{model} | Start installation of a device
[**device_installation_stop**](DeviceApi.md#device_installation_stop) | **DELETE** /site/{site_id}/device-install/{model} | Stop installation of a device
[**device_reset**](DeviceApi.md#device_reset) | **POST** /site/{site_id}/device/{device_id}/reset | Build an encrypted reset payload for the mobile app
[**device_reset_wifi**](DeviceApi.md#device_reset_wifi) | **POST** /site/{site_id}/device/{device_id}/reset_wifi | Reset wifi for a video device
[**device_set_details**](DeviceApi.md#device_set_details) | **PUT** /site/{site_id}/device/{device_id} | Set details of a device
[**device_sound**](DeviceApi.md#device_sound) | **POST** /site/{site_id}/device/{device_id}/sound/{sound_ref} | Play a sound on a device
[**device_uninstall_start**](DeviceApi.md#device_uninstall_start) | **POST** /site/{site_id}/device/{device_id}/uninstall | Uninstall a device previously installed on this site
[**device_uninstall_stop**](DeviceApi.md#device_uninstall_stop) | **DELETE** /site/{site_id}/device/{device_id}/uninstall | Stop uninstall process of a device
[**device_update_firmware**](DeviceApi.md#device_update_firmware) | **POST** /site/{site_id}/device/{device_id}/update | Ask firmware update for a device
[**get_compatible_devices**](DeviceApi.md#get_compatible_devices) | **GET** /site/{site_id}/device-compatible | Get list of devices compatible for installation on site
[**get_installed_devices**](DeviceApi.md#get_installed_devices) | **GET** /site/{site_id}/device | Get list of installed devices on site
[**get_site_lorawan_device_test**](DeviceApi.md#get_site_lorawan_device_test) | **POST** /site/{site_id}/device/lorawan/test | LoRawan device trame test
[**gsm_backup_sms_test**](DeviceApi.md#gsm_backup_sms_test) | **POST** /site/{site_id}/device/backup-gsm/test | Backup GSM device test
[**mode_hkp_get_details**](DeviceApi.md#mode_hkp_get_details) | **GET** /site/{site_id}/mode/{mode_name} | Get details of a mode
[**mode_hkp_update**](DeviceApi.md#mode_hkp_update) | **PUT** /site/{site_id}/mode/{mode_name} | Update HKP mode settings



## answer_call

> models::Job answer_call(site_id, device_id, session_id)
Ask WebRTC Answer Call

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**device_id** | **String** | Device identifier | [required] |
**session_id** | [**WebRtcSessionInput**](WebRtcSessionInput.md) | WebRTC Session identifier | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## device_action

> models::JobCollection device_action(site_id, device_id, device_details)
Run an action on a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**device_id** | **String** | Device identifier or 'all' | [required] |
**device_details** | [**DeviceAction**](DeviceAction.md) | Device action | [required] |

### Return type

[**models::JobCollection**](JobCollection.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## device_calibration_start

> models::Job device_calibration_start(site_id, device_id)
Start calibration of a device

Start calibration of a device (only for IntelliTAG)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**device_id** | **String** | Device identifier | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## device_calibration_stop

> models::Job device_calibration_stop(site_id, device_id)
Stop calibration of a device

Stop calibration of a device (only for IntelliTAG)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**device_id** | **String** | Device identifier | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## device_get_details

> models::DeviceOutput device_get_details(site_id, device_id)
Get details of a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**device_id** | **String** | Device identifier | [required] |

### Return type

[**models::DeviceOutput**](DeviceOutput.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## device_homekit_installation_start

> models::Job device_homekit_installation_start(site_id, model, install_options)
Start installation of a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**model** | **String** | Model of device | [required] |
**install_options** | Option<[**InstallOptionsHomeKit**](InstallOptionsHomeKit.md)> | Options for installing the device |  |

### Return type

[**models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## device_homekit_updates_get

> device_homekit_updates_get(site_id, device_id)
List of mandatory updates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**device_id** | **String** | Device identifier | [required] |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## device_installation_extend

> models::Job device_installation_extend(site_id, model, extend_options)
Extend installation duration of a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**model** | **String** | Model of device | [required] |
**extend_options** | [**ExtendOptions**](ExtendOptions.md) | Options for extending installation | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## device_installation_start

> models::Job device_installation_start(site_id, model, install_options)
Start installation of a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**model** | **String** | Model of device | [required] |
**install_options** | Option<[**InstallOptions**](InstallOptions.md)> | Options for installing the device |  |

### Return type

[**models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## device_installation_stop

> models::Job device_installation_stop(site_id, model)
Stop installation of a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**model** | **String** | Model of device | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## device_reset

> models::Reset device_reset(site_id, device_id, wifi_credentials)
Build an encrypted reset payload for the mobile app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**device_id** | **String** | Device identifier | [required] |
**wifi_credentials** | Option<[**WifiCredentials**](WifiCredentials.md)> | Wifi SSID and password |  |

### Return type

[**models::Reset**](Reset.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## device_reset_wifi

> models::Reset device_reset_wifi(site_id, device_id)
Reset wifi for a video device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**device_id** | **String** | Device identifier | [required] |

### Return type

[**models::Reset**](Reset.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## device_set_details

> models::Job device_set_details(site_id, device_id, device_details)
Set details of a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**device_id** | **String** | Device identifier | [required] |
**device_details** | [**DeviceInput**](DeviceInput.md) | Device details | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## device_sound

> models::Job device_sound(site_id, device_id, sound_ref)
Play a sound on a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**device_id** | **String** | Device identifier | [required] |
**sound_ref** | **String** | Sound to play | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## device_uninstall_start

> models::Job device_uninstall_start(site_id, device_id)
Uninstall a device previously installed on this site

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**device_id** | **String** | Device identifier | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## device_uninstall_stop

> models::Job device_uninstall_stop(site_id, device_id)
Stop uninstall process of a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**device_id** | **String** | Device identifier | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## device_update_firmware

> models::Job device_update_firmware(site_id, device_id)
Ask firmware update for a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**device_id** | **String** | Device identifier | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_compatible_devices

> models::DeviceCompatibilityCollection get_compatible_devices(site_id)
Get list of devices compatible for installation on site

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |

### Return type

[**models::DeviceCompatibilityCollection**](DeviceCompatibilityCollection.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_installed_devices

> models::DeviceCollection get_installed_devices(site_id)
Get list of installed devices on site

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |

### Return type

[**models::DeviceCollection**](DeviceCollection.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_site_lorawan_device_test

> get_site_lorawan_device_test(site_id)
LoRawan device trame test

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gsm_backup_sms_test

> gsm_backup_sms_test(site_id)
Backup GSM device test

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mode_hkp_get_details

> models::HkpMode mode_hkp_get_details(site_id, mode_name)
Get details of a mode

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**mode_name** | **String** | Mode name | [required] |

### Return type

[**models::HkpMode**](HkpMode.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mode_hkp_update

> models::Job mode_hkp_update(site_id, mode_name)
Update HKP mode settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**mode_name** | **String** | Mode name | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

