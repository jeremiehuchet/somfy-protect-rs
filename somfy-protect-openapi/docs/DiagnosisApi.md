# \DiagnosisApi

All URIs are relative to */v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**global_testing_diagnosis**](DiagnosisApi.md#global_testing_diagnosis) | **POST** /site/{site_id}/device/diagnosis | Start the global testing of all devices for a site
[**global_testing_diagnosis_extend**](DiagnosisApi.md#global_testing_diagnosis_extend) | **POST** /site/{site_id}/device/diagnosis/extend | Extends the global testing of all devices for a site
[**global_testing_diagnosis_stop**](DiagnosisApi.md#global_testing_diagnosis_stop) | **POST** /site/{site_id}/device/diagnosis/stop | Stops the global testing of all devices for a site



## global_testing_diagnosis

> crate::models::Job global_testing_diagnosis(site_id)
Start the global testing of all devices for a site

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


## global_testing_diagnosis_extend

> crate::models::Job global_testing_diagnosis_extend(site_id)
Extends the global testing of all devices for a site

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


## global_testing_diagnosis_stop

> crate::models::Job global_testing_diagnosis_stop(site_id)
Stops the global testing of all devices for a site

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

