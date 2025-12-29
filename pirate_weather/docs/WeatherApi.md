# \WeatherApi

All URIs are relative to *https://api.pirateweather.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**weather**](WeatherApi.md#weather) | **GET** /forecast/{api_key}/{lat_and_long_or_time} | Make a request to Pirate Weather



## weather

> models::Weather200Response weather(api_key, lat_and_long_or_time, exclude, extend, extra_vars, lang, units, version, tmextra, icon, includes)
Make a request to Pirate Weather

Fetch a weather forecast or get historical weather data based on input latitude and longitude.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** | Pirate Weather Authentication Token. | [required] |
**lat_and_long_or_time** | **String** | A single comma-delimited string containing Latitude and Longitude information. Optionally, either a UNIX timestamp, ISO 8601 date string, or number of seconds before present can be added. | [required] |
**exclude** | Option<**String**> | Exclude some keys (hourly, minutely, daily, flags, alerts), models (nbm, hrrr, gefs), or detailed text summaries (summary) from the Pirate Weather forecast response. |  |
**extend** | Option<**String**> | Fetch the next 168 hours (7 days) worth of hourly data, instead of the next 24. |  |
**extra_vars** | Option<**String**> | Add additional variables to the response. |  |
**lang** | Option<**String**> | Returns the forecast summaries in the desired language. |  |
**units** | Option<**String**> | Return the weather forecast data in the requested unit system. |  |
**version** | Option<**i32**> | Include fields which were not part of the Dark Sky API but were introduced in API version 2. |  |
**tmextra** | Option<**i32**> | Include the full set of parameters in recent time machine requests. |  |
**icon** | Option<**String**> | Changes the icon field to return icons which aren't part of the Dark Sky icon set. |  |
**includes** | Option<**String**> | Include extra blocks that were not part of the Dark Sky API. |  |

### Return type

[**models::Weather200Response**](Weather_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

