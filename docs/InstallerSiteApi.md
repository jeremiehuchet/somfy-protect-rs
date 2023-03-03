# \InstallerSiteApi

All URIs are relative to */v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**site_installer_add**](InstallerSiteApi.md#site_installer_add) | **POST** /site/{site_id}/installer | Add a link between a site and the installer
[**site_installer_delete**](InstallerSiteApi.md#site_installer_delete) | **DELETE** /site/{site_id}/installer/{user_id} | Delete the link between a site and the installer



## site_installer_add

> crate::models::UserOutput site_installer_add(site_id, access_token, email)
Add a link between a site and the installer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**access_token** | **String** | Access token | [required] |
**email** | [**SiteInstallerAddRequest**](SiteInstallerAddRequest.md) | User email | [required] |

### Return type

[**crate::models::UserOutput**](UserOutput.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_installer_delete

> site_installer_delete(site_id, user_id, access_token, password)
Delete the link between a site and the installer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**user_id** | **String** | User identifier | [required] |
**access_token** | **String** | Access token | [required] |
**password** | [**SiteInstallerDeleteRequest**](SiteInstallerDeleteRequest.md) | Password to check. | [required] |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

