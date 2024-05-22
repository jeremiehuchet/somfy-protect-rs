# \PhotoApi

All URIs are relative to */v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_photo**](PhotoApi.md#get_photo) | **GET** /photo/{photo_id} | Get a photo uploader by user
[**upload_photo**](PhotoApi.md#upload_photo) | **POST** /photo | Upload a new photo



## get_photo

> get_photo(photo_id)
Get a photo uploader by user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**photo_id** | **String** | Photo identifier | [required] |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_photo

> models::UploadedPhoto upload_photo(photo)
Upload a new photo

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**photo** | [**NewPhoto**](NewPhoto.md) | Photo to upload | [required] |

### Return type

[**models::UploadedPhoto**](UploadedPhoto.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

