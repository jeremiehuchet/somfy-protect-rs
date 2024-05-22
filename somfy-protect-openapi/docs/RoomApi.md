# \RoomApi

All URIs are relative to */v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_room**](RoomApi.md#create_room) | **POST** /site/{site_id}/room | Create a new room
[**delete_room**](RoomApi.md#delete_room) | **DELETE** /site/{site_id}/room/{room_id} | Delete a room
[**list_rooms**](RoomApi.md#list_rooms) | **GET** /site/{site_id}/room | List rooms
[**update_room**](RoomApi.md#update_room) | **PUT** /site/{site_id}/room/{room_id} | Update a room



## create_room

> models::RoomOutput create_room(site_id, room)
Create a new room

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**room** | [**RoomInput**](RoomInput.md) | Room | [required] |

### Return type

[**models::RoomOutput**](RoomOutput.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_room

> delete_room(site_id, room_id)
Delete a room

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**room_id** | **String** | Room identifier | [required] |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_rooms

> models::RoomList list_rooms(site_id)
List rooms

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |

### Return type

[**models::RoomList**](RoomList.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_room

> models::RoomOutput update_room(site_id, room_id)
Update a room

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**room_id** | **String** | Room identifier | [required] |

### Return type

[**models::RoomOutput**](RoomOutput.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

