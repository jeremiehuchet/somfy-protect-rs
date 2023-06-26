# \MobileApi

All URIs are relative to */v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_mobile_create**](MobileApi.md#user_mobile_create) | **POST** /user/{user_id}/mobile | Provide details of mobile phone for a user
[**user_mobile_delete**](MobileApi.md#user_mobile_delete) | **DELETE** /user/{user_id}/mobile/{phone_id} | Remove association between an user and a mobile



## user_mobile_create

> crate::models::Mobile user_mobile_create(user_id, mobile)
Provide details of mobile phone for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User identifier | [required] |
**mobile** | [**Mobile**](Mobile.md) | Mobile details | [required] |

### Return type

[**crate::models::Mobile**](Mobile.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_mobile_delete

> user_mobile_delete(user_id, phone_id)
Remove association between an user and a mobile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User identifier | [required] |
**phone_id** | **String** | Identifier of the phone | [required] |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

