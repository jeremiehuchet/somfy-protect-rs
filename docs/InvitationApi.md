# \InvitationApi

All URIs are relative to */v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**site_invitation_answer**](InvitationApi.md#site_invitation_answer) | **POST** /site/invitation | Answer to an invitation
[**site_invitation_send**](InvitationApi.md#site_invitation_send) | **POST** /site/{site_id}/invitation | Send an invitation for using a site



## site_invitation_answer

> site_invitation_answer(security_token, answer)
Answer to an invitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**security_token** | **String** | Security token | [required] |
**answer** | [**SiteInvitationAnswer**](SiteInvitationAnswer.md) | Invitation answer details | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_invitation_send

> crate::models::UserOutput site_invitation_send(site_id, invitation)
Send an invitation for using a site

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**invitation** | [**SiteInvitationRequest**](SiteInvitationRequest.md) | Invitation details | [required] |

### Return type

[**crate::models::UserOutput**](UserOutput.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

