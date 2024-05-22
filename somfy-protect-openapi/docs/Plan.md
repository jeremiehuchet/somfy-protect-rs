# Plan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** | Code. | 
**name** | **String** | Name. | 
**sub_name** | Option<**String**> | Sub Name. | [optional]
**description** | Option<**String**> | Description. | [optional]
**currency** | **String** | ISO-4217 Currency. | 
**tax_included** | **bool** | Tax included. | 
**prices** | [**Vec<models::PlanPrice>**](PlanPrice.md) | List of prices. | 
**display_cvr** | **bool** | Display CVR. | 
**display_emergency_network** | Option<**bool**> | Display EmergencyNetwork. | [optional]
**display_gsm_network** | Option<**bool**> | Display GsmNetwork. | [optional]
**comment** | Option<**String**> | Comment. | [optional]
**features** | [**Vec<models::PlanFeature>**](PlanFeature.md) | Features. | 
**cvr_options** | [**Vec<models::PlanCvrOption>**](PlanCvrOption.md) | CVR Options. | 
**button_type** | **String** | Button type. | 
**button_url** | Option<**String**> | Button URL. | [optional]
**more_url** | Option<**String**> | More URL. | [optional]
**popup** | Option<[**models::PlanPopup**](PlanPopup.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


