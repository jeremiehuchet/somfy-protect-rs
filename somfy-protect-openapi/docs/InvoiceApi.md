# \InvoiceApi

All URIs are relative to */v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**invoice_list**](InvoiceApi.md#invoice_list) | **GET** /site/{site_id}/invoice | Get invoices list
[**invoice_pdf**](InvoiceApi.md#invoice_pdf) | **GET** /site/{site_id}/invoice/{invoice_id} | Get PDF invoice



## invoice_list

> models::ServiceInvoiceCollection invoice_list(site_id, date_min, date_max, order, limit, start_after, end_before)
Get invoices list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**date_min** | Option<**String**> | ISO 8601 UTC timestamp (example: '2014-07-14T07:45:00.000Z'). |  |
**date_max** | Option<**String**> | ISO 8601 UTC timestamp (example: '2014-07-14T07:45:00.000Z'). |  |
**order** | Option<**String**> | Set results order. '1' for ascending, '-1' for descending. Default: descending. |  |
**limit** | Option<**i32**> | Limit of results. Default: 50. Max: 200. |  |
**start_after** | Option<**String**> | ISO 8601 microseconds timestamp to get next page (based on 'paid_on' value). |  |
**end_before** | Option<**String**> | ISO 8601 microseconds timestamp to get previous page (based on 'paid_on' value). |  |

### Return type

[**models::ServiceInvoiceCollection**](ServiceInvoiceCollection.md)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoice_pdf

> invoice_pdf(site_id, invoice_id)
Get PDF invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site identifier | [required] |
**invoice_id** | **String** | Invoice identifier | [required] |

### Return type

 (empty response body)

### Authorization

[myfox_auth](../README.md#myfox_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

