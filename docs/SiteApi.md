# \SiteApi

All URIs are relative to */v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_security_level**](SiteApi.md#change_security_level) | **PUT** /site/{site_id}/security | Change security level
[**create_site**](SiteApi.md#create_site) | **POST** /site | Creates a new site
[**delete_site**](SiteApi.md#delete_site) | **DELETE** /site/{site_id} | Delete a site
[**device_updates_get**](SiteApi.md#device_updates_get) | **GET** /site/{site_id}/device/{device_id}/update | List of mandatory updates
[**get_site**](SiteApi.md#get_site) | **GET** /site/{site_id} | Get a specific site for the current user
[**get_site_lorawan_cover_test**](SiteApi.md#get_site_lorawan_cover_test) | **GET** /site/{site_id}/lorawan/covertest | LoRawan Cover test on site
[**site_alarm_stop**](SiteApi.md#site_alarm_stop) | **PUT** /site/{site_id}/alarm/stop | Stop current site alarm
[**site_domestic_alarm_stop**](SiteApi.md#site_domestic_alarm_stop) | **PUT** /site/{site_id}/domestic-alarm/{alarm_id}/stop | Stop domestic alarm
[**site_get_list**](SiteApi.md#site_get_list) | **GET** /site | List available sites for the current user
[**site_history**](SiteApi.md#site_history) | **GET** /site/{site_id}/history | Get filtered and sorted site events
[**site_manual_alarm_trigger**](SiteApi.md#site_manual_alarm_trigger) | **PUT** /site/{site_id}/alarm/trigger_manual_alarm | Trigger site alarm
[**site_panic_start**](SiteApi.md#site_panic_start) | **POST** /site/{site_id}/panic | Put site in panic mode
[**site_set_privacy**](SiteApi.md#site_set_privacy) | **PUT** /site/{site_id}/privacy | Set the privacy state
[**update_site**](SiteApi.md#update_site) | **PUT** /site/{site_id} | update a site



## change_security_level

> crate::models::Job change_security_level(site_id, security_level)
Change security level

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**security_level** | [**Security**](Security.md) | Security object | [required] |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_site

> crate::models::SiteOutput create_site(site)
Creates a new site

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site** | [**SiteInput**](SiteInput.md) | Site object | [required] |

### Return type

[**crate::models::SiteOutput**](SiteOutput.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_site

> delete_site(site_id)
Delete a site

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


## device_updates_get

> device_updates_get(site_id, device_id)
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


## get_site

> crate::models::SiteOutput get_site(site_id)
Get a specific site for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |

### Return type

[**crate::models::SiteOutput**](SiteOutput.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_site_lorawan_cover_test

> crate::models::LoraWanCover get_site_lorawan_cover_test(site_id)
LoRawan Cover test on site

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |

### Return type

[**crate::models::LoraWanCover**](LoraWanCover.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_alarm_stop

> crate::models::Job site_alarm_stop(site_id)
Stop current site alarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_domestic_alarm_stop

> site_domestic_alarm_stop(site_id, alarm_id)
Stop domestic alarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**alarm_id** | **String** | Domestic alarm identifier | [required] |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_get_list

> crate::models::SiteCollection site_get_list()
List available sites for the current user

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SiteCollection**](SiteCollection.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_history

> crate::models::SiteEventCollection site_history(site_id, date_min, date_max, message_type, order, limit, start_after, end_before)
Get filtered and sorted site events

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier. | [required] |
**date_min** | Option<**String**> | ISO 8601 UTC timestamp (example: '2014-07-14T07:45:00.000Z'). |  |
**date_max** | Option<**String**> | ISO 8601 UTC timestamp (example: '2014-07-14T07:45:00.000Z'). |  |
**message_type** | Option<**String**> | Comma separated list of event types (example: 'securityAlarm,securityLevel'). |  |
**order** | Option<**String**> | Set results order. '1' for ascending, '-1' for descending. Default: descending. |  |
**limit** | Option<**i32**> | Limit of results. Default: 50. Max: 100. |  |
**start_after** | Option<**String**> | ISO 8601 microseconds timestamp to get next page (based on 'occurred_at' value). |  |
**end_before** | Option<**String**> | ISO 8601 microseconds timestamp to get previous page (based on 'occurred_at' value). |  |

### Return type

[**crate::models::SiteEventCollection**](SiteEventCollection.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_manual_alarm_trigger

> crate::models::Job site_manual_alarm_trigger(site_id)
Trigger site alarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_panic_start

> crate::models::Job site_panic_start(site_id, panic)
Put site in panic mode

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**panic** | [**Panic**](Panic.md) | Panic mode details | [required] |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_set_privacy

> crate::models::Job site_set_privacy(site_id, privacy)
Set the privacy state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**privacy** | [**SitePrivacy**](SitePrivacy.md) | Privacy details | [required] |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_site

> update_site(site_id, site)
update a site

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**site** | [**SiteInput**](SiteInput.md) | Site object | [required] |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

