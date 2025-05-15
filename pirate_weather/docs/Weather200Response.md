# Weather200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**latitude** | Option<**f64**> | The requested latitude. | [optional]
**longitude** | Option<**f64**> | The requested longitude. | [optional]
**timezone** | Option<**String**> | The timezone name for the requested location. | [optional]
**offset** | Option<**f64**> | The timezone offset in hours. | [optional]
**elevation** | Option<**i32**> | The elevation in meters of the forecast point. | [optional]
**currently** | Option<[**models::Currently**](currently.md)> |  | [optional]
**minutely** | Option<[**models::Minutely**](minutely.md)> |  | [optional]
**hourly** | Option<[**models::Hourly**](hourly.md)> |  | [optional]
**daily** | Option<[**models::Daily**](daily.md)> |  | [optional]
**alerts** | Option<[**Vec<models::AlertsInner>**](alerts_inner.md)> | A block containing any severe weather alerts for the current location. | [optional]
**flags** | Option<[**models::Flags**](flags.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


