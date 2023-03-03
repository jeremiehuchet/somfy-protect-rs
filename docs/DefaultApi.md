# \DefaultApi

All URIs are relative to */v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_info**](DefaultApi.md#get_user_info) | **GET** /user/{user_id} | Get user information
[**site_user_token_action**](DefaultApi.md#site_user_token_action) | **GET** /site/{site_id}/user/{user_id}/token_action/{token} | Send a token action to a link between a site and an user
[**update_site_user_location**](DefaultApi.md#update_site_user_location) | **PUT** /site/{site_id}/user/{user_id}/location | Send location information to a link between a site and an user



## get_user_info

> crate::models::UserOutputWithSites get_user_info(user_id)
Get user information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User identifier. Current logged user if not provided | [required] |

### Return type

[**crate::models::UserOutputWithSites**](UserOutputWithSites.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_user_token_action

> site_user_token_action(site_id, user_id, token)
Send a token action to a link between a site and an user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**user_id** | **String** | User identifier | [required] |
**token** | **String** | Token | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_site_user_location

> crate::models::LocationOutput update_site_user_location(site_id, user_id, location)
Send location information to a link between a site and an user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**user_id** | **String** | User identifier | [required] |
**location** | [**LocationInput**](LocationInput.md) | Details to update geoFencing or presence | [required] |

### Return type

[**crate::models::LocationOutput**](LocationOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

