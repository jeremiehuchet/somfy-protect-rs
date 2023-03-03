# \UserSiteApi

All URIs are relative to */v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**site_user_action**](UserSiteApi.md#site_user_action) | **POST** /site/{site_id}/user/{user_id}/action | Signal an action to a link between a site and an user
[**site_user_code_add_or_update**](UserSiteApi.md#site_user_code_add_or_update) | **PUT** /site/{site_id}/user/{user_id}/code | add or update personal code
[**site_user_code_delete**](UserSiteApi.md#site_user_code_delete) | **DELETE** /site/{site_id}/user/{user_id}/code | delete personal code
[**site_user_delete**](UserSiteApi.md#site_user_delete) | **DELETE** /site/{site_id}/user/{user_id} | Delete the link between a site and an user
[**site_user_get**](UserSiteApi.md#site_user_get) | **GET** /site/{site_id}/user/{user_id} | Get the link between a site and an user
[**site_user_update**](UserSiteApi.md#site_user_update) | **PUT** /site/{site_id}/user/{user_id} | Update the link between a site and an user
[**site_users_get_list**](UserSiteApi.md#site_users_get_list) | **GET** /site/{site_id}/user | Retrieves the list of users of the site



## site_user_action

> site_user_action(site_id, user_id, action)
Signal an action to a link between a site and an user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**user_id** | **String** | User identifier | [required] |
**action** | [**UserSiteAction**](UserSiteAction.md) | Details of an action on a link between site and user | [required] |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_user_code_add_or_update

> site_user_code_add_or_update(site_id, user_id, code)
add or update personal code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**user_id** | **String** | User identifier | [required] |
**code** | **serde_json::Value** | The individual code | [required] |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_user_code_delete

> site_user_code_delete(site_id, user_id)
delete personal code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**user_id** | **String** | User identifier | [required] |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_user_delete

> site_user_delete(site_id, user_id)
Delete the link between a site and an user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**user_id** | **String** | User identifier | [required] |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_user_get

> site_user_get(site_id, user_id)
Get the link between a site and an user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**user_id** | **String** | User identifier | [required] |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_user_update

> crate::models::SiteUserLinkOutput site_user_update(site_id, user_id, site_user_link)
Update the link between a site and an user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**user_id** | **String** | User identifier | [required] |
**site_user_link** | [**SiteUserLinkInput**](SiteUserLinkInput.md) | Details of link between site and user | [required] |

### Return type

[**crate::models::SiteUserLinkOutput**](SiteUserLinkOutput.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_users_get_list

> crate::models::UserCollection site_users_get_list(site_id)
Retrieves the list of users of the site

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |

### Return type

[**crate::models::UserCollection**](UserCollection.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

