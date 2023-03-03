# \UserApi

All URIs are relative to */v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user**](UserApi.md#create_user) | **POST** /user/register | Create a user account
[**delete_sp_data**](UserApi.md#delete_sp_data) | **DELETE** /user/gdpr-delete | Delete SP data for the current user
[**send_password_token**](UserApi.md#send_password_token) | **POST** /user/password/send | Send password change token via email to user
[**update_user**](UserApi.md#update_user) | **PUT** /user/{user_id} | Update an user
[**update_user_with_token**](UserApi.md#update_user_with_token) | **PUT** /user/secure | Update a user with a secure token sent by email
[**user_guest_create**](UserApi.md#user_guest_create) | **POST** /user-guest | Creates a guest user account
[**user_password_set**](UserApi.md#user_password_set) | **PUT** /user/{user_id}/password | Update user password
[**validate_email**](UserApi.md#validate_email) | **GET** /user/validate-email/{email} | Is email valid



## create_user

> crate::models::UserOutput create_user(site_user_link)
Create a user account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_user_link** | [**UserInput**](UserInput.md) | Details of link between site and user | [required] |

### Return type

[**crate::models::UserOutput**](UserOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sp_data

> delete_sp_data(access_token, client_id, password, delete)
Delete SP data for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_token** | **String** | Access token | [required] |
**client_id** | **String** | Current OAuth Client ID (Example : 123456_132456) | [required] |
**password** | **serde_json::Value** | Password to check. | [required] |
**delete** | Option<**i32**> | (1) -> Delete user and SSO link<br>(2) -> Make user GDPR compliant (default) |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_password_token

> send_password_token(username)
Send password change token via email to user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | Email of user that needs password change | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> update_user(user_id, user)
Update an user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User identifier. Current logged user if not provided | [required] |
**user** | [**UserInputForUpdate**](UserInputForUpdate.md) | User object | [required] |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_with_token

> update_user_with_token(security_token, user)
Update a user with a secure token sent by email

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**security_token** | **String** | secure token | [required] |
**user** | [**UserInputWithSecureToken**](UserInputWithSecureToken.md) | User details | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_guest_create

> crate::models::UserGuestOutput user_guest_create(site_user_link)
Creates a guest user account

Creates a new user without credentials, associated to the site and available for fob / user association

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_user_link** | [**UserGuestInput**](UserGuestInput.md) | Details of link between site and user | [required] |

### Return type

[**crate::models::UserGuestOutput**](UserGuestOutput.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_password_set

> user_password_set(user_id, password)
Update user password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User identifier | [required] |
**password** | [**Password**](Password.md) | Password details | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_email

> crate::models::ValidateEmail200Response validate_email(email)
Is email valid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email to validate. | [required] |

### Return type

[**crate::models::ValidateEmail200Response**](validateEmail_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

