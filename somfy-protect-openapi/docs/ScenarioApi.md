# \ScenarioApi

All URIs are relative to */v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**site_scenario_create**](ScenarioApi.md#site_scenario_create) | **POST** /site/{site_id}/scenario | Create a new scenario on a specific site
[**site_scenario_delete**](ScenarioApi.md#site_scenario_delete) | **DELETE** /site/{site_id}/scenario/{scenario_id} | Delete a scenario
[**site_scenario_get_list**](ScenarioApi.md#site_scenario_get_list) | **GET** /site/{site_id}/scenario | Get list of scenarios
[**site_scenario_update**](ScenarioApi.md#site_scenario_update) | **PUT** /site/{site_id}/scenario/{scenario_id} | Update a scenario



## site_scenario_create

> models::Job site_scenario_create(site_id, scenario)
Create a new scenario on a specific site

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**scenario** | [**ScenarioInput**](ScenarioInput.md) | New scenario to create | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_scenario_delete

> models::Job site_scenario_delete(site_id, scenario_id)
Delete a scenario

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**scenario_id** | **String** | Scenario identifier | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_scenario_get_list

> models::ScenarioCollection site_scenario_get_list(site_id)
Get list of scenarios

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |

### Return type

[**models::ScenarioCollection**](ScenarioCollection.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_scenario_update

> models::Job site_scenario_update(site_id, scenario_id, scenario)
Update a scenario

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**scenario_id** | **String** | Scenario identifier | [required] |
**scenario** | [**ScenarioInput**](ScenarioInput.md) | New scenario data | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

