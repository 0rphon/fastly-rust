# KvStoreApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**create_store**](KvStoreApi.md#create_store) | **POST** /resources/stores/kv | Create a KV store.
[**delete_store**](KvStoreApi.md#delete_store) | **DELETE** /resources/stores/kv/{store_id} | Delete a KV store.
[**get_store**](KvStoreApi.md#get_store) | **GET** /resources/stores/kv/{store_id} | Describe a KV store.
[**get_stores**](KvStoreApi.md#get_stores) | **GET** /resources/stores/kv | List KV stores.



## create_store

Create a new KV store.

```rust
let cfg = &Configuration::default();
let params = CreateStoreParams {
    // parameters
};
create_store(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location** | Option\<**String**> |  |  |
**store** | Option\<[**Store**](Store.md)> |  |  |

### Return type

[**crate::models::StoreResponse**](StoreResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_store

A KV store must be empty before it can be deleted.  Deleting a KV store that still contains keys will result in a `409` (Conflict).

```rust
let cfg = &Configuration::default();
let params = DeleteStoreParams {
    // parameters
};
delete_store(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_store

Get a KV store by ID.

```rust
let cfg = &Configuration::default();
let params = GetStoreParams {
    // parameters
};
get_store(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |

### Return type

[**crate::models::StoreResponse**](StoreResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_stores

Get all stores for a given customer.

```rust
let cfg = &Configuration::default();
let params = GetStoresParams {
    // parameters
};
get_stores(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option\<**String**> |  |  |
**limit** | Option\<**i32**> |  |  |[default to 1000]

### Return type

[**crate::models::InlineResponse2003**](InlineResponse2003.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

