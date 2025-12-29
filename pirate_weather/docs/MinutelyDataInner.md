# MinutelyDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**time** | Option<**i32**> | The time of the data point in UNIX format. | [optional]
**precip_intensity** | Option<**f64**> | The intensity of precipitation. | [optional]
**precip_probability** | Option<**f64**> | The probability of precipitation. | [optional]
**precip_intensity_error** | Option<**f64**> | The standard deviation of the precipitation intensity. | [optional]
**precip_type** | Option<**String**> | The type of precipitation occurring. | [optional]
**rain_intensity** | Option<**f64**> | The intensity of rain precipitation. Only returned when version>1. | [optional]
**snow_intensity** | Option<**f64**> | The intensity of snow precipitation. Only returned when version>1. | [optional]
**ice_intensity** | Option<**f64**> | The intensity of ice precipitation. Only returned when version>1. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


