# \MiscApi

All URIs are relative to */v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_cloud_info**](MiscApi.md#get_cloud_info) | **GET** /info | Get cloud version and enabled features
[**get_dictionary**](MiscApi.md#get_dictionary) | **GET** /dictionary/{dictionary_id}/{locale} | Get a dictionary file for translations
[**get_user_job**](MiscApi.md#get_user_job) | **GET** /job/{job_id} | Get user job
[**user_feedback_create**](MiscApi.md#user_feedback_create) | **POST** /user/{user_id}/feedback | Send feedback



## get_cloud_info

> get_cloud_info()
Get cloud version and enabled features

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dictionary

> get_dictionary(dictionary_id, locale)
Get a dictionary file for translations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dictionary_id** | **String** | Dictionary identifier | [required] |
**locale** | **String** | Dictionary locale | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_job

> models::Job get_user_job(job_id)
Get user job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Job uniquer identifier | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_feedback_create

> user_feedback_create(user_id, feedback)
Send feedback

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User identifier | [required] |
**feedback** | [**Feedback**](Feedback.md) | Feedback | [required] |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

