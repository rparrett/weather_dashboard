# Flags

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sources** | Option<**Vec<String>**> | The models used to generate the forecast. | [optional]
**source_times** | Option<[**models::FlagsSourceTimes**](flags_sourceTimes.md)> |  | [optional]
**nearest_station** | Option<**i32**> | The distance to the nearest station (not implemented, always returns 0). | [optional]
**units** | Option<**String**> | The units used in the forecasts. | [optional]
**version** | Option<**String**> | The version of Pirate Weather used to generate the forecast. | [optional]
**source_idx** | Option<[**models::FlagsSourceIdx**](flags_sourceIDX.md)> |  | [optional]
**process_time** | Option<**i32**> | The time taken to process the request in milliseconds. Only returned when version>2. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


