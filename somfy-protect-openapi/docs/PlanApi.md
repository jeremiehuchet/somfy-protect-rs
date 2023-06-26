# \PlanApi

All URIs are relative to */v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**site_available_plan_list**](PlanApi.md#site_available_plan_list) | **GET** /site/{site_id}/plan/available | Get available plans
[**site_cancellationi_immediately**](PlanApi.md#site_cancellationi_immediately) | **POST** /site/{site_id}/plan/unsubscribe/immediately | Cancellation imma
[**site_checkout**](PlanApi.md#site_checkout) | **POST** /site/{site_id}/plan/checkout | Send cart and get checkout result
[**site_current_plan_list**](PlanApi.md#site_current_plan_list) | **GET** /site/{site_id}/plan/current | Get current plans
[**site_payment_method_update**](PlanApi.md#site_payment_method_update) | **POST** /site/{site_id}/payment-method/update | Get an URL to update the payment method
[**site_remove_scheduled_cancellation**](PlanApi.md#site_remove_scheduled_cancellation) | **DELETE** /site/{site_id}/plan/unsubscribe | Remove scheduled cancellation



## site_available_plan_list

> crate::models::PlanAvailable site_available_plan_list(site_id)
Get available plans

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |

### Return type

[**crate::models::PlanAvailable**](PlanAvailable.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_cancellationi_immediately

> site_cancellationi_immediately(site_id, unsubscribe_input)
Cancellation imma

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**unsubscribe_input** | Option<[**ServiceUnsubscribeInput**](ServiceUnsubscribeInput.md)> | Send email on unsubscribe |  |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_checkout

> crate::models::ServiceCheckoutOutput site_checkout(site_id, checkout_input)
Send cart and get checkout result

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**checkout_input** | [**ServiceCheckoutInput**](ServiceCheckoutInput.md) | Checkout data | [required] |

### Return type

[**crate::models::ServiceCheckoutOutput**](ServiceCheckoutOutput.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_current_plan_list

> crate::models::PlanCurrent site_current_plan_list(site_id)
Get current plans

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |

### Return type

[**crate::models::PlanCurrent**](PlanCurrent.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_payment_method_update

> crate::models::ServiceCheckoutOutput site_payment_method_update(site_id)
Get an URL to update the payment method

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |

### Return type

[**crate::models::ServiceCheckoutOutput**](ServiceCheckoutOutput.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## site_remove_scheduled_cancellation

> site_remove_scheduled_cancellation(site_id)
Remove scheduled cancellation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

