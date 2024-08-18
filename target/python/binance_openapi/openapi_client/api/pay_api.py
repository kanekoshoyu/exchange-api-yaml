# coding: utf-8

"""
    Binance Public Spot API

    OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)

    The version of the OpenAPI document: 1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501

import warnings
from pydantic import validate_call, Field, StrictFloat, StrictStr, StrictInt
from typing import Any, Dict, List, Optional, Tuple, Union
from typing_extensions import Annotated

from pydantic import Field, StrictInt, StrictStr
from typing import Optional
from typing_extensions import Annotated
from openapi_client.models.sapi_v1_pay_transactions_get200_response import SapiV1PayTransactionsGet200Response

from openapi_client.api_client import ApiClient, RequestSerialized
from openapi_client.api_response import ApiResponse
from openapi_client.rest import RESTResponseType


class PayApi:
    """NOTE: This class is auto generated by OpenAPI Generator
    Ref: https://openapi-generator.tech

    Do not edit the class manually.
    """

    def __init__(self, api_client=None) -> None:
        if api_client is None:
            api_client = ApiClient.get_default()
        self.api_client = api_client


    @validate_call
    def sapi_v1_pay_transactions_get(
        self,
        timestamp: Annotated[StrictInt, Field(description="UTC timestamp in ms")],
        signature: Annotated[StrictStr, Field(description="Signature")],
        start_time: Annotated[Optional[StrictInt], Field(description="UTC timestamp in ms")] = None,
        end_time: Annotated[Optional[StrictInt], Field(description="UTC timestamp in ms")] = None,
        limit: Annotated[Optional[StrictInt], Field(description="default 100, max 100")] = None,
        recv_window: Annotated[Optional[StrictInt], Field(description="The value cannot be greater than 60000")] = None,
        _request_timeout: Union[
            None,
            Annotated[StrictFloat, Field(gt=0)],
            Tuple[
                Annotated[StrictFloat, Field(gt=0)],
                Annotated[StrictFloat, Field(gt=0)]
            ]
        ] = None,
        _request_auth: Optional[Dict[StrictStr, Any]] = None,
        _content_type: Optional[StrictStr] = None,
        _headers: Optional[Dict[StrictStr, Any]] = None,
        _host_index: Annotated[StrictInt, Field(ge=0, le=0)] = 0,
    ) -> SapiV1PayTransactionsGet200Response:
        """Get Pay Trade History (USER_DATA)

        - If startTime and endTime are not sent, the recent 90 days' data will be returned. - The max interval between startTime and endTime is 90 days. - Support for querying orders within the last 18 months.  Weight(UID): 3000

        :param timestamp: UTC timestamp in ms (required)
        :type timestamp: int
        :param signature: Signature (required)
        :type signature: str
        :param start_time: UTC timestamp in ms
        :type start_time: int
        :param end_time: UTC timestamp in ms
        :type end_time: int
        :param limit: default 100, max 100
        :type limit: int
        :param recv_window: The value cannot be greater than 60000
        :type recv_window: int
        :param _request_timeout: timeout setting for this request. If one
                                 number provided, it will be total request
                                 timeout. It can also be a pair (tuple) of
                                 (connection, read) timeouts.
        :type _request_timeout: int, tuple(int, int), optional
        :param _request_auth: set to override the auth_settings for an a single
                              request; this effectively ignores the
                              authentication in the spec for a single request.
        :type _request_auth: dict, optional
        :param _content_type: force content-type for the request.
        :type _content_type: str, Optional
        :param _headers: set to override the headers for a single
                         request; this effectively ignores the headers
                         in the spec for a single request.
        :type _headers: dict, optional
        :param _host_index: set to override the host_index for a single
                            request; this effectively ignores the host_index
                            in the spec for a single request.
        :type _host_index: int, optional
        :return: Returns the result object.
        """ # noqa: E501

        _param = self._sapi_v1_pay_transactions_get_serialize(
            timestamp=timestamp,
            signature=signature,
            start_time=start_time,
            end_time=end_time,
            limit=limit,
            recv_window=recv_window,
            _request_auth=_request_auth,
            _content_type=_content_type,
            _headers=_headers,
            _host_index=_host_index
        )

        _response_types_map: Dict[str, Optional[str]] = {
            '200': "SapiV1PayTransactionsGet200Response",
            '400': "Error",
            '401': "Error",
        }
        response_data = self.api_client.call_api(
            *_param,
            _request_timeout=_request_timeout
        )
        response_data.read()
        return self.api_client.response_deserialize(
            response_data=response_data,
            response_types_map=_response_types_map,
        ).data


    @validate_call
    def sapi_v1_pay_transactions_get_with_http_info(
        self,
        timestamp: Annotated[StrictInt, Field(description="UTC timestamp in ms")],
        signature: Annotated[StrictStr, Field(description="Signature")],
        start_time: Annotated[Optional[StrictInt], Field(description="UTC timestamp in ms")] = None,
        end_time: Annotated[Optional[StrictInt], Field(description="UTC timestamp in ms")] = None,
        limit: Annotated[Optional[StrictInt], Field(description="default 100, max 100")] = None,
        recv_window: Annotated[Optional[StrictInt], Field(description="The value cannot be greater than 60000")] = None,
        _request_timeout: Union[
            None,
            Annotated[StrictFloat, Field(gt=0)],
            Tuple[
                Annotated[StrictFloat, Field(gt=0)],
                Annotated[StrictFloat, Field(gt=0)]
            ]
        ] = None,
        _request_auth: Optional[Dict[StrictStr, Any]] = None,
        _content_type: Optional[StrictStr] = None,
        _headers: Optional[Dict[StrictStr, Any]] = None,
        _host_index: Annotated[StrictInt, Field(ge=0, le=0)] = 0,
    ) -> ApiResponse[SapiV1PayTransactionsGet200Response]:
        """Get Pay Trade History (USER_DATA)

        - If startTime and endTime are not sent, the recent 90 days' data will be returned. - The max interval between startTime and endTime is 90 days. - Support for querying orders within the last 18 months.  Weight(UID): 3000

        :param timestamp: UTC timestamp in ms (required)
        :type timestamp: int
        :param signature: Signature (required)
        :type signature: str
        :param start_time: UTC timestamp in ms
        :type start_time: int
        :param end_time: UTC timestamp in ms
        :type end_time: int
        :param limit: default 100, max 100
        :type limit: int
        :param recv_window: The value cannot be greater than 60000
        :type recv_window: int
        :param _request_timeout: timeout setting for this request. If one
                                 number provided, it will be total request
                                 timeout. It can also be a pair (tuple) of
                                 (connection, read) timeouts.
        :type _request_timeout: int, tuple(int, int), optional
        :param _request_auth: set to override the auth_settings for an a single
                              request; this effectively ignores the
                              authentication in the spec for a single request.
        :type _request_auth: dict, optional
        :param _content_type: force content-type for the request.
        :type _content_type: str, Optional
        :param _headers: set to override the headers for a single
                         request; this effectively ignores the headers
                         in the spec for a single request.
        :type _headers: dict, optional
        :param _host_index: set to override the host_index for a single
                            request; this effectively ignores the host_index
                            in the spec for a single request.
        :type _host_index: int, optional
        :return: Returns the result object.
        """ # noqa: E501

        _param = self._sapi_v1_pay_transactions_get_serialize(
            timestamp=timestamp,
            signature=signature,
            start_time=start_time,
            end_time=end_time,
            limit=limit,
            recv_window=recv_window,
            _request_auth=_request_auth,
            _content_type=_content_type,
            _headers=_headers,
            _host_index=_host_index
        )

        _response_types_map: Dict[str, Optional[str]] = {
            '200': "SapiV1PayTransactionsGet200Response",
            '400': "Error",
            '401': "Error",
        }
        response_data = self.api_client.call_api(
            *_param,
            _request_timeout=_request_timeout
        )
        response_data.read()
        return self.api_client.response_deserialize(
            response_data=response_data,
            response_types_map=_response_types_map,
        )


    @validate_call
    def sapi_v1_pay_transactions_get_without_preload_content(
        self,
        timestamp: Annotated[StrictInt, Field(description="UTC timestamp in ms")],
        signature: Annotated[StrictStr, Field(description="Signature")],
        start_time: Annotated[Optional[StrictInt], Field(description="UTC timestamp in ms")] = None,
        end_time: Annotated[Optional[StrictInt], Field(description="UTC timestamp in ms")] = None,
        limit: Annotated[Optional[StrictInt], Field(description="default 100, max 100")] = None,
        recv_window: Annotated[Optional[StrictInt], Field(description="The value cannot be greater than 60000")] = None,
        _request_timeout: Union[
            None,
            Annotated[StrictFloat, Field(gt=0)],
            Tuple[
                Annotated[StrictFloat, Field(gt=0)],
                Annotated[StrictFloat, Field(gt=0)]
            ]
        ] = None,
        _request_auth: Optional[Dict[StrictStr, Any]] = None,
        _content_type: Optional[StrictStr] = None,
        _headers: Optional[Dict[StrictStr, Any]] = None,
        _host_index: Annotated[StrictInt, Field(ge=0, le=0)] = 0,
    ) -> RESTResponseType:
        """Get Pay Trade History (USER_DATA)

        - If startTime and endTime are not sent, the recent 90 days' data will be returned. - The max interval between startTime and endTime is 90 days. - Support for querying orders within the last 18 months.  Weight(UID): 3000

        :param timestamp: UTC timestamp in ms (required)
        :type timestamp: int
        :param signature: Signature (required)
        :type signature: str
        :param start_time: UTC timestamp in ms
        :type start_time: int
        :param end_time: UTC timestamp in ms
        :type end_time: int
        :param limit: default 100, max 100
        :type limit: int
        :param recv_window: The value cannot be greater than 60000
        :type recv_window: int
        :param _request_timeout: timeout setting for this request. If one
                                 number provided, it will be total request
                                 timeout. It can also be a pair (tuple) of
                                 (connection, read) timeouts.
        :type _request_timeout: int, tuple(int, int), optional
        :param _request_auth: set to override the auth_settings for an a single
                              request; this effectively ignores the
                              authentication in the spec for a single request.
        :type _request_auth: dict, optional
        :param _content_type: force content-type for the request.
        :type _content_type: str, Optional
        :param _headers: set to override the headers for a single
                         request; this effectively ignores the headers
                         in the spec for a single request.
        :type _headers: dict, optional
        :param _host_index: set to override the host_index for a single
                            request; this effectively ignores the host_index
                            in the spec for a single request.
        :type _host_index: int, optional
        :return: Returns the result object.
        """ # noqa: E501

        _param = self._sapi_v1_pay_transactions_get_serialize(
            timestamp=timestamp,
            signature=signature,
            start_time=start_time,
            end_time=end_time,
            limit=limit,
            recv_window=recv_window,
            _request_auth=_request_auth,
            _content_type=_content_type,
            _headers=_headers,
            _host_index=_host_index
        )

        _response_types_map: Dict[str, Optional[str]] = {
            '200': "SapiV1PayTransactionsGet200Response",
            '400': "Error",
            '401': "Error",
        }
        response_data = self.api_client.call_api(
            *_param,
            _request_timeout=_request_timeout
        )
        return response_data.response


    def _sapi_v1_pay_transactions_get_serialize(
        self,
        timestamp,
        signature,
        start_time,
        end_time,
        limit,
        recv_window,
        _request_auth,
        _content_type,
        _headers,
        _host_index,
    ) -> RequestSerialized:

        _host = None

        _collection_formats: Dict[str, str] = {
        }

        _path_params: Dict[str, str] = {}
        _query_params: List[Tuple[str, str]] = []
        _header_params: Dict[str, Optional[str]] = _headers or {}
        _form_params: List[Tuple[str, str]] = []
        _files: Dict[str, Union[str, bytes]] = {}
        _body_params: Optional[bytes] = None

        # process the path parameters
        # process the query parameters
        if start_time is not None:
            
            _query_params.append(('startTime', start_time))
            
        if end_time is not None:
            
            _query_params.append(('endTime', end_time))
            
        if limit is not None:
            
            _query_params.append(('limit', limit))
            
        if recv_window is not None:
            
            _query_params.append(('recvWindow', recv_window))
            
        if timestamp is not None:
            
            _query_params.append(('timestamp', timestamp))
            
        if signature is not None:
            
            _query_params.append(('signature', signature))
            
        # process the header parameters
        # process the form parameters
        # process the body parameter


        # set the HTTP header `Accept`
        if 'Accept' not in _header_params:
            _header_params['Accept'] = self.api_client.select_header_accept(
                [
                    'application/json'
                ]
            )


        # authentication setting
        _auth_settings: List[str] = [
            'ApiKeyAuth'
        ]

        return self.api_client.param_serialize(
            method='GET',
            resource_path='/sapi/v1/pay/transactions',
            path_params=_path_params,
            query_params=_query_params,
            header_params=_header_params,
            body=_body_params,
            post_params=_form_params,
            files=_files,
            auth_settings=_auth_settings,
            collection_formats=_collection_formats,
            _host=_host,
            _request_auth=_request_auth
        )


